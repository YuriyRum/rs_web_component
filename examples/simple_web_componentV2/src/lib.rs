use rs_web_component::{define_element_v2, ComponentV2};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, ShadowRoot, ShadowRootInit, ShadowRootMode};

pub enum RootVal {
    Value(ShadowRoot),
    None,
}

struct MyComponent {
    root: RootVal,
}

impl ComponentV2 for MyComponent {
    fn init(&mut self, this: HtmlElement) {
       
    }

    fn observed_attributes(&self, this: HtmlElement) -> Vec<String> {
        return vec!["test".to_string()];
    }

    fn attribute_changed_callback(&self, _name: String, _old_value: JsValue, _new_value: JsValue, _this: HtmlElement) {
        if !_old_value.is_undefined() {
            self.get_root().set_inner_html(self.render().as_str())
        }
    }

    fn connected_callback(&mut self, this: HtmlElement) {
        self.root = RootVal::Value(
            this.attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
                .unwrap(),
        );

        self.get_root().set_inner_html(self.render().as_str())
    }

    fn disconnected_callback(&self, this: HtmlElement) {}
}

impl MyComponent {
    fn render(&self) -> String {
        "<div><span>Hello from Rust</span></div>".to_string()
    }

    fn get_root(&self) -> &ShadowRoot {
        return match &self.root {
            RootVal::Value(root) => &root,
            RootVal::None => panic!("not a root!"),
        };
    }
}

#[wasm_bindgen(start)]
fn run() {
    define_element_v2("test-component".to_string(), || -> Box<dyn ComponentV2> {
        Box::new(MyComponent {
            root: RootVal::None,
        })
    });
}
