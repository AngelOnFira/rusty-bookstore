use reqwasm::http::Request;
use rusty_bookstore_schema::schema::books_book as book;
use yew::prelude::*;
use yew::{function_component, html};
use yewprint::Button;

#[function_component(App)]
pub fn app() -> Html {
    let books = use_state(|| vec![]);
    {
        let books = books.clone();
        use_effect_with_deps(
            move |_| {
                let books = books.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_books: Vec<book::Model> = Request::get("localhost:3000/books")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    books.set(fetched_books);
                });
                || ()
            },
            (),
        );
    }

    let books = books
        .iter()
        .map(|book| {
            html! {
                <p>{format!("{}: {}", book.name, book.price)}</p>
            }
        })
        .collect::<Html>();

    html! {
        <>
        <div>
        { books }
        </div>
        </>
    }
}

// pub struct App {
//     dark_theme: bool,
//     link: ComponentLink<Self>,
// }

// pub enum Msg {
//     ToggleLight,
// }

// impl Component for App {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//         App {
//             dark_theme: web_sys::window()
//                 .and_then(|x| x.match_media("(prefers-color-scheme: dark)").ok().flatten())
//                 .map(|x| x.matches())
//                 .unwrap_or(true),
//             link,
//         }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         match msg {
//             Msg::ToggleLight => {
//                 self.dark_theme ^= true;
//                 true
//             }
//         }
//     }

//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         true
//     }

//     fn view(&self) -> Html {
//         html! {
//             <div class=classes!(self.dark_theme.then(|| "bp3-dark"))>
//                 <Button
//                     onclick=self.link.callback(|_| Msg::ToggleLight)
//                     icon=IconName::Flash
//                 >
//                     {"Toggle light"}
//                 </Button>
//             </div>
//         }
//     }
// }

// #[function_component(HelloWorld)]
// fn hello_world() -> Html {
//     // let books = use_state(|| vec![]);
//     // {
//     //     let books = books.clone();
//     //     use_effect_with_deps(
//     //         move |_| {
//     //             let books = books.clone();
//     //             wasm_bindgen_futures::spawn_local(async move {
//     //                 let fetched_videos: Vec<Video> =
//     //                     Request::get("https://yew.rs/tutorial/data.json")
//     //                         .send()
//     //                         .await
//     //                         .unwrap()
//     //                         .json()
//     //                         .await
//     //                         .unwrap();
//     //                 books.set(fetched_videos);
//     //             });
//     //             || ()
//     //         },
//     //         (),
//     //     );
//     // }

//     html! { "Hello world" }
// }
