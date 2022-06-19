#![recursion_limit = "256"]
mod components;
use crate::components::Header;
use crate::components::Homec;


use serde_derive::Deserialize;
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::html;
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};
use crate::router::AppRoute;
use yew_router::prelude::*;
mod router;

pub type Anchor = RouterAnchor<AppRoute>;
mod Pages;
struct ShieldhostApp {
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub user_id: u64,
    pub id: u64,
    pub title: String,
    pub completed: bool,
}
#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Toied {
    albumId: u64,
    id: u64,
    title: String,
    url: String,
    thumbnailUrl: String,
  
}

impl Component for ShieldhostApp {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        
        Self {
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
       
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {

        //&arry.val1 = "e3";
        //&arry.info1 = "e3".to_string();
        //&arry.val2 = "e3".to_string();
        //&arry.info2 = "e3".to_string();
        //&arry.val3 = "e3".to_string();
        //&arry.info3 = "e3".to_string();
        //&arry.val4 = "e3".to_string();
        //&arry.info4 = "e3".to_string();
        //let icon1shi = ({<svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">

        html! {
            <div class="h-screen w-screen flex bg-gray-200">
                    <Header icon1="" icon2="" icon3="" icon4="" icon5="" icon6="" link1="" link2="" link3="" link4="" link5="" link6="" />
                    <div class=classes!("content")>
                    <Router<AppRoute, ()>
                                render = Router::render(move |switch: AppRoute| {
                                    match switch {
                                        AppRoute::Products => {
                                            html! {
                                                <div>
                                                    <Pages::Products::Products/>
                                                </div>}
                                        }
                                        AppRoute::ShieldhostApp => {
                                            html! {
                                                <Homec />
                                            }
                                        }
                                    }
                                })
                            />
                        </div>
                    </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

/*
enum Msg {
    MakeReq,
    Resp(Result<Vec<Todo>, anyhow::Error>),
}
impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq);
        Self {
            link,
            todos: None,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                self.todos = None;
                let req = Request::get("https://jsonplaceholder.typicode.com/todos")
                    .body(Nothing)
                    .expect("can make req to jsonplaceholder");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<Todo>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.todos = Some(data);
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let todos = self.todos.clone();
        let cb = self.link.callback(|_| Msg::MakeReq);
        ConsoleService::info(&format!("render TodoApp: {:?}", todos));
        html! {
            <div class=classes!("todo")>
                <div class=classes!("nav")>
                    <Anchor route=AppRoute::Home>{"Home"}</Anchor>
                </div>
                <div class=classes!("content")>
                    <Router<AppRoute, ()>
                        render = Router::render(move |switch: AppRoute| {
                            match switch {
                                AppRoute::Detail(todo_id) => {
                                    html! {
                                        <div>
                                            <todo::detail::Detail toied_id=todo_id/>
                                        </div>}
                                }
                                AppRoute::Home => {
                                    html! {
                                        <div>
                                            <div class=classes!("refresh")>
                                                <button onclick=cb.clone()>
                                                    { "refresh" }
                                                </button>
                                            </div>
                                            <todo::list::List todos=todos.clone()/>
                                        </div>
                                    }
                                }
                            }
                        })
                    />
                </div>
            </div>
        }
    }
}
*/
#[wasm_bindgen(start)]
pub fn run_app() {
    println!("Starting App");
    println!("Hello World");
    ConsoleService::info("Starting App");
    App::<ShieldhostApp>::new().mount_to_body();
    wasm_logger::init(wasm_logger::Config::default());
}
