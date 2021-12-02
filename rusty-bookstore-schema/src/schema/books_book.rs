//! SeaORM Entity. Generated by sea-orm-codegen 0.3.1

#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;

#[cfg(feature = "frontend")]
use rust_decimal::Decimal;


use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(DeriveEntityModel))]
#[cfg_attr(feature = "backend", sea_orm(table_name = "books_book"))]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    pub id: i64,
    pub name: String,
    pub isbn: String,
    #[cfg_attr(feature = "backend", sea_orm(column_type = "Decimal(Some((10, 2)))"))]
    pub price: Decimal,
    pub quantity: i32,
    #[cfg_attr(feature = "backend", sea_orm(column_type = "Text"))]
    pub description: String,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(EnumIter))]

pub enum Relation {}

#[cfg(feature = "backend")]
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => panic!("No RelationDef"),
        }
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}