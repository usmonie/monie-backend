use std::str::FromStr;
use tonic::{Request, Response, Status};
use async_trait::async_trait;
use rug::{Integer};
use cryptography::e521::{AddPoint, PointE521};
use cryptography::{generate_private_key, generate_public_key};
use monie_rpc::monie::authentication_server::Authentication;
use monie_rpc::monie::{Anonymous, AnonymousResult, CodeResult, PhoneNumber, PublicKey};

pub struct AuthenticationService {}

#[async_trait]
impl Authentication for AuthenticationService {
    async fn generate_private_key(&self, request: Request<PublicKey>) -> Result<Response<PublicKey>, Status> {
        let client_public_key = request.get_ref();
        let (private_key, public_key) = create_public_key();

        let secret_key = create_secret_key(&private_key, &PointE521 {
            x: Integer::from_str(&client_public_key.x[..]).unwrap(),
            y: Integer::from_str(&client_public_key.y[..]).unwrap(),
        });

        Ok(Response::new(
            PublicKey {
                x: public_key.x.to_string(),
                y: public_key.y.to_string(),
            }
        ))
    }

    async fn login_with_phone_number(&self, request: Request<PhoneNumber>) -> Result<Response<CodeResult>, Status> {
        todo!()
    }

    async fn login_anonymously(&self, request: Request<Anonymous>) -> Result<Response<AnonymousResult>, Status> {
        todo!()
    }
}

fn create_secret_key(private_key: &Integer, public_key: &PointE521) -> PointE521 {
    cryptography::diffie_hellman(private_key, public_key)
}

pub fn create_public_key() -> (Integer, PointE521) {
    let private_key: Integer = generate_private_key();
    let public_key_point: PointE521 = generate_public_key(&private_key);
    (private_key, public_key_point)
}
