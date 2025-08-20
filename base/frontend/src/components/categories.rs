use std::collections::HashMap;

use dioxus::{logger::tracing::info, prelude::*};
use sam_ui::{
    input::{Input, InputAppearance},
    popup::{Msg, MsgConfig, PopupState, Spinner, Toast},
};
use sam_util::{fetch_data, post_json, put_json};
use shared::{user::UserResponse, Category, CategoryName, CategoryWithNames, Language};

#[component]
pub fn Categories() -> Element {
    rsx! {
        div { class: "categories", AddCategory {} }
    }
}

#[component]
pub fn AddCategory() -> Element {
    rsx! {
        CategoryForm { is_edit_mode: false }
    }
}

#[component]
pub fn EditCategory(category: Category, names: Vec<CategoryName>) -> Element {
    rsx! {
        CategoryForm {
            edited_category: category,
            edited_names: names,
            is_edit_mode: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct CategoryFormProps {
    pub edited_category: Option<Category>,
    pub edited_names: Option<Vec<CategoryName>>,
    #[props(default = false)]
    pub is_edit_mode: bool,
}

#[component]
pub fn CategoryForm(props: CategoryFormProps) -> Element {
    let mut msg = use_signal(|| MsgConfig::default());
    let mut err_msg = use_signal(|| MsgConfig::default());
    let mut spinner_state = use_signal(|| PopupState::Close);
    let mut names: Signal<HashMap<String, CategoryName>> = use_signal(HashMap::new);
    let mut category_id = use_signal(String::new);
    let mut parent_id = use_signal(String::new);
    let is_edit = props.is_edit_mode;

    // Initialize form with existing data if in edit mode
    use_effect(move || {
        if let Some(category) = &props.edited_category {
            category_id.set(category.id.clone());
            parent_id.set(category.parent_id.clone().unwrap_or_default());
        }

        if let Some(edited_names) = &props.edited_names {
            let mut names_map = HashMap::new();
            for name in edited_names {
                names_map.insert(name.language_id.clone(), name.clone());
            }
            names.set(names_map);
        }
    });

    let languages: Resource<Vec<Language>> = use_resource(move || async move {
        let url = format!("{}/languages", crate::enviroment::BASE_URL);
        match fetch_data(&url).await {
            Ok(res) => match res.json::<UserResponse>().await {
                Ok(user_res) => {
                    serde_json::from_value::<Vec<Language>>(user_res.json().unwrap()).unwrap()
                }
                Err(e) => {
                    err_msg.set(MsgConfig::with_err(e.to_string()));
                    vec![]
                }
            },
            Err(e) => {
                err_msg.set(MsgConfig::with_err(e.to_string()));
                vec![]
            }
        }
    });

    let mut reset = move || {
        if !is_edit {
            category_id.set(String::new());
            parent_id.set(String::new());
            names.write_unchecked().clear();
        }
    };

    let send_request = move |_| {
        spinner_state.set(PopupState::Open);
        async move {
            let p_id = if parent_id().trim().is_empty() {
                None
            } else {
                Some(parent_id())
            };
            let category = Category::new(category_id(), p_id);
            let category_names: Vec<CategoryName> =
                names.read_unchecked().values().cloned().collect();
            let category_with_names = CategoryWithNames {
                category,
                names: category_names,
            };

            let result = if is_edit {
                // PUT request for editing
                let url = format!(
                    "{}/categories/{}",
                    crate::enviroment::BASE_URL,
                    category_id()
                );
                put_json(&url, &category_with_names).await
            } else {
                // POST request for adding
                let url = format!("{}/categories", crate::enviroment::BASE_URL);
                post_json(&url, &category_with_names).await
            };

            match result {
                Ok(res) => {
                    spinner_state.set(PopupState::Close);
                    let user_res: UserResponse = res.json().await.unwrap();
                    if res.ok() {
                        let success_msg = if is_edit {
                            "Category updated successfully!"
                        } else {
                            "Category added successfully!"
                        };
                        msg.set(MsgConfig::with_success(success_msg));
                        reset();
                    } else {
                        err_msg.set(MsgConfig::with_err(user_res.message()));
                    }
                }
                Err(e) => {
                    spinner_state.set(PopupState::Close);
                    err_msg.set(MsgConfig::with_err(e.to_string()));
                }
            }
        }
    };

    let form_title = if is_edit {
        "Edit Category"
    } else {
        "Add Category"
    };
    let button_text = if is_edit {
        "Update Category"
    } else {
        "Add Category"
    };

    rsx! {
        div { class: "categories",
            h2 { class: "text-xl font-bold mb-4", "{form_title}" }
            div { class: "flex flex-col gap-4",
                div { class: "p-5 w-[400px]",
                    Input {
                        name: "id",
                        appearance: InputAppearance::square,
                        label: "Category Code",
                        value: category_id(),
                        disabled: is_edit, // Disable ID field in edit mode
                        oninput: move |evt: FormEvent| {
                            if !is_edit {
                                category_id.set(evt.value());
                            }
                        },
                    }
                }
                div { class: "p-5 w-[400px]",
                    Input {
                        name: "parent_id",
                        appearance: InputAppearance::rounded,
                        label: "Parent",
                        value: parent_id(),
                        oninput: move |evt: FormEvent| {
                            parent_id.set(evt.value());
                        },
                    }
                }
            }
            {
                match &*languages.read_unchecked() {
                    Some(langs) => rsx! {
                        div { class: "flex flex-col gap-4",
                            for lang in langs.clone() {
                                div { class: "p-5 w-[400px]",
                                    Input {
                                        name: "{lang.id}",
                                        appearance: InputAppearance::square,
                                        label: "{lang.name}",
                                        value: names.read_unchecked().get(&lang.id).map_or(String::new(), |n| n.name.clone()),
                                        oninput: move |evt: FormEvent| {
                                            let name = evt.value();
                                            let slug = format!("{}-{}", lang.id.clone(), category_id());
                                            let category_name = CategoryName::new(
                                                name,
                                                lang.id.clone(),
                                                category_id(),
                                                slug,
                                            );
                                            names.write_unchecked().insert(lang.id.clone(), category_name);
                                        },
                                    }
                                }
                            }
                        }
                    },
                    None => rsx! {
                        div { "Loading languages..." }
                    },
                }
            }
            button {
                class: if is_edit { "btn btn-warning" } else { "btn btn-primary" },
                onclick: send_request,
                "{button_text}"
            }
            {
                if names.read_unchecked().is_empty() {
                    rsx! {
                        div { "No category names added yet." }
                    }
                } else {
                    rsx! {
                        ul { class: "category-names-list",
                            for (lang_id , name) in names.read_unchecked().iter() {
                                li { class: "category-name-item", "{name.name} >>> {name.language_id} >>> {name.category_id}" }
                            }
                        }
                    }
                }
            }
        }
        {Toast(msg())}
        {Msg(err_msg())}
        Spinner { state: spinner_state }
    }
}
