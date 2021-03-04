pub mod db;
pub mod errors;

use rocket::http::{Cookie, Cookies, SameSite};

pub fn generate_id() -> String {
    use nanoid::nanoid;

    nanoid!()
}

pub fn get_session_id(cookies: &mut Cookies) -> String {
    match cookies.get_private("session") {
        Some(id) => id.value().to_string(),
        None => {
            let user_id = generate_id();

            let cookie = Cookie::build("session", user_id.clone())
                .domain("rustebin.com")
                .same_site(SameSite::Lax)
                .secure(true)
                .permanent()
                .finish();

            cookies.add_private(cookie);

            user_id
        }
    }
}

pub fn is_url(text: String) -> bool {
    use regex::Regex;

    let re = Regex::new("^(https?://)?((([a-z\\d]([a-z\\d-]*[a-z\\d])*)\\.)+[a-z]{2,}|((\\d{1,3}\\.){3}\\d{1,3}))(:\\d+)?(/[-a-z\\d%_.~+]*)*(\\?[;&a-z\\d%_.~+=-]*)?(#[-a-z\\d_]*)?$").unwrap();
    re.is_match(&text)
}
