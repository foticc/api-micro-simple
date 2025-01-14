use std::io::Error;
use crate::common::result::{CommonResult, FilterParam, PageResult};
use crate::entity::menu::Model;
use crate::entity::menu::Column;
use crate::entity::menu::ActiveModel;
use crate::entity::prelude::Menu;
use crate::{AppState, UserError};
use actix_web::web::{Data, Json, Path, Query};
use log::info;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DbErr, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTime;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct MenuService{}

#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateMenu {
    pub father_id: i32,
    pub menu_name: String,
    pub menu_type: String,
    pub al_icon: Option<String>,
    pub icon: Option<String>,
    pub path: Option<String>,
    pub code: String,
    pub order_num: i32,
    pub status: Option<bool>,
    pub new_link_flag: Option<bool>,
    pub visible: Option<bool>,
}

#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMenu {
    pub id:i64,
    #[serde(flatten)]
    pub create_menu: CreateMenu,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct DelParams {
    pub ids:Vec<i32>
}

#[derive(Deserialize,Serialize,Debug)]
pub struct SearchParams {
    #[serde(rename(deserialize = "menuName",serialize = "menuName"))]
    pub menu_name:Option<String>,
    pub visible:Option<bool>,
}

impl MenuService {

    pub async fn find_all(state: Data<AppState>, Json(params) :Json<FilterParam<SearchParams>>) -> Result<PageResult<Model>, UserError> {
        let mut condition = Condition::all();
        if let Some(filter) = params.filters {
            if let Some(menu_name) = filter.menu_name {
                condition = condition.add(Column::MenuName.contains(menu_name));
            }
            if let Some(visible) = filter.visible {
                condition = condition.add(Column::Visible.eq(visible));
            }
        }
        let list = Menu::find()
            .filter(condition)
            .all(&state.conn)
            .await?;
        Ok(PageResult::new(0, 0, list.clone(), list.len() as u64))

    }

    pub async fn create(state: Data<AppState>, create_params : CreateMenu) ->Result<Model, UserError> {
        let model = ActiveModel {
            id: NotSet,
            father_id: Set(create_params.father_id),
            menu_name: Set(create_params.menu_name),
            menu_type: Set(create_params.menu_type),
            al_icon: Set(create_params.al_icon),
            icon: Set(create_params.icon),
            path: Set(create_params.path),
            code: Set(create_params.code),
            order_num: Set(create_params.order_num),
            status: Set(create_params.status),
            new_link_flag: Set(create_params.new_link_flag),
            visible: Set(create_params.visible),
            updated_at: Set(Some(Local::now().naive_local())),
            created_at: Set(Local::now().naive_local()),
            deleted_at: NotSet
        };
        let x = model.insert(&state.conn).await?;
        Ok(x)
    }

    pub async fn find_one(state: Data<AppState>,id :Path<i32>)->Result<Model, DbErr> {
        let key = id.into_inner();
        let x = Menu::find_by_id(key).one(&state.conn).await?;
        if let Some(s) = x {
            Ok(s)
        }else {
            Err(DbErr::RecordNotFound(key.to_string()))
        }
    }

    pub async fn update(state:Data<AppState>,Json(update_params) : Json<UpdateMenu>)->Result<Model,UserError> {
        let value = serde_json::to_value(&update_params)?;
        let mut result = ActiveModel::from_json(value)?;
        result.created_at = NotSet;
        let model = result.update(&state.conn).await?;
        Ok(model)
    }

    pub async fn delete(state:Data<AppState>,del_params :DelParams)->Result<u64,UserError> {
        let result = Menu::delete_many()
            .filter(Column::Id.is_in(del_params.ids))
            .exec(&state.conn)
            .await?;

        Ok(result.rows_affected)
    }

    pub async fn get_menu_by_user_auth_code(state:Data<AppState>, auth_code:Vec<String>) ->Result<Vec<Model>,UserError> {
        let vec = Menu::find()
            .filter(Column::Code.is_in(auth_code))
            .all(&state.conn)
            .await?;
        Ok(vec)
    }

}