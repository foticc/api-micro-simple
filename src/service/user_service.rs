use actix_web::web::Data;
use argon2::PasswordVerifier;
use log::info;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DbErr, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QuerySelect, QueryTrait, SelectColumns, TransactionTrait, TryIntoModel};
use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::prelude::DateTime;
use sea_orm::sea_query::UnOper::Not;
use sea_orm::sqlx::encode::IsNull::No;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use crate::entity::user::{ActiveModel, Column, Model};
use crate::{AppState, UserError};
use crate::common::result::{FilterParam, PageResult};
use crate::common::security::Security;
use crate::entity::prelude::{SysRolePerm, SysUserRole, User};

pub struct UserService;
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub user_name: String,
    pub password:Option<String>,
    pub sex: i32,
    pub available: bool,
    pub telephone: String,
    pub mobile: String,
    pub email: String,
    pub department_id: i32,
    pub role_id:Vec<i32>,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    pub id:i32,
    #[serde(flatten)]
    pub user: CreateUser,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserName {
    pub id:i32,
    pub password: String,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchParams {
    pub user_name: Option<String>,
    pub department_id:Option<i32>,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteParam {
    pub ids: Vec<i32>,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub role_id: Vec<i32>,
    #[serde(flatten)]
    pub result:Option<Model>
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword {
    pub id: i32,
    pub new_password: String,
    pub old_password: String,
}

impl UserService {


    pub async fn create_user(state:Data<AppState>,user: CreateUser) -> Result<Model,UserError> {
        if user.password.is_none() {
            return Err(UserError::Error(String::from("Invalid password")));
        }
        let password = Security::hash_password(user.password.unwrap().as_str())?;
        let model = ActiveModel {
            id: NotSet,
            email: Set(Some(user.email)),
            user_name: Set(user.user_name),
            password: Set(password),
            available: Set(user.available),
            sex: Set(user.sex),
            mobile: Set(user.mobile),
            telephone: Set(Some(user.telephone)),
            department_id: Set(user.department_id),
            last_login_time: NotSet,
            updated_at: NotSet,
            created_at: Set(Local::now().naive_local()),
            deleted_at: NotSet,
        };
        let txn = state.conn.begin().await?;
        let x = model.insert(&txn).await?;
        let user_role:Vec<crate::entity::sys_user_role::ActiveModel> = user.role_id.iter()
            .map(|&m| {
                crate::entity::sys_user_role::ActiveModel {
                    id: NotSet,
                    role_id: Set(m),
                    user_id: Set(x.id),
                    updated_at: NotSet,
                    created_at: Set(Local::now().naive_local()),
                    deleted_at: NotSet,
                }
            }).collect();

        SysUserRole::insert_many(user_role)
            .exec(&txn).await?;
        txn.commit().await?;
        Ok(x)
    }

    pub async fn find_one_by_user_name(state:Data<AppState>, user_name: String)->Result<UserName,DbErr> {
        let option = User::find()
            .filter(Column::UserName.eq(&user_name))
            .one(&state.conn)
            .await?;
        if let Some(user) = option {
            Ok(UserName {
                id: user.id,
                password: user.password,
            })
        }else {
            Err(DbErr::RecordNotFound(user_name.clone()))
        }
    }

    pub async fn find_all(state:Data<AppState>, page: FilterParam<SearchParams>) ->Result<PageResult<Model>,DbErr> {
        let mut conditions = Condition::all();
        if let Some(f) = page.filters {
            if let Some(user_name) = f.user_name {
                conditions = conditions.add(Column::UserName.contains(user_name));
            }
            if let Some(department_id) = f.department_id {
                conditions = conditions.add(Column::DepartmentId.eq(department_id));
            }
        }
        let paginator = User::find()
            .filter(conditions)
            .paginate(&state.conn, page.page_size);

        let total = paginator.num_items().await?;
        paginator.fetch_page(page.page_index-1)
            .await.map(|list| {
            PageResult::new(page.page_index,page.page_size,list,total)
        })
    }

    pub async fn find_one(state:Data<AppState>,id:i32)->Result<UserDto, DbErr> {
        let mut user_dto = UserDto {
            role_id: vec![],
            result: None,
        };
        let option = User::find_by_id(id)
            .one(&state.conn)
            .await?;
        if option.is_none() {
            return Err(DbErr::RecordNotFound(id.to_string()));
        }
        user_dto.result = Some(option.unwrap());
        let vec:Vec<i32> = SysUserRole::find()
            .filter(crate::entity::sys_user_role::Column::UserId.eq(id))
            .all(&state.conn)
            .await?
            .iter()
            .map(|m|m.role_id)
            .collect();
        user_dto.role_id = vec;
        Ok(user_dto)
    }

    pub async fn find_one_auth_code(state:Data<AppState>,id: i32) ->Result<Vec<String>,DbErr> {
        info!("{:?}",id);
        let roles = SysUserRole::find()
            .filter(crate::entity::sys_user_role::Column::UserId.eq(id))
            .all(&state.conn)
            .await?
            .iter().map(|m| m.role_id).collect::<Vec<_>>();
        println!("{:?}",roles);
        let vec = SysRolePerm::find()
            .filter(crate::entity::sys_role_perm::Column::RoleId.is_in(roles))
            .all(&state.conn)
            .await?
            .iter().map(|m|m.perm_code.to_string()).collect::<Vec<_>>();
        Ok(vec)
    }

    pub async fn update(state:Data<AppState>,update_user: UpdateUser)->Result<Model,UserError> {
        let txn = state.conn.begin().await?;
        let update_model = ActiveModel {
            id: Set(update_user.id),
            email: Set(Some(update_user.user.email)),
            user_name: Set(update_user.user.user_name),
            password: NotSet,
            available: Set(update_user.user.available),
            sex: Set(update_user.user.sex),
            mobile: Set(update_user.user.mobile),
            telephone: Set(Some(update_user.user.telephone)),
            department_id: Set(update_user.user.department_id),
            last_login_time: NotSet,
            updated_at: Set(Some(Local::now().naive_local())),
            created_at: NotSet,
            deleted_at: NotSet,
        };
        let model = update_model.update(&txn).await?;

        SysUserRole::delete_many()
            .filter(crate::entity::sys_user_role::Column::UserId.eq(update_user.id))
            .exec(&txn).await?;

        let sys_user_role:Vec<crate::entity::sys_user_role::ActiveModel> = update_user.user.role_id
            .iter()
            .map(|&m| {
                crate::entity::sys_user_role::ActiveModel {
                    id: NotSet,
                    role_id: Set(m),
                    user_id: Set(update_user.id),
                    updated_at: NotSet,
                    created_at: Set(Local::now().naive_local()),
                    deleted_at: NotSet,
                }
            })
            .collect();
        if !sys_user_role.is_empty() {
             SysUserRole::insert_many(sys_user_role)
                .exec(&txn).await?;
        }

        txn.commit().await?;
        Ok(model)
    }

    pub async fn change_pwd(state:Data<AppState>, pwd:ChangePassword)->Result<(),UserError> {
        let option = User::find_by_id(pwd.id)
            .one(&state.conn)
            .await?;
        if option.is_none() {
           return Err(UserError::DbErr(DbErr::RecordNotFound(pwd.id.to_string())));
        }
        let mut model = option.unwrap();
        let verify = Security::verify(&model.password, &pwd.old_password);
        if !verify {
            return Err(UserError::Error("old password is invalid".to_string()));
        }
        let new_pass = Security::hash_password(pwd.new_password.as_str())?;
        let active_model = ActiveModel {
            id: Set(model.id),
            email: NotSet,
            user_name: NotSet,
            password: Set(new_pass),
            available: NotSet,
            sex: NotSet,
            mobile: NotSet,
            telephone: NotSet,
            department_id: NotSet,
            last_login_time: NotSet,
            updated_at: Set(Some(Local::now().naive_local())),
            created_at: NotSet,
            deleted_at: NotSet,
        };
        let _ = active_model.update(&state.conn).await?;
        Ok(())
    }

    pub async fn delete(state:Data<AppState>,ids:DeleteParam)->Result<u64,DbErr> {
        let result = User::delete_many()
            .filter(Column::Id.is_in(ids.ids))
            .exec(&state.conn)
            .await?;
        Ok(result.rows_affected)
    }

}