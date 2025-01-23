use axum::response::Html;
use axum::Form;
use serde::Deserialize;
use usecases::RegisterRequest;

#[derive(Deserialize)]
pub struct RegisterForm {
    username: String,
    password: String,
    email: String,
    private_key: String,
    public_key: String,
}

impl RegisterForm {
    pub fn to_register_request(&self) -> RegisterRequest {
        RegisterRequest {
            username: &self.username,
            email: &self.email,
            password: &self.password,
            private_key: &self.private_key,
            public_key: &self.public_key,
        }
    }
}
pub async fn register(Form(form): Form<RegisterForm>) -> Html<&'static str> {
    tracing::info!("Htmx register Started with username: {}", form.username);
    Html(include_str!("../page/htmx/signup_success.html"))
}
