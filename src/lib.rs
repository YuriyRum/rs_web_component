use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/js/custom_element.js")]
extern "C" {
    fn create_element(name: String, renderer: BaseComponent);
}

enum HandlerVal {
    #[allow(dead_code)]
    Value(RefCell<Box<dyn Component>>),
    None,
}

/// A trait that provides the necessary methods for a custom element lifecycle
///
/// # Basic example
///
/// ```
/// use rs_web_component::{define_component, Component};
/// use wasm_bindgen::prelude::*;
/// use web_sys::{HtmlElement, ShadowRoot, ShadowRootInit, ShadowRootMode};

/// pub enum ThisVal {
///     Value(HtmlElement),
///     None,
/// }

/// pub enum RootVal {
///     Value(ShadowRoot),
///     None,
/// }

/// struct MyComponent {
///     root: RootVal,
///     this: ThisVal,
/// }

/// impl Component for MyComponent {
///     fn init(&mut self, this: HtmlElement) {
///         self.this = ThisVal::Value(this);
///     }

///     fn observed_attributes(&self) -> Vec<String> {
///         return vec!["test".to_string()];
///     }

///     fn attribute_changed_callback(&self, _name: String, _old_value: String, _new_value: String) {
///         if _old_value != _new_value {
///             self.get_root().set_inner_html(self.render().as_str())
///         }
///     }

///     fn connected_callback(&mut self) {
///         self.root = RootVal::Value(
///             self.get_this()
///                 .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
///                 .unwrap(),
///         );

///         self.get_root().set_inner_html(self.render().as_str())
///     }

///     fn disconnected_callback(&self) {}
/// }

/// impl MyComponent {
///     fn render(&self) -> String {
///         "<div><span>Hello from Rust</span></div>".to_string()
///     }

///     fn get_root(&self) -> &ShadowRoot {
///        return match &self.root {
///             RootVal::Value(root) => &root,
///             RootVal::None => panic!("not a root!"),
///         };
///     }

///     fn get_this(&self) -> &HtmlElement {
///         match &self.this {
///             ThisVal::Value(val) => val,
///             ThisVal::None => panic!("not an HtmlElement"),
///         }
///     }
/// }

/// #[wasm_bindgen(start)]
/// fn run() {
///     define_component("test-component".to_string(), || -> Box<dyn Component> {
///         Box::new(MyComponent {
///             root: RootVal::None,
///             this: ThisVal::None,
///         })
///     });
/// }
/// ```

///
/// # An Example with a button and an event handler
///
/// ```

/// use rs_web_component::{define_element, Component};
/// use wasm_bindgen::prelude::*;
/// use web_sys::{
///     CustomEvent, CustomEventInit, Event, HtmlElement, ShadowRoot, ShadowRootInit, ShadowRootMode,
/// };

/// const BUTTON_EVENT_NAME: &str = "buttonClicked";

/// pub enum ThisVal {
///     Value(HtmlElement),
///      None,
/// }

/// pub enum RootVal {
///     Value(ShadowRoot),
///     None,
/// }

/// pub enum CallbackVal {
///     Value(Closure<dyn FnMut(Event) + 'static>),
///     None,
/// }

/// struct MyComponent {
///     root: RootVal,
///     this: ThisVal,
///     callback: CallbackVal,
/// }

/// impl Component for MyComponent {
///     fn init(&mut self, this: HtmlElement) {
///         self.this = ThisVal::Value(this);
///     }
///
///     fn observed_attributes(&self) -> Vec<String> {
///         return vec!["test".to_string()];
///     }
///
///     fn attribute_changed_callback(&self, _name: String, _old_value: String, _new_value: String) {}
///
///     fn connected_callback(&mut self) {
///         self.root = RootVal::Value(
///             self.get_this()
///                 .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
///                 .unwrap(),
///         );
///
///         self.get_root().set_inner_html(self.render().as_str());
///         self.attach_event_handler();
///     }
///
///     fn disconnected_callback(&self) {
///         self.detach_event_handler();
///     }
/// }

/// impl MyComponent {
///     fn render(&self) -> String {
///         "<div><button>Click me</button></div>".to_string()
///     }
///
///     fn attach_event_handler(&mut self) {
///         let btn = self.get_root().query_selector("button").unwrap().unwrap();
///         let closure = Closure::<dyn FnMut(Event) + 'static>::new(move |e: Event| {
///             let evt = CustomEvent::new_with_event_init_dict(
///                 BUTTON_EVENT_NAME,
///                 CustomEventInit::new().composed(true).bubbles(true),
///             )
///             .unwrap();
///             let _ = btn.dispatch_event(&evt);
///         });
///         self.callback = CallbackVal::Value(closure);
///         let btn = self.get_root().query_selector("button").unwrap().unwrap();
///         let _ = btn.add_event_listener_with_callback(
///             "click",
///             self.get_callback().as_ref().unchecked_ref(),
///         );
///     }
///
///     fn detach_event_handler(&self) {
///         let btn = self.get_this().query_selector("button").unwrap().unwrap();
///         let _ = btn.remove_event_listener_with_callback(
///             "click",
///             self.get_callback().as_ref().unchecked_ref(),
///         );
///     }
///
///     fn get_callback(&self) -> &Closure<dyn FnMut(Event) + 'static> {
///         return match &self.callback {
///             CallbackVal::Value(callback) => callback,
///             &CallbackVal::None => panic!("not a callback!"),
///         };
///     }
///
///     fn get_root(&self) -> &ShadowRoot {
///         return match &self.root {
///             RootVal::Value(root) => &root,
///             RootVal::None => panic!("not a root!"),
///         };
///     }
///
///     fn get_this(&self) -> &HtmlElement {
///         match &self.this {
///             ThisVal::Value(val) => val,
///             ThisVal::None => panic!("not an HtmlElement"),
///         }
///     }
/// }
///
/// #[wasm_bindgen(start)]
/// fn run() {
///     define_element("test-component".to_string(), || -> Box<dyn Component> {
///         Box::new(MyComponent {
///             root: RootVal::None,
///             this: ThisVal::None,
///             callback: CallbackVal::None,
///         })
///     });
/// }
/// ```
pub trait Component {
    /// Gives access to a web_sys::HtmlElement
    /// # Arguments
    ///
    /// * `this` - A structure that holds an HtmlElement
    fn init(&mut self, this: HtmlElement);

