use std::collections::HashMap;

use dioxus::{logger::tracing::info, prelude::*};
use sam_ui::{
    input::{Input, InputAppearance},
    popup::{Msg, MsgConfig, Popup, PopupState, Spinner, Toast},
    Elem, Menu, MenuItem,
};
use sam_util::{delete_entity, fetch_data, post_json, put_json};
use shared::{user::UserResponse, Language};

use sam_icon::icon;
use wasm_bindgen::JsCast;

#[component]
pub fn Languages() -> Element {
    let mut show_form = use_signal(|| PopupState::Close);
    let mut edit_language = use_signal(|| None::<Language>);
    let mut languages_resource: Signal<Option<Vec<Language>>> = use_signal(|| None);
    let mut err_msg = use_signal(|| MsgConfig::default());
    let mut success_msg = use_signal(|| MsgConfig::default());
    let mut deleted_lang_id = use_signal(|| None::<i32>);

    let mut confirm_del_msg = use_signal(|| MsgConfig::default());

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

    let mut remove_lang_lacally = move |lang_id: i32| {
        let mut langs = languages_resource().unwrap_or_default();
        langs.retain(|lang| lang.id != lang_id);
        languages_resource.set(Some(langs));
    };

    let mut update_lang_locally = move |lang: Language| {
        let mut langs = languages_resource().unwrap_or_default();
        info!("lang {:?}", lang);
        info!("before langs {:#?}", langs);
        if let Some(index) = langs.iter().position(|l| l.id == lang.id) {
            info!("index {:?}", index);
            langs[index] = lang;
        } else {
            langs.push(lang);
        }
        info!("after langs {:#?}", langs);
        languages_resource.set(Some(langs));
    };

    // Load languages on component mount
    use_effect(move || {
        fetch_languages();
    });

    use_effect(move || {
        deleted_lang_id.with(|id| {
            let id = id.clone();
            if let Some(id) = id {
                confirm_del_msg.set(
                    MsgConfig::with_confirm("You will delete this language permanently.").callback(
                        move || {
                            // handle_delete("id".to_string());
                            let id = id.clone();
                            spawn(async move {
                                let url =
                                    format!("{}/languages/{}", crate::enviroment::BASE_URL, id);
                                match delete_entity(&url).await {
                                    Ok(res) => {
                                        if res.ok() {
                                            // Refresh the list by romoving the deleted lang from the vec in the memory.
                                            // We don't need to refetch the languages from the db.
                                            remove_lang_lacally(id);
                                            deleted_lang_id.set(None);
                                            success_msg.set(MsgConfig::with_success(
                                                "Lanuage deleted successfullly",
                                            ));
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
                        },
                    ),
                );
            }
        });
    });

    let handle_add = move |_| {
        edit_language.set(None);
        show_form.set(PopupState::Open);
    };

    let mut handle_edit = move |lang: Language| {
        edit_language.set(Some(lang));
        show_form.set(PopupState::Open);
    };

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
                    div { class: "",
                        table { class: "table table-bordered w-full",
                            thead {
                                tr {
                                    th { class: "text-left p-3", "Code" }
                                    th { class: "text-left p-3", "Name" }
                                    th { class: "text-left p-3", "Flag" }
                                    th { class: "text-left p-3", "Active" }
                                    th { class: "text-center p-3 w-16", "Actions" }
                                }
                            }
                            tbody {
                                for lang in languages.iter() {
                                    tr { class: "hover:bg-gray-50",
                                        td { class: "p-3 border-b", "{lang.code}" }
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
                                            Menu { custom_class: "dropdown_menu",
                                                MenuItem {
                                                    trigger: rsx! {
                                                        {icon!(LdEllipsis, 20)}
                                                    },
                                                    MenuItem {
                                                        trigger: rsx! { "edit" },
                                                        action: {
                                                            let lang = lang.clone();
                                                            move |_| handle_edit(lang.clone())
                                                        },
                                                    }
                                                    MenuItem {
                                                        trigger: rsx! { "delete" },
                                                        action: {
                                                            let lang_id = lang.id.clone();
                                                            move |_| {
                                                                deleted_lang_id.set(Some(lang_id.clone()));
                                                            }
                                                        },
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

            {Msg(err_msg())}
            {Msg(confirm_del_msg())}
            Popup {
                state: show_form,
                enter_anim_class: "animate__animated animate__zoomIn",
                leave_anim_class: "animate__animated animate__zoomOut",
                LanguageForm {
                    language: edit_language(),
                    on_close: move |lang: Option<Language>| {
                        show_form.set(PopupState::CloseWithAnimation);
                        if let Some(lang) = lang {
                            update_lang_locally(lang);
                            let msg = if edit_language().is_some() {
                                "Language updated successfully!"
                            } else {
                                "Language added successfully!"
                            };
                            success_msg.set(MsgConfig::with_success(msg));
                        }
                        edit_language.set(None);
                    },
                }
            }
            {Toast(success_msg())}
        }
    }
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct LanguageFormProps {
    pub language: Option<Language>,
    pub on_close: EventHandler<Option<Language>>,
}

#[component]
pub fn LanguageForm(props: LanguageFormProps) -> Element {
    let mut err_msg = use_signal(|| MsgConfig::default());
    let mut spinner_state = use_signal(|| PopupState::Close);
    let mut language_id = use_signal(|| 0);
    let mut language_code = use_signal(String::new);
    let mut language_name = use_signal(String::new);
    let mut language_flag = use_signal(String::new);
    let mut language_is_active = use_signal(|| false);

    let is_edit = props.language.is_some();

    // Initialize form with existing data if in edit mode
    use_effect(move || {
        if let Some(language) = &props.language {
            language_id.set(language.id.clone());
            language_code.set(language.code.clone());
            language_name.set(language.name.clone());
            language_flag.set(language.flag.clone());
            language_is_active.set(language.active.clone());
        } else {
            language_code.set(String::new());
            language_name.set(String::new());
            language_flag.set(String::new());
            language_is_active.set(false);
        }
    });

    // Focus on the first input field when form opens
    use_effect(move || {
        spawn(async move {
            // Small delay to ensure the DOM is rendered
            gloo_timers::future::TimeoutFuture::new(100).await;

            Elem::from("input[name='code']").focus();
            // if let Some(window) = web_sys::window() {
            //     if let Some(document) = window.document() {
            //         if let Ok(Some(element)) = document.query_selector("input[name='code']") {
            //             if let Ok(input_element) = element.dyn_into::<web_sys::HtmlInputElement>() {
            //                 let _ = input_element.focus();
            //             }
            //         }
            //     }
            // }
        });
    });

    let handle_submit = move |_| {
        spinner_state.set(PopupState::Open);
        spawn(async move {
            let language = Language {
                id: language_id(),
                code: language_code(),
                name: language_name(),
                flag: language_flag(),
                active: language_is_active(),
            };

            let url = format!("{}/languages", crate::enviroment::BASE_URL);
            let result = if is_edit {
                // PUT request for editing
                put_json(&url, &language).await
            } else {
                // POST request for adding
                post_json(&url, &language).await
            };

            spinner_state.set(PopupState::Close);
            match result {
                Ok(res) => {
                    let user_res: UserResponse = res.json().await.unwrap();
                    if res.ok() {
                        props.on_close.call(Some(language));
                    } else {
                        err_msg.set(MsgConfig::with_err(user_res.message()));
                    }
                }
                Err(e) => {
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
        div { class: "language-form m-2.5",
            div { class: "flex justify-between items-center",
                h2 { class: "text-xl font-bold", "{form_title}" }
            }
            div { class: "flex flex-col gap-11 mt-16",
                Input {
                    name: "code",
                    appearance: InputAppearance::square,
                    label: "Language Code",
                    value: language_code(),
                    oninput: move |evt: FormEvent| {
                        info!("code: {}", evt.value());
                        language_code.set(evt.value());
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
                    class: "btn-sec",
                    onclick: move |_| props.on_close.call(None),
                    "Cancel"
                }
                button { class: "btn", onclick: handle_submit, "{button_text}" }
            }
            {Msg(err_msg())}
            Spinner { state: spinner_state }
        }
    }
}
