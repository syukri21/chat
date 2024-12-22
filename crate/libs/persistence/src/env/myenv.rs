use std::env;

pub struct Env {
    pub db_url: String,
    pub email_from: String,
    pub email_from_email: String,
    pub email_smtp_username: String,
    pub email_smtp_password: String,
    pub email_smtp_host: String,
    pub email_smtp_port: String,
    pub app_key_main: String,
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

impl Env {
    pub fn new() -> Self {
        let environment_variable = Self {
            db_url: env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string()),
            email_from: env::var("EMAIL_FROM").unwrap_or_else(|_| "".to_string()),
            email_from_email: env::var("EMAIL_FROM_EMAIL").unwrap_or_else(|_| "".to_string()),
            email_smtp_username: env::var("EMAIL_SMTP_USERNAME").unwrap_or_else(|_| "".to_string()),
            email_smtp_password: env::var("EMAIL_SMTP_PASSWORD").unwrap_or_else(|_| "".to_string()),
            email_smtp_host: env::var("EMAIL_SMTP_HOST").unwrap_or_else(|_| "".to_string()),
            email_smtp_port: env::var("EMAIL_SMTP_PORT").unwrap_or_else(|_| "".to_string()),
            app_key_main: env::var("APP_KEY_MAIN").unwrap_or_else(|_| "".to_string()),
        };
        environment_variable.validate();
        environment_variable
    }

    fn validate(&self) {
        if self.db_url.is_empty() {
            panic!("Database URL is empty");
        }

        // validate email
        if self.email_from.is_empty() {
            panic!("Email is not valid");
        }
        if self.email_from_email.is_empty() {
            panic!("Email from email is empty");
        }
        if self.email_smtp_username.is_empty() {
            panic!("Email smtp username is empty");
        }
        if self.email_smtp_password.is_empty() {
            panic!("Email smtp password is empty");
        }
        if self.email_smtp_host.is_empty() {
            panic!("Email smtp host is empty");
        }
        if self.email_smtp_port.is_empty() {
            panic!("Email smtp port is empty");
        }

        // validate crypto
        if self.app_key_main.is_empty() {
            panic!("App main key is empty");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_env_new() {
        dotenv::dotenv().ok();
        let env = Env::new();
        assert!(!env.db_url.is_empty());
        assert!(!env.email_from.is_empty());
        assert!(!env.email_from_email.is_empty());
        assert!(!env.email_smtp_username.is_empty());
        assert!(!env.email_smtp_password.is_empty());
        assert!(!env.email_smtp_host.is_empty());
        assert!(!env.email_smtp_port.is_empty());
        assert!(!env.app_key_main.is_empty());
    }
    #[test]
    #[should_panic]
    fn test_env_new_empty_db_url() {
        let env = Env {
            db_url: "".to_string(),
            email_from: "".to_string(),
            email_from_email: "".to_string(),
            email_smtp_username: "".to_string(),
            email_smtp_password: "".to_string(),
            email_smtp_host: "".to_string(),
            email_smtp_port: "".to_string(),
            app_key_main: "".to_string(),
        };
        env.validate();
    }
}
