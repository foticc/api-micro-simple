use std::io::Error;
use crate::common::security::{Claims, Security};
use crate::service::user_service::UserService;
use crate::{AppState, UserError};
use actix_web::web::Data;
use jsonwebtoken::{EncodingKey, Header};
use log::info;
use serde::{Deserialize, Serialize};
use crate::common::simple_cache::Cache;

pub struct Auth;
impl Auth {
    pub async fn sign_in(state:Data<AppState>, username:String, password:String) -> Result<String, UserError> {
        if Cache::get_cache(username.clone()).is_some() {
            let token = Cache::get_cache(username).unwrap();
            return Ok(token);
        }
        info!("Username: {}", username);
        let user = UserService::find_one_by_user_name(state, username.clone()).await?;
        let verify = Security::verify(
            user.password.as_str(),
            password.as_str()
        );
        if !verify {
            return Err(UserError::Error("password not match".to_string()));
        }

        match Security::encode_token(user.id,username.clone()) {
            Ok(token) => {
                Cache::set_cache(username.clone(),token.clone());
                Ok(token)
            },
            Err(_) => Err(UserError::Error("error encoding token".to_string()))
        }

    }

    pub async fn sign_out(token:String) -> Result<String, UserError> {
        let t:Vec<&str> = token.split_whitespace().collect();
        let real = if let Some(s) = t.get(1) {
            s.to_string()
        }else {
            return Err(UserError::Error("token is fail".to_string()))
        };
        if real.is_empty() {
            return Err(UserError::Error("token is empty".to_string()));
        }
        let claims = Security::decode_token(real.as_str())?;
        if let Some(user_name) =  Cache::remove_cache(claims.user_name) {
            Ok(user_name)
        }else {
            Err(UserError::Error("error".to_string()))
        }
    }

}
