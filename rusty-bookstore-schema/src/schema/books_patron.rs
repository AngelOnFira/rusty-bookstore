//! SeaORM Entity. Generated by sea-orm-codegen 0.3.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "books_patron")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::books_order::Entity")]
    BooksOrder,
    #[sea_orm(has_many = "super::books_basket::Entity")]
    BooksBasket,
}

impl Related<super::books_order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BooksOrder.def()
    }
}

impl Related<super::books_basket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BooksBasket.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}