use yew::{prelude::*, html::{IntoOptPropValue, ImplicitClone}};
use crate::router::AppRoute;
use crate::components::H2;
use yew_router::prelude::*;
pub type Anchor = RouterAnchor<AppRoute>;
pub struct Headerlist {
    props: Props,
}



#[derive(Properties, Clone)]
pub struct Props {
   pub svgpath: String,
   pub svgname: String,
}

impl Component for Headerlist {
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
        let mut svgpaths = (self.props.svgpath).to_string();
        let mut svgnames = (self.props.svgname).to_string();
      //let mut svgroutes = (self.props.svgroute).to_string();
        //svgpaths.push_str(&self.props.svgpath);
        if &svgnames == "Home" || &svgnames == "Products" || &svgnames == "Why Us?" || &svgnames == "Support" || &svgnames == "Client Area"  {
            html! {
                  
                <li class="hover:bg-gray-100">
                   <a
                      class="h-16 px-6 flex flex items-center w-full
                      focus:text-orange-500">

                      <svg
                         class="h-7 w-7 mr-6 float-left align-left mr-6 float-left align-left"
                         xmlns="http://www.w3.org/2000/svg"
                         width="24"
                         height="24"
                         viewBox="0 0 24 24"
                         fill="none"
                         stroke="currentColor"
                         stroke-width="2"
                         stroke-linecap="round"
                         stroke-linejoin="round">
                         <path strokeLinecap="round" strokeLinejoin="round" 
                           d={svgpaths}/>
                      </svg>
                      <H2 svgpath={svgnames}/>
                   </a>

                </li>
       
      }

        } else {
            html! {
                  
                <li class="hover:bg-gray-100">
                   <a
                      class="h-16 px-6 flex flex items-center w-full
                      focus:text-orange-500">

                      <svg
                         class="h-7 w-7 mr-6 float-left align-left mr-6 float-left align-left"
                         xmlns="http://www.w3.org/2000/svg"
                         width="24"
                         height="24"
                         viewBox="0 0 24 24"
                         fill="none"
                         stroke="currentColor"
                         stroke-width="2"
                         stroke-linecap="round"
                         stroke-linejoin="round">
                         <path strokeLinecap="round" strokeLinejoin="round" 
                           d={svgpaths}/>
                      </svg>
                      <H2 svgpath={"Error"}/>
                   </a>

                </li>
       
      }
        }
        
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}
