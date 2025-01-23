use mail_send::mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;
use persistence::env::myenv::EnvInterface;
use shaku::{Component, Interface};
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = SendEmail)]
pub struct Mail {
    #[shaku(inject)]
    env: Arc<dyn EnvInterface>,
}

impl Mail {
    pub fn new(env: Arc<dyn EnvInterface>) -> Self {
        Self { env }
    }

    pub fn new_arc(env: Arc<dyn EnvInterface>) -> Arc<dyn SendEmail> {
        Arc::new(Mail::new(env))
    }
}

#[async_trait::async_trait]
pub trait SendEmail: Interface {
    async fn send_email(
        &self,
        to: &str,
        to_email: &str,
        subject: &str,
        body: &str,
    ) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl SendEmail for Mail {
    async fn send_email(
        &self,
        to: &str,
        to_email: &str,
        subject: &str,
        body: &str,
    ) -> anyhow::Result<()> {
        // Build a simple multipart message
        let message = MessageBuilder::new()
            .from((self.env.get_email_from(), self.env.get_email_from_email()))
            .to(vec![(to, to_email)])
            .subject(subject)
            .html_body(body)
            .text_body(body);

        // Connect to the SMTP submissions port, upgrade to TLS and
        // authenticate using the provided credentials.
        SmtpClientBuilder::new(
            self.env.get_email_smtp_host(),
            self.env.get_email_smtp_port().parse()?,
        )
        .implicit_tls(false)
        .credentials((
            self.env.get_email_smtp_username(),
            self.env.get_email_smtp_password(),
        ))
        .connect()
        .await?
        .send(message)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::Mail;
    use crate::SendEmail;
    use persistence::Env;
    use persistence::env::myenv::EnvInterface;

    #[tokio::test]
    async fn test_send_email() {
        let env: Arc<dyn EnvInterface> = Arc::new(Env {
            db_url: "".to_string(),
            email_from: "admin".to_string(),
            email_from_email: "syukrihsbofficial@gmail.com".to_string(),
            email_smtp_username: "syukrihsbofficial@gmail.com".to_string(),
            email_smtp_password: "admin".to_string(),
            email_smtp_host: "smtp.gmail.com".to_string(),
            email_smtp_port: "587".to_string(),
            app_key_main: "testkey".to_string(),
            app_callback_url: "".to_string(),
            app_key_jwt: "".to_string(),
        });
        let mail = Mail::new(env);
        let result = mail
            .send_email("test", "syukrihsb148@gmail.com", "test", "test")
            .await;
        assert!(!result.is_ok());
    }
}
