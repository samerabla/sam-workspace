/////////////////////////////////////////////////
// Developer Notes
/////////////////////////////////////////////////

// To use a lib you should first add a feature in cargo.toml:
//      dioxus-free-icons = { version = "0.9", features = ["bootstrap","font-awesome-regular"] }
// All the features of dioxus-free-icons: https://crates.io/crates/dioxus-free-icons
// When no icons lib specified Lucide icons will be used

/////////////////////////////////////////////////
////// Ld Icon
/////////////////////////////////////////////////

/// A macro generates an icon using dioxus-free-icon crate.
///
/// ### Search Icon :
///
/// -  [dioxus_free_icons](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/index.html) first the name of icon you want because not all icons availabe
///
/// - [Lucide Icons](https://lucide.dev/icons/)
///
/// - [React Icons](https://react-icons.github.io/react-icons/)
///
/// - [Bootstrap icons](https://icons.getbootstrap.com/)
///
/// - [Fontawesome icons](https://fontawesome.com/search)  
///
///
/// # Examples
/// By default Lucide icons will be used
///
/// ```
/// The order of params: [name size color bg color_hover bg_hover action]
/// color=stroke | bg=fill
///
/// # Defaults
/// size: 30
/// color: "black"
/// fill: "white"
///
/// Example with only name
/// icon!(LdX)
///
/// Example with name, size
/// icon!(LdX,60)
///
/// Example with name, size, color
/// icon!(LdX,40,"red")
///
/// Example with name, size, color, bg
/// icon!(LdX,60,"white","black")
///
/// Example with name, size, color, bg, color_hover, bg_hover
/// icon!(LdX,60,"white","black","white","black")
///
/// Example with action:
/// rsx! {
///         div {
///                 {
///                    let action = move |e: MouseEvent| {
///                         info!("Clicked on icon with mouse event: {:#?}", e);
///                     };
///                    sam_icon::icon!(LdMail, 20, "white", "blue", "blue", "white", onclick : action)
///                 }
///         }
///},
///
/// ```
#[cfg(feature = "lucide")]
#[macro_export]
macro_rules! icon {
    (
        $name:ident
    ) => {{
        use $crate::Icon;
        use $crate::icons::ld_icons::$name;
        rsx! {
            Icon {
                width: 30,
                height: 30,
                icon: $name,
            }
        }
    }};

    (
        $name:ident,
        $size:expr
    ) => {{
        use $crate::Icon;
        use $crate::icons::ld_icons::$name;
        rsx! {
            Icon {
                width: $size,
                height: $size,
                icon: $name,
            }
        }
    }};

    (
        $name:ident,
        $size:expr,
        $color:expr
    ) => {{
        use $crate::Icon;
        use $crate::icons::ld_icons::$name;
        rsx! {
            Icon {
                width: $size,
                height: $size,
                icon: $name,
                style: format!("stroke:{};fill:'white';",$color)

            }
        }
    }};

    (
        $name:ident,
        $size:expr,
        $color:expr,
        $bg:expr
    ) => {{
        use $crate::Icon;
        use $crate::icons::ld_icons::$name;
        rsx! {
            Icon {
                width: $size,
                height: $size,
                icon: $name,
                style: format!("stroke:{};fill:{};",$color,$bg)
            }
        }
    }};

    (
        $name:ident,
        $size:expr,
        $color:expr,
        $bg:expr,
        $color_hover:expr,
        $bg_hover:expr,
    ) => {{
        use dioxus::prelude::*;
        use $crate::Icon;
        use $crate::icons::ld_icons::$name;
        let mut color = use_signal(|| $color);
        let mut bg = use_signal(|| $bg);
        rsx! {
            div {
                onmouseover: move |_| {
                    color.set($color_hover);
                    bg.set($bg_hover);
                },
                onmouseout: move |_| {
                    color.set($color);
                    bg.set($bg);
                },
                Icon {
                    width: $size,
                    height: $size,
                    icon: $name,
                    style: format!("stroke:{};fill:{};",color(),bg())
                }
            }
        }
    }};

    (
        $name:ident,
        $size:expr,
        $color:expr,
        $bg:expr,
        $color_hover:expr,
        $bg_hover:expr,
        onclick: $onclick:ident
    ) => {{
        use dioxus::prelude::*;
        use $crate::Icon;
        use $crate::icons::ld_icons::$name;
        let mut color = use_signal(|| $color);
        let mut bg = use_signal(|| $bg);
        rsx! {
            div {
                onclick: move |e| {
                    $onclick(e);
                },
                onmouseover: move |_| {
                    color.set($color_hover);
                    bg.set($bg_hover);
                },
                onmouseout: move |_| {
                    color.set($color);
                    bg.set($bg);
                },
                Icon {
                    width: $size,
                    height: $size,
                    icon: $name,
                    style: format!("stroke:{};fill:{};",color(),bg())
                }
            }
        }
    }};
}

/////////////////////////////////////////////////
////// Io Icon
/////////////////////////////////////////////////
/////////////////////////////////////////////////
#[cfg(feature = "ionicons")]
#[macro_export]
macro_rules! icon_io {
    (
        $name:ident
    ) => {{
        use $crate::Icon;
        use $crate::icons::io_icons::$name;
        rsx! {
            Icon {
                width: 30,
                height: 30,
                icon: $name,
            }
        }
    }};

    (
        $name:ident,
        $size:expr
    ) => {{
        use $crate::Icon;
        use $crate::icons::io_icons::$name;
        rsx! {
            Icon {
                width: $size,
                height: $size,
                icon: $name,
            }
        }
    }};

    (
        $name:ident,
        $size:expr,
        $color:expr
    ) => {{
        use $crate::Icon;
        use $crate::icons::io_icons::$name;
        rsx! {
            Icon {
                width: $size,
                height: $size,
                icon: $name,
                style: format!("stroke:{};fill:'white';",$color)
            }
        }
    }};

    (
        $name:ident,
        $size:expr,
        $color:expr,
        $bg:expr
    ) => {{
        use $crate::Icon;
        use $crate::icons::io_icons::$name;
        rsx! {
            Icon {
                width: $size,
                height: $size,
                icon: $name,
                style: format!("stroke:{};fill:{};",$color,$bg)
            }
        }
    }};
}
