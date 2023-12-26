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
/// # Example
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
/// `
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

    /// Called every time when one of the observed attributes is changed
    /// # Arguments
    ///
    /// * `_name` - A name of an attribute
    /// * `_old_value` - A previous value of an attribute
    /// * `_old_value` - A new value of an attribute
    fn attribute_changed_callback(&self, _name: String, _old_value: String, _new_value: String);

    /// Called when custom element is attached to the DOM
    fn connected_callback(&mut self);

    /// Called when custom element is detached from the DOM
    fn disconnected_callback(&self);
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
        _old_value: String,
        _new_value: String,
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
