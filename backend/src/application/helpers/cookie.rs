use actix_web::cookie::{time, Cookie, SameSite};

pub fn create_cookie(token: String) -> Cookie<'static> {
    Cookie::build("token", token)
        .path("/")
        .http_only(true)
        .same_site(SameSite::None)
        .secure(true)
        .max_age(time::Duration::days(30))
        .finish()
}

pub fn clear_cookie() -> Cookie<'static> {
    Cookie::build("token", "")
        .path("/")
        .http_only(true)
        .same_site(SameSite::None)
        .secure(true)
        .max_age(time::Duration::seconds(0))
        .finish()
}