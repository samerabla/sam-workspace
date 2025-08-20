pub fn set_session_storage(key: &str, value: &str) {
    if let Some(win) = web_sys::window() {
        if let Ok(Some(ss)) = win.session_storage() {
            let _ = ss.set_item(key, value);
        }
    }
}

pub fn get_session_storage(key: &str) -> Option<String> {
    if let Some(win) = web_sys::window() {
        if let Ok(Some(ss)) = win.session_storage() {
            return ss.get_item(key).ok().flatten();
        }
    }
    None
}

pub fn remove_session_storage(key: &str) {
    if let Some(win) = web_sys::window() {
        if let Ok(Some(ss)) = win.session_storage() {
            let _ = ss.remove_item(key);
        }
    }
}
