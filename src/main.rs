#![allow(non_snake_case)]

use dioxus::prelude::*;
mod components;
mod backend;

use crate::components::*;
use crate::backend::*;

//mod guide_backend;
//TODO hacer que el backend este en otro .rs!!!
static CSS: Asset = asset!("/assets/main.css");


#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}


fn main() {
    dioxus::launch(App);
}



#[component]
fn App() -> Element {
//    use_context_provider(|| TitleState("HotDog".to_string()));
    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}
/// An enum of all of the possible routes in the app.
#[derive(Routable, PartialEq, Clone)]
#[rustfmt::skip]
enum Route {
    // All routes under the NavBar layout will be rendered inside of the NavBar Outlet
    #[layout(NavBar)]
        #[route("/")]
        DogView {},
        #[route("/favorites")]
        Favorites {}, // <------ add this new variant
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },


}

#[component]
fn DogView() -> Element { 
    
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>() .await
            .unwrap()
            .message
    });    

    rsx! {
        
        //Title {}
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button {
                id: "save",
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = save_dog(current).await;
                },

                "save!"
            }
        }
    }    
}
#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
/*
#[derive(Clone)]
struct TitleState(String);



fn Title() -> Element {
    // Consume that type as a Context
    let title = use_context::<TitleState>();
    rsx! {
        h1 { "{title.0}" }
    }
}

*/

//DESDE ACA PARA ABAJO ES BACKEND
// on the client:

