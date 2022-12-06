use std::str::FromStr;
use std::time::{Duration, Instant};
use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;
use bytes::Bytes;
use rand::Rng;
use rand::rngs::OsRng;
use rug::Integer;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use cryptography::e521::{PointE521};
use cryptography::{encrypt_data, generate_private_key, generate_public_key, verify};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
const PASSWORD_PEPPER: &str = "SPGG2fj59ZCnKhmeXJ5g77F5";

pub struct Client {
    id: u128,
    user_id: Option<u128>,
    hb: Instant,
    private_key: Option<PointE521>,
    previous_private_key: Option<PointE521>,
}

impl Client {
    pub fn new() -> Self {
        let mut rng = OsRng;
        Self {
            id: rng.gen(),
            user_id: None,
            hb: Instant::now(),
            private_key: None,
            previous_private_key: None,
        }
    }

    pub fn new_with_id(id: u128, user_id: u128) -> Self {
        Self {
            id,
            user_id: Some(user_id),
            hb: Instant::now(),
            private_key: None,
            previous_private_key: None,
        }
    }

    pub fn create_public_key(&mut self, ctx: &mut <Client as Actor>::Context) -> (Integer, PointE521) {
        let private_key = generate_private_key();
        let public_key = generate_public_key(&private_key);
        let response = format!("{{ x: {}, y: {} }}", &public_key.x, &public_key.y);
        ctx.text(response);
        (private_key, public_key)
    }

    // This function will run on an interval, every 5 seconds to check
    // that the connection is still alive. If it's been more than
    // 10 seconds since the last ping, we'll close the connection.
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }

    fn create_secret_key(&mut self, ctx: &mut WebsocketContext<Client>, x: String, y: String) {
        self.hb = Instant::now();
        let (private_key, _public_key) = self.create_public_key(ctx);

        let client_public_key = PointE521 {
            x: Integer::from_str(&x[..]).unwrap(),
            y: Integer::from_str(&y[..]).unwrap(),
        };
        ctx.binary(format!("{}", client_public_key));
        let server_private_key = cryptography::diffie_hellman(&private_key, &client_public_key);
        self.private_key = Some(server_private_key);
    }

    fn login_with_phone_number(&mut self, ctx: &mut WebsocketContext<Client>, number: String) {}

    fn login_anonymous(&mut self, ctx: &mut WebsocketContext<Client>, username: String, password: String) {
        let mut hashed = encrypt_data(password.as_bytes(), PASSWORD_PEPPER.as_bytes());
        ctx.binary(format!("{}", verify(&mut hashed, password.as_bytes(), PASSWORD_PEPPER.as_bytes())));
    }

    fn handle_binary_message(&mut self, ctx: &mut WebsocketContext<Client>, message: Vec<u8>) {
        // let message = &String::from_utf8(message).unwrap()[..];
        let message: Message = serde_json::from_slice(message.as_slice()).unwrap();
        match message {
            Message::PhoneNumber { number } => self.login_with_phone_number(ctx, number),
            Message::AnonymousEnter { username, password } => self.login_anonymous(ctx, username, password)
        }
    }

    fn handle_binary_socket(&mut self, ctx: &mut WebsocketContext<Client>, bytes: &Bytes) {
        let req = base64::decode(bytes).unwrap();
        let socket_binary: SocketBinary = serde_json::from_slice(&req).unwrap();

        match socket_binary {
            SocketBinary::PublicKey { x, y } => self.create_secret_key(ctx, x, y),
            SocketBinary::Encrypted { message } => self.handle_binary_message(ctx, message)
        }
    }
}

impl Actor for Client {
    type Context = WebsocketContext<Self>;

    // Start the heartbeat process for this connection
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

// The `StreamHandler` trait is used to handle the messages that are sent over the socket.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Client {
    // The `handle()` function is where we'll determine the response
    // to the client's messages. So, for example, if we ping the client,
    // it should respond with a pong. These two messages are necessary
    // for the `hb()` function to maintain the connection status.
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // Ping/Pong will be used to make sure the connection is still alive
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => self.hb = Instant::now(),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bytes)) => self.handle_binary_socket(ctx, &bytes),
            // Close will close the socket
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum SocketBinary {
    Encrypted { message: Vec<u8> },

    PublicKey { x: String, y: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag="message")]
enum Message {
    PhoneNumber { number: String },

    AnonymousEnter { username: String, password: String },
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientPublicKey {
    x: String,
    y: String,
}
