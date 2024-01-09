use std::ops::Add;

use rs_web_component::{define_element, Component};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, ShadowRoot, ShadowRootInit};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["mdc", "list", "MDCList"])]
    fn attachTo(root: Element);
}

#[derive(Serialize, Deserialize, Clone)]
struct PersonModel {
    name: String,
    lastname: String,
    age: i32,
}

pub struct MyComponent {
    this: Option<HtmlElement>,
    root: Option<ShadowRoot>,
    data: Option<Vec<PersonModel>>,
}

impl Component for MyComponent {
    fn init(&mut self, this: HtmlElement) {
        self.this = Some(this);
    }

    fn attribute_changed_callback(&self, _name: String, _old_value: JsValue, _new_value: JsValue) {
        todo!()
    }

    fn connected_callback(&mut self) {
        self.root = Some(
            self.get_this()
                .attach_shadow(&ShadowRootInit::new(web_sys::ShadowRootMode::Open))
                .unwrap(),
        );
        self.render();
    }

    fn disconnected_callback(&self) {
        todo!()
    }

    fn observed_attributes(&self) -> Vec<String> {
        vec![]
    }

    fn adopted_callback(&self) {}

    fn set_data(&mut self, _data: JsValue) {
        let data_array = match serde_wasm_bindgen::from_value::<Vec<PersonModel>>(_data) {
            Ok(data) => data,
            Err(_) => panic!("wrong data structure"),
        };
        self.data = Some(data_array);
        self.render();
    }
}

impl MyComponent {
    fn render(&mut self) {
        let items_string = self.create_items();
        let html_string = format!(
            r#"
            <link rel="stylesheet" href="https://unpkg.com/material-components-web@latest/dist/material-components-web.min.css">
                <ul class="mdc-list mdc-list--two-line mdc-list--avatar-list">
                    {}
                </ul>
            "#,
            items_string,
        );
        self.get_root().set_inner_html(html_string.as_str());
        let element = self.get_root().query_selector("ul").unwrap().unwrap();
        attachTo(element);
    }

    fn create_items(&self) -> String {
        let items_string =
            self.get_data()
                .iter()
                .fold(String::from(""), |result: String, item| -> String {
                    let value = format!(
                        r#"
                        <li class="mdc-list-item">
                            <span class="mdc-list-item__ripple"></span>
                            <span class="mdc-list-item__text">
                            <span class="mdc-list-item__primary-text">{} {}</span>
                            <span class="mdc-list-item__secondary-text">age: {}</span>
                            </span>
                        </li>
                    "#,
                        item.name, item.lastname, item.age
                    );
                    result.add(value.as_str())
                });
        items_string
    }

    fn get_data(&self) -> Vec<PersonModel> {
        match &self.data {
            Some(data) => data.to_vec(),
            None => vec![],
        }
    }

    fn get_root(&self) -> &ShadowRoot {
        match &self.root {
            Some(value) => value,
            None => panic!("not a root"),
        }
    }

    fn get_this(&self) -> &HtmlElement {
        match &self.this {
            Some(value) => value,
            None => panic!("not an HtmlElement"),
        }
    }
}

#[wasm_bindgen(start)]
fn run() {
    define_element("material-list".to_string(), || -> Box<dyn Component> {
        Box::new(MyComponent {
            root: None,
            this: None,
            data: None,
        })
    });
}
