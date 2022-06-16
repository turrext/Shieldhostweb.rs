use yew::format::{Json, Nothing};
use super::super::Toied as Toied;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use serde_derive::Deserialize;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub toied_id: i32,
}

pub struct Detail {
    props: Props,
    link: ComponentLink<Self>,
    toied: Option<Toied>,
    fetch_task: Option<FetchTask>,
}

impl Detail {
    fn render_detail(&self, toied: &Option<Toied>) -> Html {
        log::info!("{}", "In FN Render_detail");
        
        match toied {
            Some(t) => {
                log::info!("{}", "Some Used");
                html! {
                    <div class=classes!("detail")>
                        <h1>{&t.title}{" ("}<span class=classes!("id")>{t.albumId}</span>{")"}</h1>
                        <div>{"by user "}{t.id}</div>
                        <img class="product_card_image" src=classes!({&t.thumbnailUrl})/>
                    </div>
                }
            }
            None => {
                html! {
                    <div class=classes!("loading")>{"loading..."}</div>
                }
            }
        }
    }
}

pub enum Msg {
    MakeReq(i32),
    Resp(Result<Toied, anyhow::Error>),
}

impl Component for Detail {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        log::info!("{}", "Xxx");
        link.send_message(Msg::MakeReq(props.toied_id));
        Self {
            props,
            link,
            toied: None,
            fetch_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_detail(&self.toied)}
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq(id) => {
                let req = Request::get(&format!(
                    "https://jsonplaceholder.typicode.com/photos/{}",
                    id
                ))
                .body(Nothing)
                .expect("can make req to jsonplaceholder");
                
                let cb =
                    self.link
                        .callback(|response: Response<Json<Result<Toied, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::Resp(data)
                        });

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.toied = Some(data);
                    log::info!("Data: {}", "Data Assigned!");
                } else {
                    log::info!("Data: {}", "Not OK");
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn destroy(&mut self) {}
}
