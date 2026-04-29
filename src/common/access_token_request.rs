use serde::Deserialize;

pub struct AccessTokenRequest;

impl AccessTokenRequest {
    pub async fn request_token(token_endpoint:&str,
                               username:String,
                               password:String,
                               client_id:&str,
                               client_secret:&str,
                                )->Result<TokenResponse, reqwest::Error> {
        let client = reqwest::Client::new();

        let response = client
            .post(token_endpoint)
            .basic_auth(client_id, Some(client_secret)) // 客户端 ID 和密码
            .form(&[
                ("username", username),
                ("password", password),
                ("grant_type", "authorization_password".to_string()),
            ])
            .send()
            .await?
            .json::<TokenResponse>().await?;

        log::info!("Response Status: {:?}", response);
        Ok(response)
    }

}

#[derive(Debug,Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    refresh_token: String,
    scope: String,
    id_token: String,
    token_type: String,
    expires_in: u32,
}

#[cfg(test)]
mod tests {
    use crate::common::access_token_request::TokenResponse;

    #[tokio::test]
    pub async fn test_sign_in()->Result<(), reqwest::Error> {
        let client = reqwest::Client::new();

        let response = client
            .post("http://127.0.0.1:3000/oauth2/token")
            .basic_auth("client-msg", Some("123456")) // 客户端 ID 和密码
            .form(&[
                ("username", "admin"),
                ("password", "123456"),
                ("grant_type", "authorization_password"),
            ])
            .send()
            .await?
            .json::<TokenResponse>().await?;

        println!("Response Status: {:?}", response);
        Ok(())
    }

}