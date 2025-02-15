//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "department")]
pub struct Model {
    #[serde(skip_deserializing)] // Skip deserializing
    #[sea_orm(primary_key)]
    pub id: i32,
    pub father_id: Option<i32>,
    pub department_name: Option<String>,
    pub order_num: Option<i32>,
    pub state: Option<bool>,
    pub updated_at: Option<DateTime>,
    #[serde(skip_deserializing)] // Skip deserializing
    pub created_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
