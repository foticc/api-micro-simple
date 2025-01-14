use crate::entity::role::{ActiveModel, Model};
use crate::{AppState, UserError};
use actix_web::web::{Data, Json};
use log::info;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DbErr, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use crate::common::result::{FilterParam, PageResult};
use crate::entity::role::Column;
use crate::entity::prelude::{Role};

pub struct RoleService;

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoleDto {
    pub role_name: String,
    pub role_desc: String,
}

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchRoleDto {
    pub role_name: Option<String>,
    pub role_desc: Option<String>,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct DelParams {
    pub ids:Vec<i32>
}

#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRole {
    pub id:i32,
    #[serde(flatten)]
    pub create_role_dto: CreateRoleDto,
}

impl RoleService {
    pub async fn create(state:Data<AppState>, dto: CreateRoleDto) ->Result<Model,UserError> {
        let model = ActiveModel {
            id: NotSet,
            role_name: Set(dto.role_name),
            role_desc: Set(Some(dto.role_desc)),
            updated_at: NotSet,
            created_at: Set(Local::now().naive_local()),
            deleted_at: NotSet,
        };
        let x = model.insert(&state.conn).await?;
        Ok(x)
    }

    pub async fn find_all(state:Data<AppState>, dto: FilterParam<SearchRoleDto>) ->Result<PageResult<Model>,DbErr>{
        let mut condition = Condition::all();
        if let Some(filter) = dto.filters {
            if let Some(role_name) = filter.role_name {
                condition = condition.add(Column::RoleName.contains(role_name));
            }
        }
        let list = Role::find()
            .filter(condition)
            .all(&state.conn)
            .await?;
        Ok(
            PageResult::new(0, 0, list.clone(), list.len() as u64)
        )
    }
    

    pub async fn find_one(state:Data<AppState>, id:i32) ->Result<Model,DbErr> {
        let option = Role::find_by_id(id)
            .one(&state.conn)
            .await?;
        if let Some(s) = option {
            Ok(s)
        }else {
            Err(DbErr::RecordNotFound(id.to_string()))
        }
    }

    pub async fn update(state:Data<AppState>,update_params: UpdateRole)->Result<Model,UserError> {
        let model = ActiveModel {
            id: Set(update_params.id),
            role_name: Set(update_params.create_role_dto.role_name),
            role_desc: Set(Some(update_params.create_role_dto.role_desc)),
            updated_at: Set(Some(Local::now().naive_local())),
            created_at: NotSet,
            deleted_at: NotSet,
        };
        let result = model.update(&state.conn).await?;
        Ok(result)
    }

    pub async fn delete(state:Data<AppState>,del_params :DelParams)->Result<u64,UserError> {
        let result = Role::delete_many()
            .filter(Column::Id.is_in(del_params.ids))
            .exec(&state.conn)
            .await?;

        Ok(result.rows_affected)
    }
}