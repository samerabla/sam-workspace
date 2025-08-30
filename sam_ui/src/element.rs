use wasm_bindgen::JsCast;
use web_sys::window;

/// A wrapper around `web_sys::Element` to provide additional functionality
#[derive(Debug, Clone)]
pub struct Elem(pub Option<web_sys::Element>);

impl Elem {
    pub fn to_web_sys_elem(self) -> Option<web_sys::Element> {
        self.0
    }

    pub fn to_html_elem(self) -> Option<web_sys::HtmlElement> {
        if let Some(el) = self.0 {
            el.dyn_into::<web_sys::HtmlElement>().ok()
        } else {
            None
        }
    }
    pub fn remove_class(&self, name: &str) {
        if let Some(elem) = &self.0 {
            let arr = sam_util::to_js_array(name);
            elem.class_list().remove(&arr).ok();
        }
    }

    pub fn focus(self) {
        if let Some(el) = self.to_html_elem() {
            let _ = el.focus();
        }
    }
}

/// Implement conversion from `&str` (CSS selector)
impl From<&str> for Elem {
    fn from(selector: &str) -> Self {
        let element = window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.query_selector(selector).ok().flatten());
        Elem(element)
    }
}

impl From<String> for Elem {
    fn from(selector: String) -> Self {
        Elem::from(selector.as_str()) // Convert `String` to `&str`
    }
}

impl From<&String> for Elem {
    fn from(selector: &String) -> Self {
        Elem::from(selector.as_str()) // Convert `&String` to `&str`
    }
}

/// Implement conversion from `Element`
impl From<web_sys::Element> for Elem {
    fn from(element: web_sys::Element) -> Self {
        Elem(Some(element))
    }
}
