use yew::{prelude::*, html::{IntoOptPropValue, ImplicitClone}};
use crate::router::AppRoute;

use yew_router::prelude::*;
pub type Anchor = RouterAnchor<AppRoute>;
pub struct H2 {
    props: Props,
}



#[derive(Properties, Clone)]
pub struct Props {
   pub svgpath: String,
}

impl Component for H2 {
    type Message = ();
    type Properties = Props;


    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        //self.props = props;
        true
    }

    fn view(&self) -> Html {
      //let mut svgroutes = (self.props.svgroute).to_string();
        //svgpaths.push_str(&self.props.svgpath);
        if &self.props.svgpath == "Home" {
         html! {
                  
            <h2 class="px-6 text-gray-700 text-xl title-font align-center text-center font-bold"><Anchor route=AppRoute::ShieldhostApp>{"Home"}</Anchor></h2>

            }
            
        } else if &self.props.svgpath == "Products" {
         html! {
                  
            <h2 class="px-6 text-gray-700 text-xl title-font align-center text-center font-bold"><Anchor route=AppRoute::Products>{"Products"}</Anchor></h2>

            }
            
        } else if &self.props.svgpath == "Why Us?" {
         html! {
                  
            <h2 class="px-6 text-gray-700 text-xl title-font align-center text-center font-bold"><Anchor route=AppRoute::ShieldhostApp>{"Why Us?"}</Anchor></h2>

            }
            
        } else if &self.props.svgpath == "Support" {
         html! {
                  
            <h2 class="px-6 text-gray-700 text-xl title-font align-center text-center font-bold"><Anchor route=AppRoute::ShieldhostApp>{"Support"}</Anchor></h2>

            }
            
        } else if &self.props.svgpath == "Client Area" {
         html! {
                  
            <h2 class="px-6 text-gray-700 text-xl title-font align-center text-center font-bold"><Anchor route=AppRoute::ShieldhostApp>{"Client Area"}</Anchor></h2>

            }
            
        } else {
         html! {
                  
            <h2 class="px-6 text-gray-700 text-xl title-font align-center text-center font-bold"><Anchor route=AppRoute::ShieldhostApp>{"Not Found"}</Anchor></h2>

            }
        }
            
        

    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}
