#![allow(non_snake_case)]

use dioxus::prelude::*;

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
    use_context_provider(|| TitleState("HotDog".to_string()));
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
    }
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


#[derive(Clone)]
struct TitleState(String);



fn Title() -> Element {
    // Consume that type as a Context
    let title = use_context::<TitleState>();
    rsx! {
        h1 { "{title.0}" }
    }
}

//DESDE ACA PARA ABAJO ES BACKEND
// on the client:

#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;

    // Open the `dogs.txt` file in append-only mode, creating it if it doesn't exist;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    // And then write a newline to it with the image url
    file.write_fmt(format_args!("{image}\n"));

    Ok(())
}


