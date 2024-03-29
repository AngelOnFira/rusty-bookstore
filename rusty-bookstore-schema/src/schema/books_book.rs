//! SeaORM Entity. Generated by sea-orm-codegen 0.3.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "books_book")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub isbn: String,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub price: Decimal,
    pub page_count: i32,
    pub genre_id: Option<i64>,
    pub publisher_id: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::books_genre::Entity",
        from = "Column::GenreId",
        to = "super::books_genre::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    BooksGenre,
    #[sea_orm(
        belongs_to = "super::books_publisher::Entity",
        from = "Column::PublisherId",
        to = "super::books_publisher::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    BooksPublisher,
}

impl Related<super::books_genre::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BooksGenre.def()
    }
}

impl Related<super::books_publisher::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BooksPublisher.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
