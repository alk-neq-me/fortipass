#[derive(Debug)]
pub struct Password {
    pub site: String,
    pub username: String,
    pub password: String
}

impl Password {
    pub fn new(site: &str, username: &str, password: &str) -> Password {
        Password { site: site.to_string(), username: username.to_string(), password: password.to_string() }
    }
}
