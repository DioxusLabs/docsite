use dioxus::prelude::*;
use dioxus_sdk::i18n::*;
use dioxus_sdk::translate;
use std::str::FromStr;

fn main() {
    launch(app);
}

static EN_US: &str = r#"{
    "id": "en-US",
    "texts": {
        "messages": {
            "hello_world": "Hello World!"
        },
        "messages.hello": "Hello {name}"
    }
}"#;
static ES_ES: &str = r#"{
    "id": "es-ES",
    "texts": {
        "messages": {
            "hello_world": "Hola Mundo!"
        },
        "messages.hello": "Hola {name}"
    }
}"#;

#[allow(non_snake_case)]
fn Body() -> Element {
    let mut i18 = use_i18();

    let change_to_english = move |_| i18.set_language("en-US".parse().unwrap());
    let change_to_spanish = move |_| i18.set_language("es-ES".parse().unwrap());

    rsx! {
        button { onclick: change_to_english, label { "English" } }
        button { onclick: change_to_spanish, label { "Spanish" } }
        p { {translate!(i18, "messages.hello_world")} }
        p { {translate!(i18, "messages.hello", name: "Dioxus")} }
    }
}

fn app() -> Element {
    use_init_i18n("en-US".parse().unwrap(), "en-US".parse().unwrap(), || {
        let en_us = Language::from_str(EN_US).unwrap();
        let es_es = Language::from_str(ES_ES).unwrap();
        vec![en_us, es_es]
    });

    rsx! { Body {} }
}
