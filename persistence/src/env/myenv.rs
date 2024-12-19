use std::env;

pub struct Env {
    pub db_url: String,
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
        };
        environment_variable.validate();
        environment_variable
    }

    fn validate(&self)  {
        if self.db_url.is_empty() {
            panic!("Database URL is empty");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_env_new() {
        let env = Env::new();
        assert!(!env.db_url.is_empty());
    }
    #[test]
    #[should_panic]
    fn test_env_new_empty_db_url() {
        let env = Env { db_url: "".to_string() };
        env.validate();
    }
}