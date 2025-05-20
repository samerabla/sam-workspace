pub const BASE_URL: &str = if cfg!(debug_assertions) {
    "http://127.0.0.1:3000"
} else {
    "https://some-domain"
};
