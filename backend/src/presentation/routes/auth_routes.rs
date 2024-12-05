use actix_web::web::{get, post, scope};
use actix_web::Scope;

use crate::presentation::handlers::auth_handlers::{current_user, guest_login, login, signup};

pub fn auth_scope() -> Scope {
    scope("/auth")
        .route("/guest_login", post().to(guest_login))
        .route("/signup", post().to(signup))
        .route("/login", post().to(login))
        .route("/current_user", get().to(current_user))
        // .route("/verify_email", get().to(auth::verify_email))
}
