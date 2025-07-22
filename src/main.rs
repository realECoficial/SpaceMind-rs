#![allow(non_snake_case)]
mod components;
mod backend;
use dioxus::prelude::*;
use crate::components::*;
use crate::backend::*;
//mod guide_backend;
//TODO hacer que el backend este en otro .rs!!!
static CSS: Asset = asset!("/assets/main.css");



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
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

