use js_sys::Array;
use wasm_bindgen::JsValue;

/// Converts a space-separated string into a JavaScript array.
pub fn to_js_array<T: AsRef<str>>(str: T) -> js_sys::Array {
    let list: Vec<&str> = str.as_ref().split(" ").collect();
    let js_array = list.iter().map(|c| JsValue::from_str(c)).collect::<Array>();
    js_array
}
