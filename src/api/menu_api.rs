use actix_web::{get, post, put, Responder};
use actix_web::web::{Data, Json, Query,Path};
use log::info;
use crate::{AppState, UserError};
use crate::common::result::{CommonResult, FilterParam};
use crate::entity::prelude::Menu;
use crate::service::menu_service::{CreateMenu, DelParams, MenuService, SearchParams, UpdateMenu};

#[post("/create")]
pub async fn create(state: Data<AppState>, Json(create_params) : Json<CreateMenu>) -> Result<impl Responder,UserError> {
    info!("{:?}", create_params);
    let create = MenuService::create(state, create_params).await?;
    Ok(CommonResult::success(create))
}

#[post("/list")]
pub async fn list(state: Data<AppState>, page :Json<FilterParam<SearchParams>>) ->Result<impl Responder,UserError> {
    info!("{:?}", page);
    let all = MenuService::find_all(state, page).await?;
    Ok(CommonResult::success(all))
}

#[get("/{id}")]
pub async fn find_one(state: Data<AppState>, id :Path<i32>) ->Result<impl Responder,UserError> {
    info!("{:?}", id);
    let one = MenuService::find_one(state,id).await?;
    Ok(CommonResult::success(one))
}

#[put("/update")]
pub async fn update(state: Data<AppState>, data :Json<UpdateMenu>) ->Result<impl Responder,UserError> {
    info!("{:?}", data);
    let update = MenuService::update(state, data).await?;
    Ok(CommonResult::success(update))
}

#[post("/del")]
pub async fn delete(state: Data<AppState>, Json(del):Json<DelParams>) ->Result<impl Responder,UserError> {
    info!("{:?}", del);
    let i = MenuService::delete(state, del).await?;
    Ok(CommonResult::success(i))
}