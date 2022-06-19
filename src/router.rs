use yew_router::prelude::*;

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/Products"]
    Products,
    #[to = "/"]
    ShieldhostApp,
}