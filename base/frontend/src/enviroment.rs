pub const BASE_URL: &str = if cfg!(debug_assertions) {
    "https://localhost:3000"
    // "https://127.0.0.1:3000"
    // "http://127.0.0.1:3000"
} else {
    "https://some-domain"
};
