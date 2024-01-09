A sample project that shows how a custom component can be created in Rust.

v0.1.5
- The function set_data is added. Allows to pass state to a cusstom element.
- [An example with a material two line list and a state was added](https://github.com/YuriyRum/rs_web_component/tree/master/examples/material_two_line_list

v0.1.4:
- [An issue with the arguments type of the attribute_changed callback function has been fixed](https://github.com/YuriyRum/rs_web_component/issues/1)
- [An example with a material outlined text field was added](https://github.com/YuriyRum/rs_web_component/tree/master/examples/material_input)
- The function adopted_callback with the default implementation was added

v0.1.3:
- The function add_template was added
- An example with a template was added

[Documentation v0.1.5](https://docs.rs/rs_web_component/0.1.5/rs_web_component)

Examples:
- [Basic example](https://github.com/YuriyRum/rs_web_component/tree/master/examples/simple_web_component)
- [Button with the event handler](https://github.com/YuriyRum/rs_web_component/tree/master/examples/button_with_event_handler)
- [An example with a template](https://github.com/YuriyRum/rs_web_component/tree/master/examples/simple_template)
- [An example with a material outlined text field](https://github.com/YuriyRum/rs_web_component/tree/master/examples/material_input)

Basic example:
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

Example with the event handler:

```
use rs_web_component::{define_element, Component};
use wasm_bindgen::prelude::*;
use web_sys::{
    CustomEvent, CustomEventInit, Event, HtmlElement, ShadowRoot, ShadowRootInit, ShadowRootMode,
};

const BUTTON_EVENT_NAME: &str = "buttonClicked";

pub enum ThisVal {
    Value(HtmlElement),
    None,
}

pub enum RootVal {
    Value(ShadowRoot),
    None,
}

pub enum CallbackVal {
    Value(Closure<dyn FnMut(Event) + 'static>),
    None,
}

struct MyComponent {
    root: RootVal,
    this: ThisVal,
    callback: CallbackVal,
}

impl Component for MyComponent {
    fn init(&mut self, this: HtmlElement) {
        self.this = ThisVal::Value(this);
    }

    fn observed_attributes(&self) -> Vec<String> {
        return vec!["test".to_string()];
    }

    fn attribute_changed_callback(&self, _name: String, _old_value: String, _new_value: String) {}

    fn connected_callback(&mut self) {
        self.root = RootVal::Value(
            self.get_this()
                .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
                .unwrap(),
        );

        self.get_root().set_inner_html(self.render().as_str());
        self.attach_event_handler();
    }

    fn disconnected_callback(&self) {
        self.detach_event_handler();
    }
}

impl MyComponent {
    fn render(&self) -> String {
        "<div><button>Click me</button></div>".to_string()
    }

    fn attach_event_handler(&mut self) {
        let btn = self.get_root().query_selector("button").unwrap().unwrap();
        let closure = Closure::<dyn FnMut(Event) + 'static>::new(move |e: Event| {
            let evt = CustomEvent::new_with_event_init_dict(
                BUTTON_EVENT_NAME,
                CustomEventInit::new().composed(true).bubbles(true),
            )
            .unwrap();
            let _ = btn.dispatch_event(&evt);
        });
        self.callback = CallbackVal::Value(closure);
        let btn = self.get_root().query_selector("button").unwrap().unwrap();
        let _ = btn.add_event_listener_with_callback(
            "click",
            self.get_callback().as_ref().unchecked_ref(),
        );
    }

    fn detach_event_handler(&self) {
        let btn = self.get_this().query_selector("button").unwrap().unwrap();
        let _ = btn.remove_event_listener_with_callback(
            "click",
            self.get_callback().as_ref().unchecked_ref(),
        );
    }

    fn get_callback(&self) -> &Closure<dyn FnMut(Event) + 'static> {
        return match &self.callback {
            CallbackVal::Value(callback) => callback,
            &CallbackVal::None => panic!("not a callback!"),
        };
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
            callback: CallbackVal::None,
        })
    });
}
```