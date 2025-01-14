use actix_web::cookie::time::format_description::parse;
use actix_web::{get, post, Responder};
use actix_web::web::{Data, Json, Path};
use crate::service::permission_service::{PermissionAssignRoleMenuReqDto, PermissionService};
use crate::{AppState, UserError};
use crate::common::result::CommonResult;

#[get("/list-role-resources/{role_id}")]
pub async fn get_menus_permission_by_role_id(state:Data<AppState>,path:Path<i32>)->Result<impl Responder,UserError> {
    let role_id = path.into_inner();
    let permissions = PermissionService::get_menus_permission_by_role_id(state,role_id).await?;
    Ok(CommonResult::success(permissions))
}

#[post("/assign-role-menu")]
pub async fn assign_role_perm_code(state:Data<AppState>,Json(dto):Json<PermissionAssignRoleMenuReqDto>)->Result<impl Responder,UserError> {
    let permissions = PermissionService::assign_role_perm_code(state,dto).await?;
    Ok(CommonResult::success(permissions))
}