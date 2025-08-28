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

fn set_direction(dir: &str) {
    if let Some(document) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
    {
        let _ = document.set_attribute("dir", dir);
    }
}

pub fn to_rtl() {
    set_direction("rtl");
}

pub fn to_ltr() {
    set_direction("ltr");
}

pub fn is_rtl() -> bool {
    let document = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element());

    if let Some(element) = document {
        let dir = element
            .get_attribute("dir")
            .unwrap_or_else(|| "ltr".to_string());
        dir.to_lowercase() == "rtl"
    } else {
        false
    }
}
