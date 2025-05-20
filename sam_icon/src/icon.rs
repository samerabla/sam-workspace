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
///     The order of params: [name size color bg]
///     color=stroke | bg=fill
///
///     [Defaults:]
///     size: 30
///     color: "black"
///     fill: "white"
///
///     You can provide onyly name
///     icon!(LdX)
///
///     You can provide name, size
///     icon!(LdX,60)
///
///     You can provide name, size, color
///     icon!(LdX,40,"red")
///
///     You can provide name, size, color, bg
///     icon!(LdX,60,"white","black")
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
