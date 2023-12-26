A sample project to see how a custom component can be created in Rust.

Example:
```
use rs_web_component::{define_element, Component};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, ShadowRoot, ShadowRootInit, ShadowRootMode};

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
        return vec!["test".to_string()];
    }

    fn attribute_changed_callback(&self, _name: String, _old_value: String, _new_value: String) {
        if _old_value != _new_value {
            self.get_root().set_inner_html(self.render().as_str())
        }
    }

    fn connected_callback(&mut self) {
        self.root = RootVal::Value(
            self.get_this()
                .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
                .unwrap(),
        );

        self.get_root().set_inner_html(self.render().as_str())
    }

    fn disconnected_callback(&self) {}
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

    fn get_this(&self) -> &HtmlElement {
        match &self.this {
            ThisVal::Value(val) => val,
            ThisVal::None => panic!("not an HtmlElement"),
        }
    }
}

#[wasm_bindgen(start)]
fn run() {
    define_element("test-component".to_string(), || -> Box<dyn Component> {
        Box::new(MyComponent {
            root: RootVal::None,
            this: ThisVal::None,
        })
    });
}
```
