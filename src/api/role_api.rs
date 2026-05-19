use crate::common::result::{CommonResult, FilterParam};
use crate::service::role_service::{
    RoleDto, DelParams, RoleService, SearchRoleDto, UpdateRole,
};
use crate::{AppState, UserError};
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, put, Responder};

#[post("/page")]
pub async fn page(
    state: Data<AppState>,
    Json(page): Json<FilterParam<SearchRoleDto>>,
) -> Result<impl Responder, UserError> {
    let vec = RoleService::find_page(state, page).await?;
    Ok(CommonResult::success(vec))
}

#[post("/list")]
pub async fn list(
    state: Data<AppState>,
    Json(filter): Json<FilterParam<SearchRoleDto>>,
) -> Result<impl Responder, UserError> {
    let vec = RoleService::find_all(state,filter).await?;
    Ok(CommonResult::success(vec))
}

#[post("/create")]
pub async fn create(
    state: Data<AppState>,
    Json(create): Json<RoleDto>,
) -> Result<impl Responder, UserError> {
    let vec = RoleService::create(state, create).await?;
    Ok(CommonResult::success(vec))
}

#[get("/{id}")]
pub async fn find_one(state: Data<AppState>, id: Path<i32>) -> Result<impl Responder, UserError> {
    let data = RoleService::find_one(state, id.into_inner()).await?;
    Ok(CommonResult::success(data))
}

#[put("/update")]
pub async fn update(
    state: Data<AppState>,
    Json(update): Json<UpdateRole>,
) -> Result<impl Responder, UserError> {
    let data = RoleService::update(state, update).await?;
    Ok(CommonResult::success(data))
}

#[post("/del")]
pub async fn delete(
    state: Data<AppState>,
    Json(dels): Json<DelParams>,
) -> Result<impl Responder, UserError> {
    let data = RoleService::delete(state, dels).await?;
    Ok(CommonResult::success(data))
}
