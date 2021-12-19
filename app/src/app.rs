use reqwasm::http::Request;
// use material_yew::MatButton;
// use reqwasm::http::Request;
use rusty_bookstore_schema::schema::books_book as book;
use yew::html;
use yew::prelude::*;
use yewprint::{Card};

use serde::Deserialize;

#[function_component(App)]
pub fn app() -> Html {
    let books = use_state(|| vec![]);
    {
        let books = books.clone();
        use_effect_with_deps(
            move |_| {
                let books = books.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<book::Model> =
                        Request::get("http://localhost:3030/books")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    books.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    let book_nodes = books.iter().map(|book| {
        let book = book.clone();
        html! {
            <div>
                <div>{book.name}</div>
                <div>{book.price}</div>
            </div>
        }
    });

    html! {
        <>
        <p> { "Hello, world!" } </p>
        <ul>
            <>
            {
                for book_nodes
            }
            </>
        </ul>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct BookCardProps {
    pub title: String,
}

#[function_component(BookCard)]
pub fn card(props: &BookCardProps) -> Html {
    html! {
        <Card>
            <p> { "Hello, world!" } </p>
        </Card>
    }
}
