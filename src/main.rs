#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::prelude::*;
use gloo_events::EventListener;
use js_sys::Array;
use serde::Serialize;
use std::{ops::Deref, rc::Rc};
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{Blob, BlobPropertyBag, FileReader, HtmlAnchorElement, HtmlInputElement, Url};

#[derive(Serialize)]
struct Json {
    text: String,
    int: u32,
    float: f32,
}

fn App(cx: Scope) -> Element {
    let download_json = move |_| {
        let anchor = gloo_utils::document()
            .create_element("a")
            .unwrap_throw()
            .unchecked_into::<HtmlAnchorElement>();
        anchor.set_download("my_data.json");

        let content = Json {
            text: "hello, world!".to_string(),
            int: 10,
            float: 3.0_f32,
        };

        let mut blob_option = BlobPropertyBag::new();
        blob_option.type_("application/json");

        let blob = Blob::new_with_str_sequence_and_options(
            &Array::of1(&JsValue::from_str(
                serde_json::to_string(&content).unwrap_throw().as_str(),
            )),
            &blob_option,
        )
        .unwrap_throw();

        let object_url = Url::create_object_url_with_blob(&blob).unwrap_throw();
        anchor.set_href(&object_url);

        anchor.click();

        Url::revoke_object_url(&object_url).unwrap_throw();
    };

    let upload_json_v1 = move |_| {
        let input = gloo_utils::document()
            .get_element_by_id("json-file-input")
            .unwrap_throw()
            .unchecked_into::<HtmlInputElement>();
        let file = input.files().unwrap_throw().item(0).unwrap_throw();
        let file_reader = Rc::new(FileReader::new().unwrap_throw());
        EventListener::once(file_reader.deref(), "load", {
            let file_reader = Rc::clone(&file_reader);
            move |_| {
                let result_string = file_reader
                    .result()
                    .unwrap_throw()
                    .as_string()
                    .unwrap_throw();
                gloo_console::log!(JsValue::from_str(&result_string));
            }
        })
        .forget();

        file_reader.read_as_text(&file).unwrap_throw();
    };

    cx.render(rsx! {
        div {
            button {onclick: download_json, "jsonファイルをダウンロード"}
        }
        div {
            input {r#type: "file", id: "json-file-input", onchange: upload_json_v1, "jsonファイルをアップロード v1"}
        }
    })
}

fn main() {
    dioxus_web::launch(App);
}
