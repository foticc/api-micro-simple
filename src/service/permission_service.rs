use actix_web::web::{Data, Json};
use sea_orm::{ColumnTrait, EntityTrait, NotSet, QueryFilter, QueryTrait, TransactionTrait};
use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use crate::{AppState, UserError};
use crate::entity::prelude::SysRolePerm;
use crate::entity::sys_role_perm;
use crate::entity::sys_role_perm::ActiveModel;

pub struct PermissionService;
#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignRoleMenuReqDto {
    pub role_id:i32,
    pub perm_codes:Vec<String>,
}

impl PermissionService {


    pub async fn assign_role_perm_code(state:Data<AppState>, dto:PermissionAssignRoleMenuReqDto)->Result<(),UserError>{
        let PermissionAssignRoleMenuReqDto{role_id,perm_codes} = dto;
        let txn  = state.conn.begin().await?;

        let _ = SysRolePerm::delete_many()
            .filter(sys_role_perm::Column::RoleId.eq(role_id))
            .exec(&txn)
            .await?;
        let inserts = perm_codes.iter().map(|f| {
            ActiveModel {
                id: NotSet,
                role_id:Unchanged(role_id),
                perm_code: Set(f.to_string()),
                updated_at: NotSet,
                created_at: Set(Local::now().naive_local()),
                deleted_at: NotSet,
            }
        }).collect::<Vec<ActiveModel>>();

         SysRolePerm::insert_many(inserts)
            .exec(&txn)
            .await?;

        txn.commit().await?;
        Ok(())
    }

    pub async fn get_menus_permission_by_role_id(state:Data<AppState>, id:i32)->Result<Vec<String>,UserError> {
        let vec = SysRolePerm::find()
            .filter(sys_role_perm::Column::RoleId.eq(id))
            .all(&state.conn)
            .await?
            .iter()
            .map(move |d| d.perm_code.to_owned())
            .collect::<Vec<String>>();
        Ok(vec)
    }

}