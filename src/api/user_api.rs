use actix_web::{get, post, put, Responder};
use actix_web::web::{Data, Json, Path};
use crate::{AppState, UserError};
use crate::common::result::{CommonResult, FilterParam};
use crate::service::user_service::{ChangePassword, CreateUser, SearchParams, UpdateUser, UserService};

#[get("/auth-code/{id}")]
pub async fn find_one_auth_code(state:Data<AppState>,path:Path<i32>)-> Result<impl Responder,UserError>{
    let vec = UserService::find_one_auth_code(state, path.into_inner()).await?;
    Ok(CommonResult::success(vec))
}

#[post("/list")]
pub async fn list(state:Data<AppState>,Json(page): Json<FilterParam<SearchParams>>)-> Result<impl Responder,UserError>{
    let vec = UserService::find_all(state, page).await?;
    Ok(CommonResult::success(vec))
}

#[get("/{id}")]
pub async fn find_one(state:Data<AppState>,id: Path<i32>)-> Result<impl Responder,UserError>{
    let vec = UserService::find_one(state, id.into_inner()).await?;
    Ok(CommonResult::success(vec))
}

#[post("/create")]
pub async fn create(state:Data<AppState>,Json(user):Json<CreateUser>)->Result<impl Responder,UserError> {
    let r = UserService::create_user(state, user).await?;
    Ok(CommonResult::success(r))
}

#[put("/update")]
pub async fn update(state:Data<AppState>,Json(user):Json<UpdateUser>)->Result<impl Responder,UserError> {
    let r = UserService::update(state, user).await?;
    Ok(CommonResult::success(r))
}

#[put("/psd")]
pub async fn modify_psd(state:Data<AppState>,Json(pwd):Json<ChangePassword>)->Result<impl Responder,UserError> {
    let _ = UserService::change_pwd(state, pwd).await?;
    Ok(CommonResult::<String>::success_none())
}