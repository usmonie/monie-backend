use std::str::FromStr;

use async_trait::async_trait;
use rug::Integer;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

use e521_curve::cryptography::{decrypt_data, encrypt_password, verify_password};
use e521_curve::e521::PointE521;
use e521_curve::{cryptography, generate_private_key, generate_public_key, generate_salt};
use monie_rpc::monie::auth::{Anonymous, AnonymousCreate, CodeResult, PhoneNumber, PhoneNumberWithCode, PublicKeyRequest, PublicKeyResponse};
use monie_rpc::monie::auth::authentication_server::Authentication;
use monie_rpc::monie::media::Graphic;
use monie_rpc::monie::user::User;

use crate::data::db::{create_session, get_user_session, get_username_id, is_username_exist, PASSWORD_PEPPER, store_user};
use crate::domain::models::media::GraphicMediaCore;
use crate::domain::models::user::UserCore;

#[derive(Debug)]
pub struct AuthenticationService {}

#[async_trait]
impl Authentication for AuthenticationService {

    async fn generate_private_key(&self, request: Request<PublicKeyRequest>) -> Result<Response<PublicKeyResponse>, Status> {
        let client_public_key = request.into_inner();
        let (private_key, public_key) = create_public_key();

        let secret_key = create_secret_key(&private_key, &PointE521 {
            x: Integer::from_str(&client_public_key.x[..]).unwrap(),
            y: Integer::from_str(&client_public_key.y[..]).unwrap(),
        });

        let session = create_session(secret_key);

        let public_key = PublicKeyResponse {
            id: session.0.to_string(),
            x: public_key.x.to_string(),
            y: public_key.y.to_string(),
        };

        Ok(Response::new(public_key))
    }

    async fn send_code_to_phone_number(&self, request: Request<PhoneNumber>) -> Result<Response<CodeResult>, Status> {
        todo!()
    }

    async fn login_with_phone_number(&self, request: Request<PhoneNumberWithCode>) -> Result<Response<User>, Status> {
        let request = request.into_inner();
        let phone = &request.phone;
        let code = &request.code;

        if code == "123456" {
            Ok(Response::new(get_user(String::from(phone))))
        } else {
            Err(Status::new(Code::InvalidArgument, "Code error"))
        }
    }

    async fn login_anonymously(&self, request: Request<Anonymous>) -> Result<Response<User>, Status> {
        let request = request.into_inner();

        let username_encrypted = &request.username;
        let password_encrypted = &request.password;

        let user_uuid = get_username_id(username_encrypted);
        match user_uuid {
            None => Err(Status::not_found("User not found")),
            Some(uuid) => self.check_user_password(&uuid, password_encrypted)
        }
    }

    async fn create_anonymous_account(&self, request: Request<AnonymousCreate>) -> Result<Response<User>, Status> {
        let request = request.into_inner();
        if is_username_exist(&request.username) {
            return Err(Status::already_exists("Username already exist"));
        }

        let salt = generate_salt();
        let salt = salt.as_slice();

        let password_encrypted = encrypt_password(
            request.password.as_slice(),
            salt,
            PASSWORD_PEPPER.as_bytes()
        );

        let user = store_user(
            &Uuid::parse_str(request.id.as_str()).unwrap(),
            request.name.clone(),
            None,
            None,
            Some(request.username.clone()),
            None,
            None,
            &vec![],
            password_encrypted,
            &salt.to_vec(),
        );

        match user {
            None => Err(Status::aborted("Something went wrong")),
            Some(user) => Ok(Response::new(map_user(&user.user)))
        }
    }
}

impl AuthenticationService {

    fn check_user_password(&self, uuid: &Uuid, password_encrypted: &Vec<u8>) -> Result<Response<User>, Status> {
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
                ) {
                    Ok(Response::new(map_user(&session.user)))
                } else {
                    Err(Status::not_found("User not found"))
                }
            }
        }
    }
}

fn create_secret_key(private_key: &Integer, public_key: &PointE521) -> Vec<u8> {
    let point = e521_curve::diffie_hellman(private_key, public_key);
    e521_curve::generate_secret_key(point)
}

pub fn create_public_key() -> (Integer, PointE521) {
    let private_key: Integer = generate_private_key();
    let public_key_point: PointE521 = generate_public_key(&private_key);
    (private_key, public_key_point)
}

fn get_user(phone: String) -> User {
    User {
        id: Uuid::new_v4().to_string(),
        name: "nie".to_string(),
        avatar: None,
        status: Some(String::from("YO CEO mnie")),
        username: Some(String::from("nie")),
        phone: Some(phone),
        email: Some(String::from("nie@usmonie.com")),
    }
}

fn map_user(user: &UserCore) -> User {
    let avatar = user.avatar.as_ref()
        .map(|graphic| match graphic {
            GraphicMediaCore::Image(image) => {
                Graphic {
                    id: image.id.to_string(),
                    name: image.name.clone(),
                    size: image.size,
                    created_at: image.created_at.clone().to_string(),
                    upload_at: image.uploaded_at.clone().to_string(),
                    height: image.height,
                    width: image.width,
                    repeatable: false,
                    duration: 0,
                }
            }
            GraphicMediaCore::Video(video) => {
                Graphic {
                    id: video.id.to_string(),
                    name: video.name.clone(),
                    size: video.size,
                    created_at: video.created_at.clone().to_string(),
                    upload_at: video.uploaded_at.clone().to_string(),
                    height: video.height,
                    width: video.width,
                    repeatable: video.repeatable,
                    duration: video.duration,
                }
            }
        });

    User {
        id: user.id.to_string(),
        name: user.name.clone(),
        avatar,
        status: user.status.clone(),
        username: user.username.clone(),
        phone: user.phone.clone(),
        email: user.email.clone(),
    }
}