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

    fn attribute_changed_callback(&self, _name: String, _old_value: JsValue, _new_value: JsValue) {}

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
