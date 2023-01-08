use std::ops::Not;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;

use async_trait::async_trait;
use e521_curve::e521::Point;
use e521_curve::{generate_private_key, generate_public_key, generate_salt};
use enigma::{decrypt_data, encrypt_password, verify_password};
use num_bigint_dig::BigInt;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

use monie_rpc::monie::auth::authentication_api_server::AuthenticationApi;
use monie_rpc::monie::auth::{
    CodeResponse, CreateAnonymousAccountRequest, EnterCodeRequest, EnterPhoneNumberRequest,
    GenerateAnonymousAccountRequest, GenerateAnonymousResponse, LoginAnonymousRequest,
    PublicKeyRequest, PublicKeyResponse,
};
use monie_rpc::monie::user::UserResponse;

use crate::api::models::user::User;
use crate::data::db::{
    create_session, create_user, generate_new_username, get_user_by_id, get_user_session,
    get_username_id, is_session_id_exist, is_user_exist_for_uuid, is_username_exist,
    update_user_info,
};
use crate::data::passwords::{generate_password, PASSWORD_PEPPER};

#[derive(Debug)]
pub struct AuthenticationService {}

#[async_trait]
impl AuthenticationApi for AuthenticationService {
    async fn generate_private_key(
        &self,
        request: Request<PublicKeyRequest>,
    ) -> Result<Response<PublicKeyResponse>, Status> {
        let client_public_key = request.into_inner();
        let (private_key, public_key) = create_public_key();

        let secret_key = create_secret_key(
            &private_key,
            &Point {
                x: BigInt::from_str(&client_public_key.x[..]).unwrap(),
                y: BigInt::from_str(&client_public_key.y[..]).unwrap(),
            },
        );

        let session = create_session(secret_key);

        let public_key = PublicKeyResponse {
            id: session.0.to_string(),
            x: public_key.x.to_string(),
            y: public_key.y.to_string(),
        };

        Ok(Response::new(public_key))
    }

    async fn generate_anonymous_account(
        &self,
        request: Request<GenerateAnonymousAccountRequest>,
    ) -> Result<Response<GenerateAnonymousResponse>, Status> {
        let request = request.into_inner();
        let uuid = Uuid::from_str(request.id.as_str()).unwrap();

        if is_session_id_exist(&uuid).not() {
            return Err(Status::not_found(
                "UUID not found, you need to create new one",
            ));
        }

        if is_user_exist_for_uuid(&uuid) {
            return Err(Status::already_exists("UUID found, but user already exist"));
        }

        let password = Arc::new(generate_password().as_bytes().to_vec());
        let username = Arc::new(generate_new_username());

        self.create_user(uuid.clone(), password.clone(), username.clone());

        Ok(Response::new(GenerateAnonymousResponse {
            id: uuid.to_string(),
            username: username.to_string(),
            password: password.to_vec(),
        }))
    }

    async fn create_anonymous_account(
        &self,
        request: Request<CreateAnonymousAccountRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let request = request.into_inner();
        let uuid = Uuid::from_str(request.id.as_str()).unwrap();

        if is_session_id_exist(&uuid).not() {
            return Err(Status::not_found(
                "UUID not found, you need to create new one",
            ));
        }
        let username = request.username;

        if is_username_exist(&username).not() {
            return Err(Status::not_found(
                "Username not found, you need to create new one",
            ));
        }

        let user_uuid = get_username_id(&username).unwrap();
        let user_session = get_user_session(&user_uuid).unwrap();

        let password = request.password;

        return if verify_password(
            user_session.hashed_password.as_slice(),
            password.as_slice(),
            user_session.salt.as_slice(),
            PASSWORD_PEPPER,
        ) {
            let name = request.name;
            let about = request.about;
            update_user_info(&user_uuid, name, about);

            let user: User = get_user_by_id(&user_uuid).user.into();
            let user_response: UserResponse = user.into();

            Ok(Response::new(user_response))
        } else {
            Err(Status::permission_denied(
                "Username or password is wrong, you need to create new one",
            ))
        };
    }

    async fn send_code_to_phone_number(
        &self,
        request: Request<EnterPhoneNumberRequest>,
    ) -> Result<Response<CodeResponse>, Status> {
        todo!()
    }

    async fn login_with_phone_number(
        &self,
        request: Request<EnterCodeRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let request = request.into_inner();
        let phone = &request.phone;
        let code = &request.code;

        if code == "123456" {
            Ok(Response::new(get_user(String::from(phone)).into()))
        } else {
            Err(Status::new(Code::InvalidArgument, "Code error"))
        }
    }

    async fn login_anonymously(
        &self,
        request: Request<LoginAnonymousRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let request = request.into_inner();

        let username_encrypted = &request.username;
        let password_encrypted = &request.password;

        let user_uuid = get_username_id(username_encrypted);

        match user_uuid {
            None => Err(Status::not_found("User not found")),
            Some(uuid) => self.check_user_password(&uuid, password_encrypted),
        }
    }
}

impl AuthenticationService {
    fn check_user_password(
        &self,
        uuid: &Uuid,
        password_encrypted: &Vec<u8>,
    ) -> Result<Response<UserResponse>, Status> {
        let user_session = get_user_session(uuid);
        match user_session {
            None => Err(Status::not_found("User not found")),
            Some(session) => {
                let password = decrypt_data(
                    password_encrypted.as_slice(),
                    session.private_key.as_slice(),
                    b"usman akhmedov",
                );

                if verify_password(
                    session.hashed_password.as_slice(),
                    password.as_slice(),
                    session.salt.as_slice(),
                    PASSWORD_PEPPER,
                ) {
                    let user: User = session.user.into();
                    Ok(Response::new(user.into()))
                } else {
                    Err(Status::not_found("User not found"))
                }
            }
        }
    }

    fn create_user(&self, uuid: Uuid, password: Arc<Vec<u8>>, username: Arc<String>) {
        thread::Builder::new()
            .spawn(move || {
                let salt = generate_salt();
                let salt = salt.as_slice();

                let password = Arc::clone(&password).to_vec();
                let password_encrypted =
                    encrypt_password(password.as_slice(), salt, PASSWORD_PEPPER);
                let username = Arc::clone(&username).to_string();

                dbg!("CREATE_USER");
                create_user(
                    &uuid,
                    username.clone(),
                    Some(username),
                    password_encrypted,
                    &salt.to_vec(),
                    &vec![],
                );
            })
            .unwrap();
    }
}

fn create_secret_key(private_key: &BigInt, public_key: &Point) -> Vec<u8> {
    let point = e521_curve::diffie_hellman(private_key, public_key);
    e521_curve::generate_secret_key(point)
}

pub fn create_public_key() -> (BigInt, Point) {
    let private_key: BigInt = generate_private_key();
    let public_key_point: Point = generate_public_key(&private_key);
    (private_key, public_key_point)
}

fn get_user(phone: String) -> User {
    User {
        id: Uuid::new_v4().to_string(),
        name: "nie".to_string(),
        avatar: None,
        about: Some(String::from("YO CEO mNie")),
        username: Some(String::from("nie")),
        phone: Some(phone),
        email: Some(String::from("nie@usmonie.com")),
    }
}
