/// ## Generate a random string
///
/// ### This macro has 3 options:
///
/// ```
/// gen_id() // defult len = 8
/// gen_id(16) // specified length
/// gen_id(10, "input_") // specified length with prefix
/// ```
///
#[macro_export]
macro_rules! gen_id {
    () => {{
        use $crate::Rng;
        let mut rng = $crate::thread_rng();
        (0..8)
            .map(|_| rng.sample($crate::Alphanumeric) as char)
            .collect::<String>()
    }};

    ($len:expr) => {{
        use $crate::Rng;
        let mut rng = $crate::thread_rng();
        (0..$len)
            .map(|_| rng.sample($crate::Alphanumeric) as char)
            .collect::<String>()
    }};

    ($len:expr, $prefix:expr) => {{
        use $crate::Rng;
        let mut rng = $crate::thread_rng();
        let chars = (0..$len)
            .map(|_| rng.sample($crate::Alphanumeric) as char)
            .collect::<String>();
        format!("{}{}", $prefix, chars)
    }};
}
