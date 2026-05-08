use crate::common::result::CommonResult;
use crate::service::department_service::{CreateDepartment, DelParams, DepartmentService, SearchParams};
use crate::{AppState, UserError};
use actix_web::web::{Data, Json};
use actix_web::{post, Responder};

#[post("/list")]
pub async fn list(state:Data<AppState>, list:Json<SearchParams>) ->Result<impl Responder,UserError> {
    let result = DepartmentService::find_all(state, list).await?;
    Ok(CommonResult::success(result))
}

#[post("/create")]
pub async fn create(state:Data<AppState>, create:Json<CreateDepartment>) ->Result<impl Responder,UserError> {
    let result = DepartmentService::create(state, create).await?;
    Ok(CommonResult::success(result))
}

#[post("/del/")]
pub async fn delete(state:Data<AppState>, Json(dels):Json<DelParams>) ->Result<impl Responder,UserError> {
    let result = DepartmentService::delete(state, dels).await?;
    Ok(CommonResult::success(result))
}