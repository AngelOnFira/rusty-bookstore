use rusty_bookstore_schema::schema::books_book as book;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db: DatabaseConnection =
        Database::connect("postgres://user:pass@localhost:15432/postgres").await?;

    let books = warp::path!("books").then(move || {
        let db_clone = db.clone();

        async move { warp::reply::json(&book::Entity::find().all(&db_clone).await.unwrap()) }
    });

    let cors = warp::cors().allow_any_origin();
    let routes = warp::get().and(books).with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
