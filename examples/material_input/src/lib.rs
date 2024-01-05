use rs_web_component::{define_element, Component};
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, ShadowRoot, ShadowRootInit, ShadowRootMode};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["mdc", "textField", "MDCTextField"], js_name = "attachTo")]
    fn attach_to(obj: Element);
}

pub enum ThisVal {
    Value(HtmlElement),
    None,
}

pub enum RootVal {
    Value(ShadowRoot),
    None,
}

struct MyComponent {
    root: RootVal,
    this: ThisVal,
}

impl Component for MyComponent {
    fn init(&mut self, this: HtmlElement) {
        self.this = ThisVal::Value(this);
    }

    fn observed_attributes(&self) -> Vec<String> {
        vec!["label".to_string()]
    }

    fn attribute_changed_callback(&self, _name: String, _old_value: JsValue, _new_value: JsValue) {
        if !_old_value.is_undefined() {
            self.render();
        }
    }

    fn connected_callback(&mut self) {
        self.root = RootVal::Value(
            self.get_this()
                .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
                .unwrap(),
        );
        self.render();
    }

    fn disconnected_callback(&self) {}
}

impl MyComponent {
    fn render(&self) {
        let label = if let Some(value) = self.get_this().get_attribute("label") {
            value
        } else {
            String::from(" ")
        };
        let html_string = format!(
            r#"
                <link rel="stylesheet" href="https://unpkg.com/material-components-web@latest/dist/material-components-web.min.css">
                <label class="mdc-text-field mdc-text-field--outlined">
                    <span class="mdc-notched-outline">
                        <span class="mdc-notched-outline__leading"></span>
                        <span class="mdc-notched-outline__notch">
                        <span class="mdc-floating-label" id="my-label-id">{}</span>
                        </span>
                        <span class="mdc-notched-outline__trailing"></span>
                    </span>
                    <input type="text" class="mdc-text-field__input" aria-labelledby="my-label-id">
                </label>
            "#,
            label
        );
        self.get_root().set_inner_html(html_string.as_str());
        let element = self.get_root().query_selector("label").unwrap().unwrap();
        attach_to(element);
    }

    fn get_root(&self) -> &ShadowRoot {
        return match &self.root {
            RootVal::Value(root) => &root,
            RootVal::None => panic!("not a root!"),
        };
    }

    fn get_this(&self) -> &HtmlElement {
        match &self.this {
            ThisVal::Value(val) => val,
            ThisVal::None => panic!("not an HtmlElement"),
        }
    }
}

#[wasm_bindgen(start)]
fn run() {
    define_element("material-input".to_string(), || -> Box<dyn Component> {
        Box::new(MyComponent {
            root: RootVal::None,
            this: ThisVal::None,
        })
    });
}
