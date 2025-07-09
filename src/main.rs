use dioxus::{ prelude::*};
static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element { 
    let mut img_src = use_signal(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
     
    let skip = move |_| { img_src.set("https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.elmueble.com%2Fmedio%2F2023%2F03%2F02%2Fperro-de-raza-beagle_67c65dda_230302133829_900x900.jpg&f=1&nofb=1&ipt=e915db5adb99c1563e1990ddf83b1467aef30d673bc8ca1dec4e143f6490c921"); };
    let save = move |_| { img_src.set("https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.aon.es%2Fpersonales%2Fseguro-perro-gato%2Fwp-content%2Fuploads%2Fsites%2F2%2F2021%2F04%2Fbichon-maltes.jpg&f=1&nofb=1&ipt=b18738be135d5e3392e11f6674a723b0dff3c31a93f5595fdd67af21d63023f3");
        }; 
        
    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        div { id: "dogview",
            //img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
            div { id: "buttons",
                button { onclick: skip, id: "skip",  "skip" }
                button { onclick: save, id: "save",  "save!" }
            }
        }
    }
}
