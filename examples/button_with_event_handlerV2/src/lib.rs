use rs_web_component::{define_element_v2, ComponentV2};
use wasm_bindgen::prelude::*;
use web_sys::{
    CustomEvent, CustomEventInit, Event, HtmlElement, ShadowRoot, ShadowRootInit, ShadowRootMode,
};

const BUTTON_EVENT_NAME: &str = "buttonClicked";

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
    callback: CallbackVal,
}

impl ComponentV2 for MyComponent {
    fn init(&mut self, this: HtmlElement) {
  
    }

    fn observed_attributes(&self, this: HtmlElement) -> Vec<String> {
        return vec!["test".to_string()];
    }

    fn attribute_changed_callback(&self, _name: String, _old_value: JsValue, _new_value: JsValue, _this: HtmlElement) {}

    fn connected_callback(&mut self, this: HtmlElement) {
        self.root = RootVal::Value(
            this.attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
                .unwrap(),
        );

        self.get_root().set_inner_html(self.render().as_str());
        self.attach_event_handler();
    }

    fn disconnected_callback(&self, this: HtmlElement) {
        self.detach_event_handler(this);
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

    fn detach_event_handler(&self, this: HtmlElement) {
        let btn = this.query_selector("button").unwrap().unwrap();
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
}

#[wasm_bindgen(start)]
fn run() {
    define_element_v2("test-component".to_string(), || -> Box<dyn ComponentV2> {
        Box::new(MyComponent {
            root: RootVal::None,
            callback: CallbackVal::None,
        })
    });
}
