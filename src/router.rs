use yew_router::prelude::*;

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/todo/{id}"]
    Detail(i32),
    #[to = "/"]
    Home,
}