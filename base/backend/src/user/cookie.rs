use axum_extra::extract::cookie::Cookie;
use time::Duration;

pub fn create_cookie(key: String, value: String, seconds: i64) -> Cookie<'static> {
    let cookie: Cookie<'static> = Cookie::build((key, value))
        .http_only(true)
        .secure(true) // Ensure the cookie is only sent over HTTPS
        //.secure(false) // Set to true if using HTTPS
        // .same_site(axum_extra::extract::cookie::SameSite::Strict)
        .same_site(axum_extra::extract::cookie::SameSite::None)
        .path("/")
        .max_age(Duration::seconds(seconds))
        .build();
    cookie
}

// pub fn create_cookie(key: String, value: String, seconds: i64) -> Cookie<'static> {
//     let cookie: Cookie<'static> = Cookie::build((key, value))
//         .http_only(true)
//         .secure(false) // Set to true if using HTTPS
//         // .same_site(axum_extra::extract::cookie::SameSite::Strict)
//         .same_site(axum_extra::extract::cookie::SameSite::Lax)
//         .path("/")
//         .max_age(Duration::seconds(seconds))
//         .build();
//     cookie
// }