    /// Returns list of observed attributes
    fn observed_attributes(&self) -> Vec<String> {
        vec![]
    }

    /// Invoked when one of the custom element's attributes is added, removed, or changed.
    /// # Arguments
    ///
    /// * `_name` - A name of an attribute
    /// * `_old_value` - A previous value of an attribute
    /// * `_old_value` - A new value of an attribute
    fn attribute_changed_callback(&self, _name: String, _old_value: JsValue, _new_value: JsValue);

    /// Invoked when the custom element is first connected to the document's DOM.
    fn connected_callback(&mut self);

    /// Invoked when the custom element is disconnected from the document's DOM.
    fn disconnected_callback(&self);

    /// Invoked when the custom element is moved to a new document.
    fn adopted_callback(&self) {}
}

#[wasm_bindgen]
struct BaseComponent {
    #[allow(dead_code)]
    handler: RefCell<HandlerVal>,
    #[allow(dead_code)]
    component_constructor: fn() -> Box<(dyn Component + 'static)>,
}

#[wasm_bindgen]
impl BaseComponent {
    #[allow(dead_code)]
    pub fn init(&mut self, this: HtmlElement) {
        let is_empty = match &(*self.handler.borrow()) {
            HandlerVal::None => true,
            HandlerVal::Value(_val) => false,
        };
        if is_empty {
            self.handler = RefCell::new(HandlerVal::Value(RefCell::new((self
                .component_constructor)(
            ))));
        }
        self.get_handler().get_mut().init(this);
    }

    #[allow(dead_code)]
    pub fn observed_attributes(&mut self) -> Vec<String> {
        let is_empty = match &(*self.handler.borrow()) {
            HandlerVal::None => true,
            HandlerVal::Value(_val) => false,
        };
        if is_empty {
            self.handler = RefCell::new(HandlerVal::Value(RefCell::new((self
                .component_constructor)(
            ))));
        }
        return self.get_handler().get_mut().observed_attributes();
    }

    #[allow(dead_code)]
    pub fn attribute_changed_callback(
        &mut self,
        _name: String,
        _old_value: JsValue,
        _new_value: JsValue,
    ) {
        self.get_handler()
            .get_mut()
            .attribute_changed_callback(_name, _old_value, _new_value);
    }

    #[allow(dead_code)]
    pub fn connected_callback(&mut self) {
        self.get_handler().get_mut().connected_callback();
    }

    #[allow(dead_code)]
    pub fn disconnected_callback(&mut self) {
        self.get_handler().get_mut().disconnected_callback();
    }

    #[allow(dead_code)]
    pub fn adopted_callback(&mut self) {
        self.get_handler().get_mut().adopted_callback();
    }

    fn get_handler(&mut self) -> &mut RefCell<Box<dyn Component>> {
        match self.handler.get_mut() {
            HandlerVal::Value(val) => val,
            HandlerVal::None => panic!("not a component"),
        }
    }
}

/// Defines a new custom element
/// # Arguments
///
/// * `name` - A name of a new custom element
/// * `constructor` - Function/Closure which creates an instance of a custom element
pub fn define_element(name: String, constructor: fn() -> Box<dyn Component>) {
    let renderer: BaseComponent = BaseComponent {
        handler: RefCell::new(HandlerVal::None),
        component_constructor: constructor,
    };
    create_element(name, renderer);
}

/**
 * Creates a template element with the specified content
 * # Arguments
 *
 * * `template_id` - An id of a template
 * * `template_content` - A string representation of a content without the Template tag.
 *                      Can be validated/sanitized with some great libs <https://crates.io/search?q=sanitize%20html>
 */
pub fn add_template(template_id: String, template_content: String) {
    let window = if let Some(window) = web_sys::window() {
        window
    } else {
        panic!("could not get a window");
    };

    let document = if let Some(document) = window.document() {
        document
    } else {
        panic!("could not get a document");
    };

    let template = if let Ok(template) = document.create_element("template") {
        template
    } else {
        panic!("could not create template element");
    };
    template.set_id(&template_id);
    template.set_inner_html(&template_content);

    let body = if let Some(body) = document.body() {
        body
    } else {
        panic!("could not get a body element");
    };

    if let Err(_) = body.append_child(&template) {
        panic!("could not add a template to the body element");
    };
}
