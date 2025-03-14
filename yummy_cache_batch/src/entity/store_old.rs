//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "store_old")]
pub struct Model {
    pub name: String,
    pub r#type: Option<String>,
    pub address: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 7)))")]
    pub lat: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 7)))")]
    pub lng: Decimal,
    pub reg_dt: Option<DateTime>,
    pub reg_id: Option<String>,
    pub chg_dt: Option<DateTime>,
    pub chg_id: Option<String>,
    #[sea_orm(primary_key)]
    pub seq: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
