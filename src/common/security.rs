use std::env;
use crate::UserError;
use argon2::password_hash::SaltString;
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::sqlx::Encode;
use sea_orm::ColIdx;
use serde::{Deserialize, Serialize};
use std::io::Error;
use dotenvy::dotenv;
use once_cell::sync::Lazy;


static SECRET_KEY:Lazy<String> = Lazy::new(||{
    dotenv().ok(); // 加载 .env 文件中的环境变量
    env::var("SECRET_KEY").expect("SECRET_KEY must be set")
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_name: String,
    roles: String,
    sub: String,
    exp: usize,
}

pub struct Security;

impl Security {

    fn get_secret_key() -> &'static str {
        &SECRET_KEY
    }
    pub fn verify(hash: &str,password:&str) -> bool {
        let hash = PasswordHash::new(hash);
        if hash.is_err() {
            return false;
        }
        let result = argon2::Argon2::default().verify_password(password.as_bytes(), &hash.unwrap());
        if result.is_err() {
            return false;
        }
        true
    }

    pub fn hash_password(password:&str) -> Result<String,UserError>{
        let salt = SaltString::generate(rand::thread_rng());
        let result = argon2::Argon2::default().hash_password(password.as_bytes(), &salt);
        match result {
            Ok(o) => {
                let string = format!("{}", o);
                Ok(string)
            }
            Err(_) => {
                Err(UserError::Error("hash_password error".to_string()))
            }
        }
    }

    pub fn encode_token(user_id: i32,user_name:String) -> Result<String,UserError> {
        let secret = Security::get_secret_key();

        let now = Utc::now();
        let exp = now + Duration::seconds(60 * 5); // 过期时间设为5分钟后
        let claims = Claims {
            user_name:user_name.clone(),
            roles: "".to_string(),
            sub:user_id.to_string(),
            exp: exp.timestamp() as usize, // 将过期时间转换为时间戳
        };

        // 编码生成JWT
        let result = encode(&Header::default(),
                            &claims,
                            &EncodingKey::from_secret(secret.as_ref()));
        match result {
            Ok(re) => Ok(re),
            Err(e) => Err(UserError::Error(e.to_string()))
        }
    }

    pub fn decode_token(token: &str)-> Result<Claims,UserError> {
        let secret = Security::get_secret_key();

        let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());
        match token_data {
            Ok(d) => Ok(d.claims),
            Err(e) => Err(UserError::Error(e.to_string()))
        }
    }

}

#[cfg(test)]
mod tests{
    use crate::common::security::Claims;
    use argon2::{PasswordHash, PasswordVerifier};
    use jsonwebtoken::{decode, DecodingKey, Validation};

    #[test]
    fn test_a() {
        let ps = "$argon2id$v=19$m=65536,t=3,p=4$UnLTM2dIFV09MWWe7KFOkA$UPjQG54C6M60IGN08BPb5UMnDK/fNZPuJ03GpJOClBA";
        let success = "123456";
        let error = "123456567";

        let hash = PasswordHash::new(ps).unwrap();
        let result = argon2::Argon2::default().verify_password(success.as_bytes(), &hash);
        assert_eq!(true,result.is_ok());
        let result = argon2::Argon2::default().verify_password(error.as_bytes(), &hash);
        assert_eq!(false, result.is_ok());
    }

    #[test]
    fn test_jwt() {
        // 定义密钥
        let secret = "your_secret_key";
        //
        // // 创建声明（Claims）
        // let now = Utc::now();
        // let exp = now + Duration::seconds(60 * 5); // 过期时间设为5分钟后
        // let claims = Claims {
        //     sub: "user123".to_string(),
        //     exp: exp.timestamp() as usize, // 将过期时间转换为时间戳
        // };
        //
        // // 编码生成JWT
        // let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();
        //
        // println!("生成的JWT: {}", token);

        // 验证JWT
        let token_data = decode::<Claims>("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoxNzM2NzU0MjE4fQ.TU3EiOja6EeWXX3PU3BvWWAqKfsYf0bXcmq5eTeYeGA", &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).unwrap();

        println!("解码的声明: {:?}", token_data.claims);
    }
}