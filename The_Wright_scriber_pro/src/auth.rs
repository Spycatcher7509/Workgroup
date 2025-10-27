use crate::db;
use crate::schema::User;
use argon2::{self, Config};
use rand::Rng;

pub struct AuthService;

impl AuthService {
    pub fn login_user(_username: &str, _password: &str) -> bool {
        // Actual authentication logic will verify user credentials from the database.
        false
    }

    pub fn change_password(_user_id: i64, _new_password: &str) -> Result<(), String> {
        // Actual implementation will update user's password in the database.
        Ok(())
    }

    pub fn hash_password(password: &str) -> String {
        let salt: [u8; 16] = rand::thread_rng().gen();
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
    }

    pub fn verify_password(hash: &str, password: &str) -> bool {
        argon2::verify_encoded(hash, password.as_bytes()).unwrap_or(false)
    }
}
