//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq,Serialize,Deserialize)]
#[sea_orm(table_name = "menu")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)] // Skip deserializing
    pub id: i32,
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
    pub updated_at: Option<DateTime>,
    #[serde(skip_deserializing)] // Skip deserializing
    pub created_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
