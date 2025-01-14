use actix_web::web::{Data, Json, Path};
use log::info;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DbErr, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTime;
use sea_orm::sqlx::types::chrono::{Local, Utc};
use serde::{Deserialize, Serialize};
use crate::{AppState, UserError};
use crate::common::result::{FilterParam, PageResult};
use crate::entity::department::{ActiveModel, Column, Model};
use crate::entity::prelude::{Department};

pub struct DepartmentService{}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDepartment {
    pub father_id: Option<i32>,
    pub department_name: Option<String>,
    pub order_num: Option<i32>,
    pub state: Option<bool>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDepartment {
    pub id: i32,
    #[serde(flatten)]
    pub create_department: CreateDepartment,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct DelParams {
    pub ids:Vec<i32>
}

#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchParams {
    pub department_name:Option<String>,
}

impl DepartmentService {
    pub async fn create(state:Data<AppState>, Json(create_params):Json<CreateDepartment>) ->Result<Model,UserError> {
        let active_model = ActiveModel {
            id: NotSet,
            father_id: Set(create_params.father_id),
            department_name: Set(create_params.department_name.to_owned()),
            order_num: Set(create_params.order_num),
            state: Set(Some(create_params.state.unwrap_or(false))),
            updated_at: NotSet,
            created_at: Set(Local::now().naive_local()),
            deleted_at: NotSet,
        };
        let result = active_model.insert(&state.conn).await?;
        Ok(result)
    }

    pub async fn delete(state:Data<AppState>, del_params:DelParams) ->Result<u64,UserError> {
        let x = Department::delete_many()
            .filter(Column::Id.is_in(del_params.ids))
            .exec(&state.conn)
            .await?;
        Ok(x.rows_affected)
    }

    pub async fn update(state:Data<AppState>, Json(update_params):Json<UpdateDepartment>) ->Result<Model,UserError> {
        let value = serde_json::to_value(&update_params)?;
        let mut result = ActiveModel::from_json(value)?;
        result.created_at = NotSet;
        let model = result.update(&state.conn).await?;
        Ok(model)
    }

    pub async fn find_one(state: Data<AppState>, id :Path<i32>) ->Result<Model,UserError> {
        let key = id.into_inner();
        let option = Department::find_by_id(key).one(&state.conn).await?;
        if let Some(s) = option {
            Ok(s)
        }else {
            Err(UserError::from(DbErr::RecordNotFound(key.to_string())))
        }
    }

    pub async fn find_all(state:Data<AppState>, Json(list):Json<SearchParams>) -> Result<PageResult<Model>, DbErr> {
        let mut condition = Condition::all();
        if let Some(department_name) = list.department_name {
            condition = condition.add(Column::DepartmentName.contains(department_name));
        }

        let vec = Department::find()
            .filter(condition)
            .all(&state.conn)
            .await?;
        Ok(PageResult::new(0, 0, vec.clone(), (&vec).len() as u64))
    }
}