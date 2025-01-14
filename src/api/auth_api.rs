use actix_web::{http, post, HttpRequest, Responder};
use actix_web::cookie::time::macros::date;
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};
use crate::{AppState, UserError};
use crate::common::result::CommonResult;
use crate::service::auth::Auth;
use crate::service::menu_service::MenuService;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserNamePassword {
    user_name: String,
    password: String,
}
#[post("/signin")]
pub async fn sign_in(state:Data<AppState>, Json(data):Json<UserNamePassword>) ->Result<impl Responder,UserError> {
    let token = Auth::sign_in(state, data.user_name, data.password).await?;
    Ok(CommonResult::success(token))
}

#[post("/signout")]
pub async fn sign_out(req:HttpRequest) ->Result<impl Responder,UserError> {
    let option = req.headers().get(http::header::AUTHORIZATION);
    if option.is_none() {
        return Err(UserError::Error("error".to_string()));
    }
    let result = option.unwrap().to_str();
    if result.is_err() {
        return Err(UserError::Error("error".to_string()));
    }
    let token = result.unwrap().to_string();
    Auth::sign_out(token).await?;
    Ok(CommonResult::<String>::success_none())
}

#[post("/menu")]
pub async fn get_menu_by_user_auth_code(state:Data<AppState>, Json(data):Json<Vec<String>>) ->Result<impl Responder,UserError> {
    let vec = MenuService::get_menu_by_user_auth_code(state, data)
        .await?;
    Ok(CommonResult::success(vec))
}