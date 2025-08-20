use std::collections::HashMap;

use dioxus::{logger::tracing::info, prelude::*};
use sam_ui::{
    input::{Input, InputAppearance},
    popup::{Msg, MsgConfig, PopupState, Spinner, Toast},
};
use sam_util::{delete_entity, fetch_data, post_json, put_json};
use shared::{user::UserResponse, Language};

use sam_ui::header::*;

#[component]
pub fn Languages() -> Element {
    let mut show_form = use_signal(|| false);
    let mut edit_language = use_signal(|| None::<Language>);
    let mut context_menu_open = use_signal(|| None::<String>);
    let mut languages_resource: Signal<Option<Vec<Language>>> = use_signal(|| None);
    let mut err_msg = use_signal(|| MsgConfig::default());

    // Fetch languages
    let fetch_languages = move || {
        spawn(async move {
            let url = format!("{}/languages", crate::enviroment::BASE_URL);
            match fetch_data(&url).await {
                Ok(res) => match res.json::<UserResponse>().await {
                    Ok(user_res) => {
                        let langs =
                            serde_json::from_value::<Vec<Language>>(user_res.json().unwrap())
                                .unwrap();
                        languages_resource.set(Some(langs));
                    }
                    Err(e) => {
                        err_msg.set(MsgConfig::with_err(e.to_string()));
                    }
                },
                Err(e) => {
                    err_msg.set(MsgConfig::with_err(e.to_string()));
                }
            }
        });
    };

    // Load languages on component mount
    use_effect(move || {
        fetch_languages();
    });

    let handle_add = move |_| {
        edit_language.set(None);
        show_form.set(true);
    };

    let mut handle_edit = move |lang: Language| {
        edit_language.set(Some(lang));
        show_form.set(true);
        context_menu_open.set(None);
    };

    let mut handle_delete = move |lang_id: String| {
        spawn(async move {
            let url = format!("{}/languages/{}", crate::enviroment::BASE_URL, lang_id);
            match delete_entity(&url).await {
                Ok(res) => {
                    if res.ok() {
                        fetch_languages(); // Refresh the list
                    } else {
                        let user_res: UserResponse = res.json().await.unwrap();
                        err_msg.set(MsgConfig::with_err(user_res.message()));
                    }
                }
                Err(e) => {
                    err_msg.set(MsgConfig::with_err(e.to_string()));
                }
            }
        });
        context_menu_open.set(None);
    };

    let mut handle_form_close = move || {
        show_form.set(false);
        edit_language.set(None);
        fetch_languages(); // Refresh the list after form closes
    };

    let mut toggle_context_menu = move |lang_id: String| {
        if context_menu_open() == Some(lang_id.clone()) {
            context_menu_open.set(None);
        } else {
            context_menu_open.set(Some(lang_id));
        }
    };

    let dropdown_menu_edit = sam_ui::Menu::new("edit");
    let dropdown_menu_delete = sam_ui::Menu::new("delete");
    let dropdown_menu = sam_ui::Menu::new("action")
        .to_root()
        .children(vec![dropdown_menu_edit, dropdown_menu_delete]);

    rsx! {
        div { class: "languages-container p-6",
            // Header with Add button
            div { class: "flex justify-between items-center mb-6",
                h1 { class: "text-2xl font-bold", "Languages" }
                button {
                    class: "btn btn-primary flex items-center gap-2",
                    onclick: handle_add,
                    span { class: "text-xl", "+" }
                    "Add Language"
                }
            }

            // Languages table
            if let Some(languages) = languages_resource() {
                if languages.is_empty() {
                    div { class: "text-center py-8 text-gray-500",
                        "No languages found. Add your first language!"
                    }
                } else {
                    div { class: "overflow-x-auto",
                        table { class: "table table-bordered w-full",
                            thead {
                                tr {
                                    th { class: "text-left p-3", "ID" }
                                    th { class: "text-left p-3", "Name" }
                                    th { class: "text-left p-3", "Flag" }
                                    th { class: "text-left p-3", "Active" }
                                    th { class: "text-center p-3 w-16", "Actions" }
                                }
                            }
                            tbody {
                                for lang in languages.iter() {
                                    tr { class: "hover:bg-gray-50",
                                        td { class: "p-3 border-b", "{lang.id}" }
                                        td { class: "p-3 border-b", "{lang.name}" }
                                        td { class: "p-3 border-b", "{lang.flag}" }
                                        td { class: "p-3 border-b",
                                            if lang.active {
                                                "Yes"
                                            } else {
                                                "No"
                                            }
                                        }
                                        td { class: "p-3 border-b text-center relative",
                                            button { class: "btn btn-ghost btn-sm p-1",
                                                sam_ui::header::MenuBar { menu_list: vec![dropdown_menu.clone()] }
                                            }
                                            // Context menu
                                            if context_menu_open() == Some(lang.id.clone()) {
                                                div { class: "absolute right-0 top-8 bg-white border border-gray-200 rounded-md shadow-lg z-10 min-w-32",
                                                    div { class: "py-1",
                                                        button {
                                                            class: "w-full text-left px-4 py-2 hover:bg-gray-100 text-sm",
                                                            onclick: {
                                                                let lang = lang.clone();
                                                                move |_| handle_edit(lang.clone())
                                                            },
                                                            "Edit"
                                                        }
                                                        button {
                                                            class: "w-full text-left px-4 py-2 hover:bg-red-50 text-red-600 text-sm",
                                                            onclick: {
                                                                let lang_id = lang.id.clone();
                                                                move |_| handle_delete(lang_id.clone())
                                                            },
                                                            "Delete"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "text-center py-8",
                    div { class: "loading loading-spinner loading-lg" }
                    div { class: "mt-2", "Loading languages..." }
                }
            }

            // Form modal overlay
            if show_form() {
                div {
                    class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                    onclick: move |_| handle_form_close(),
                    div {
                        class: "bg-white rounded-lg p-6 max-w-md w-full mx-4",
                        onclick: move |e| e.stop_propagation(),
                        LanguageForm {
                            language: edit_language(),
                            on_close: handle_form_close,
                        }
                    }
                }
            }

            {Msg(err_msg())}
        }
    }
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct LanguageFormProps {
    pub language: Option<Language>,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn LanguageForm(props: LanguageFormProps) -> Element {
    let mut msg = use_signal(|| MsgConfig::default());
    let mut err_msg = use_signal(|| MsgConfig::default());
    let mut spinner_state = use_signal(|| PopupState::Close);
    let mut language_id = use_signal(String::new);
    let mut language_name = use_signal(String::new);
    let mut language_flag = use_signal(String::new);
    let mut language_is_active = use_signal(|| false);

    let is_edit = props.language.is_some();

    // Initialize form with existing data if in edit mode
    use_effect(move || {
        if let Some(language) = &props.language {
            language_id.set(language.id.clone());
            language_name.set(language.name.clone());
            language_flag.set(language.flag.clone());
            language_is_active.set(language.active.clone());
        } else {
            language_id.set(String::new());
            language_name.set(String::new());
            language_flag.set(String::new());
            language_is_active.set(false);
        }
    });

    let handle_submit = move |_| {
        spinner_state.set(PopupState::Open);
        spawn(async move {
            let language = Language {
                id: language_id(),
                name: language_name(),
                flag: language_flag(),
                active: language_is_active(),
            };

            let result = if is_edit {
                // PUT request for editing
                let url = format!("{}/languages", crate::enviroment::BASE_URL);
                put_json(&url, &language).await
            } else {
                // POST request for adding
                let url = format!("{}/languages", crate::enviroment::BASE_URL);
                post_json(&url, &language).await
            };

            match result {
                Ok(res) => {
                    spinner_state.set(PopupState::Close);
                    let user_res: UserResponse = res.json().await.unwrap();
                    if res.ok() {
                        let success_msg = if is_edit {
                            "Language updated successfully!"
                        } else {
                            "Language added successfully!"
                        };
                        msg.set(MsgConfig::with_success(success_msg));
                        // Close form after success
                        props.on_close.call(());
                    } else {
                        err_msg.set(MsgConfig::with_err(user_res.message()));
                    }
                }
                Err(e) => {
                    spinner_state.set(PopupState::Close);
                    err_msg.set(MsgConfig::with_err(e.to_string()));
                }
            }
        });
    };

    let form_title = if is_edit {
        "Edit Language"
    } else {
        "Add Language"
    };

    let button_text = if is_edit {
        "Update Language"
    } else {
        "Add Language"
    };

    rsx! {
        div { class: "language-form",
            div { class: "flex justify-between items-center mb-4",
                h2 { class: "text-xl font-bold", "{form_title}" }
                button {
                    class: "btn btn-ghost btn-sm",
                    onclick: move |_| props.on_close.call(()),
                    "✕"
                }
            }
            div { class: "flex flex-col gap-4",
                Input {
                    name: "id",
                    appearance: InputAppearance::square,
                    label: "Language ID",
                    value: language_id(),
                    disabled: is_edit, // Disable ID field in edit mode
                    oninput: move |evt: FormEvent| {
                        if !is_edit {
                            language_id.set(evt.value());
                        }
                    },
                }
                Input {
                    name: "name",
                    appearance: InputAppearance::square,
                    label: "Language Name",
                    value: language_name(),
                    oninput: move |evt: FormEvent| {
                        language_name.set(evt.value());
                    },
                }
                Input {
                    name: "flag",
                    appearance: InputAppearance::square,
                    label: "Flag URL",
                    value: language_flag(),
                    oninput: move |evt: FormEvent| {
                        language_flag.set(evt.value());
                    },
                }
            }
            div { class: "flex justify-end gap-2 mt-6",
                button {
                    class: "btn btn-ghost",
                    onclick: move |_| props.on_close.call(()),
                    "Cancel"
                }
                button {
                    class: if is_edit { "btn btn-warning" } else { "btn btn-primary" },
                    onclick: handle_submit,
                    "{button_text}"
                }
            }
            {Toast(msg())}
            {Msg(err_msg())}
            Spinner { state: spinner_state }
        }
    }
}
