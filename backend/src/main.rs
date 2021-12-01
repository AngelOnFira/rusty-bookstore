mod schema;

use std::convert::Infallible;

use sea_orm::{Database, DatabaseConnection, EntityTrait};
use warp::{Filter, hyper::StatusCode, Rejection, Reply};

use schema::books_book as book;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db: DatabaseConnection =
        Database::connect("postgres://user:pass@localhost:15432/postgres").await?;

    let books = warp::path!("books").and_then(move || {
        let db_clone = db.clone();

        async move {
            Ok(book::Entity::find()
                .into_json()
                .all(&db_clone)
                .await
                .unwrap())
        }
    });

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let routes = warp::get().and(books).or(hello).recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}