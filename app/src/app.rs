// use material_yew::MatButton;
// use reqwasm::http::Request;
use rusty_bookstore_schema::schema::books_book as book;
use yew::format::Nothing;
use yew::html;
use yew::{
    format::Json,
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yewprint::{Card, Elevation};

use serde::Deserialize;

// #[function_component(App)]
// pub fn app() -> Html {
//     let books = use_state(|| vec![]);
//     {
//         let books = books.clone();
//         use_effect_with_deps(
//             move |_| {
//                 let books = books.clone();
//                 wasm_bindgen_futures::spawn_local(async move {
//                     let fetched_books: Vec<book::Model> =
//                         Request::get("http://localhost:3030/books")
//                             .send()
//                             .await
//                             .expect("Failed to fetch books")
//                             .json()
//                             .await
//                             .expect("Failed to parse books");
//                     books.set(fetched_books);
//                 });
//                 || ()
//             },
//             (),
//         );
//     }

//     let books = books
//         .iter()
//         .map(|book| {
//             html! {
//                 <Card>
//                     <p>
//                         {format!("{}: {}", book.name, book.price)}{"test"}
//                     </p>
//                 </Card>
//             }
//         })
//         .collect::<Html>();

//     html! {
//         <>
//         <div>
//         { books }
//         </div>
//         </>
//     }
// }

pub struct App {
    props: ExampleProps,
    backend_fetch: BackendFetchService,
}

#[derive(Clone, PartialEq, Properties, Default)]
pub struct ExampleProps {
    pub elevation: Elevation,
    pub interactive: bool,
}

impl Component for App {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let backend_fetch = BackendFetchService {
            books: None,
            fetch_task: None,
            link: ComponentLink,
            error: None,
        };

        App {
            props,
            backend_fetch,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <Card elevation=self.props.elevation interactive=self.props.interactive>
                <p>
                    {
                        "This is a card component. The elevation of the card can be adjusted. \
                        An interactive card reacts to being moused over."
                    }
                </p>
            </Card>
        }
    }
}

pub struct BookCover {
    props: BookProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct BookProps {
    pub book: book::Model,
}

impl Component for BookCover {
    type Message = ();
    type Properties = BookProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        BookCover { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Card interactive=true>
                <h1>{ self.props.book.name.clone() }</h1>
                <p>{ self.props.book.price }</p>
            </Card>
        }
    }
}

pub struct BackendFetchService {
    books: Option<Vec<book::Model>>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

#[derive(Debug)]
pub enum Msg {
    GetBooks,
    ReceiveResponse(Result<Vec<book::Model>, anyhow::Error>),
}

impl BackendFetchService {
    fn view_iss_location(&self) -> Html {
        match self.books {
            Some(ref books) => {
                html! {
                    <>
                        { books.iter().map(|book| BookCover { props: BookProps {book: *book}}.view() ).collect::<Html>() }
                    </>
                }
            }
            None => {
                html! {
                     <button onclick=self.link.callback(|_| Msg::GetBooks)>
                         { "Loading books" }
                     </button>
                }
            }
        }
    }
}

impl Component for BackendFetchService {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            books: None,
            fetch_task: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            GetBooks => {
                // 1. build the request
                let request = Request::get("http://localhost:3030/books")
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<book::Model>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(books) => {
                        self.books = Some(books);
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
                // we want to redraw so that the page displays the location of the ISS instead of
                // 'fetching...'
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {}
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
