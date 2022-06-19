use yew::format::{Json, Nothing};
use yew::prelude::*;
use std::{thread, time};
use std::sync::mpsc::channel;
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}
const notselected: &str = "w-3 h-3 mx-2 bg-gray-300 rounded-full lg:mx-0 focus:outline-none hover:bg-blue-500";
const selected: &str = "w-3 h-3 mx-2 bg-blue-500 rounded-full lg:mx-0 focus:outline-none hover:bg-blue-500";
pub struct btnclrs {
   pub color1: String,
   pub color2: String,
   pub color3: String,
   pub color4: String,
}
pub struct Homec {
    props: Props,
    link: ComponentLink<Self>,
    value: btnclrs,
    msgs: String,
    current: u64,
}

pub enum Msg {
   AddOne,
   AddTwo,
   AddThree,
   AddFour,
}

fn timer() {
  

}
impl Component for Homec {
    type Properties = Props;
    type Message = Msg;
    
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
      let pasho = btnclrs {
         color1: notselected.to_string(),
         color2: notselected.to_string(),
         color3: notselected.to_string(),
         color4: notselected.to_string(),
     };
      link.send_message(Msg::AddOne);
      Self {
         props,
         link,
         value: pasho,
         msgs: "https://dummyimage.com/720x400".to_string(),
         current: 0,
      }
    }

    fn view(&self) -> Html {
      
         self.current = self.current + 1;
         if self.current == 1 {
          self.link.send_message(Msg::AddOne);
         } else if self.current == 2 {
          self.link.send_message(Msg::AddTwo);
         } else if self.current == 3 {
          self.link.send_message(Msg::AddThree);
         } else if self.current == 4 {
          self.link.send_message(Msg::AddFour);
         } else {
          self.current = 1
         }
         log::info!("{}", "In FN Render_Homec");
         
 
         log::info!("{}", "Some Used");
          html! {
             <div class="container flex flex-col px-6 py-4 mx-auto space-y-6 lg:h-[32rem] lg:py-16 lg:flex-row lg:items-center">
             <div class="flex flex-col items-center w-full lg:flex-row lg:w-1/2">
                 <div class="flex justify-center order-2 mt-6 lg:mt-0 lg:space-y-3 lg:flex-col">
                     <button onclick={self.link.callback(|_| Msg::AddOne)} class={&self.value.color1}></button>
                     <button onclick={self.link.callback(|_| Msg::AddTwo)} class={&self.value.color2}></button>
                     <button onclick={self.link.callback(|_| Msg::AddThree)} class={&self.value.color3}></button>
                     <button onclick={self.link.callback(|_| Msg::AddFour)} class={&self.value.color4}></button>
                 </div>
 
                 <div class="max-w-lg lg:mx-12 lg:order-2">
                     <h1
                        class="text-3xl font-medium tracking-wide text-gray-800 dark:text-white lg:text-4xl">{"ShieldHost - Premium Hosting Solutions"}</h1>
                     <p 
                        class="mt-4 text-gray-600 dark:text-gray-300">{"Shield Host offer "}<a class="font-semibold">{"DDoS Protected"}</a>{" Servers with enterprise grade hardware for the lowest possible prices."}<br/>{" We offer Game Server, Virtual Dedicated Servers, and VPS Servers"}</p>
                     <div class="mt-6">
                         <a 
                         href="#" 
                         class="block px-3 py-2 font-semibold text-center text-white
                          transition-colors duration-200 transform bg-blue-500
                           rounded-md lg:inline hover:bg-blue-400">{"Features"}
                         </a>
                     </div>
                 </div>
             </div>
     
             <div class="flex items-center justify-center w-full h-96 lg:w-1/2">
                 <img 
                 class="object-cover w-full h-full max-w-2xl rounded-md" 
                 src={(self.msgs).to_string()} 
                 alt="apple watch photo" />
             </div>
         </div>
 
          }
   }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {

      match msg {
         Msg::AddOne => {
            self.msgs = "https://dummyimage.com/720x400".to_string();

            self.value.color1 = selected.to_string();
            self.value.color2 = notselected.to_string();
            self.value.color3 = notselected.to_string();
            self.value.color4 = notselected.to_string();
            while self.current < 5 {
              thread::sleep(time::Duration::from_secs(5));
              self.current = self.current + 1;
              if self.current == 1 {
                self.link.send_message(Msg::AddTwo);
              }
              } else if self.current == 2 {
                self.link.send_message(Msg::AddTwo);
              } else if self.current == 3 {
                self.link.send_message(Msg::AddThree);
              } else if self.current == 4 {
                self.link.send_message(Msg::AddFour);
              } else {
                self.current = 1
            }
         }
         Msg::AddTwo => {
            self.msgs = "https://dummyimage.com/723x403".to_string();
            self.value.color1 = notselected.to_string();
            self.value.color2 = selected.to_string();
            self.value.color3 = notselected.to_string();
            self.value.color4 = notselected.to_string();
         }
         Msg::AddThree => {
            self.msgs = "https://dummyimage.com/726x403".to_string();
            self.value.color1 = notselected.to_string();
            self.value.color2 = notselected.to_string();
            self.value.color3 = selected.to_string();
            self.value.color4 = notselected.to_string();
         }
         Msg::AddFour => {
            self.msgs = "https://dummyimage.com/728x403".to_string();
            self.value.color1 = notselected.to_string();
            self.value.color2 = notselected.to_string();
            self.value.color3 = notselected.to_string();
            self.value.color4 = selected.to_string();
         }
     }
  
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn destroy(&mut self) {}

    fn rendered(&mut self, _first_render: bool) {}


}
