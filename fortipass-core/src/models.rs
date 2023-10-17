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


#[derive(Debug)]
pub struct EncryptedPassword {
    pub site: String,
    pub username: Vec<u8>,
    pub password: Vec<u8>
}

impl EncryptedPassword {
    pub fn new(site: &str, username: Vec<u8>, password: Vec<u8>) -> EncryptedPassword {
        EncryptedPassword { site: site.to_string(), username, password, }
    }
}

