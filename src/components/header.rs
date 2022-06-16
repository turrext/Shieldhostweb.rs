use yew::prelude::*;
use crate::components::Headerlist;
use crate::router::AppRoute;

use yew_router::prelude::*;
pub type Anchor = RouterAnchor<AppRoute>;

pub struct Header {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
   pub icon1: String,
   pub icon2: String,
   pub icon3: String,
   pub icon4: String,
   pub icon5: String, 
   pub icon6: String,
   pub link1: String,
   pub link2: String,
   pub link3: String,
   pub link4: String,
   pub link5: String,
   pub link6: String,
}

impl Component for Header {
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

        html! {
         <div class="h-screen w-screen flex bg-gray-200">
         
            <aside
               class="flex flex-col items-center bg-white text-gray-700 shadow h-full">
               
         
               <div class="h-24 mt-6 flex items-center w-full">
                  
                  <a class="h-24 w-24 mt-6 mx-auto">
                     <img
                        class="h-24 w-24 mx-auto"
                        src="https://i.ibb.co/4VTGxSx/Shieldhost.png"
                        alt="Shieldhost logo" />
                  </a>
               </div>
         
               <ul>
                  <Headerlist svgpath={"M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2
                                      2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1
                                       1 0 011 1v4a1 1 0 001 1m-6 0h6"} svgname={"Home"} svgroute={"AppRoute::Home"}/>
                  <Headerlist svgpath={"M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2
                                      2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2
                                      0 012-2h6a2 2 0 012 2v2M7 7h10"} svgname={"Products"} svgroute={"AppRoute::Home"}/>
                  
                  <Headerlist svgpath={"M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0
                                      1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0
                                      3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"} svgname={"Why Us?"} svgroute={"AppRoute::Home"}/>

                  <Headerlist svgpath={"M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1
                                      0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0
                                      0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2
                                      2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0
                                      0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1
                                      0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0
                                      0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65
                                      0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0
                                      1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0
                                      1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2
                                      0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0
                                      1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0
                                      2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0
                                      0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65
                                      1.65 0 0 0-1.51 1z"} svgname={"Support"} svgroute={"AppRoute::Home"}/>

                  <Headerlist svgpath={"M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002
                                      6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388
                                       6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3
                                      0 11-6 0v-1m6 0H9"} svgname={"Home"} svgroute={"AppRoute::Home"}/>
         
               </ul>
         
               /*<div class="mt-auto h-16 flex items-center w-full">
                  
                  <button
                     class="h-16 w-10 mx-auto flex flex justify-center items-center
                     w-full focus:text-orange-500 hover:bg-red-200 focus:outline-none">
                     <svg
                        class="h-5 w-5 mr-6 float-left align-left text-red-700"
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round">
                        <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
                        <polyline points="16 17 21 12 16 7"></polyline>
                        <line x1="21" y1="12" x2="9" y2="12"></line>
                     </svg>
         
                  </button>
               </div>*/
         
            </aside>
         </div>
        }
    }
}
