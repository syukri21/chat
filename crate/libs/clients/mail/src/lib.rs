use mail_send::mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;
use persistence::env::myenv::EnvInterface;
use shaku::{Component, Interface};
use std::sync::Arc;
use tracing::log::{error, info};

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
        let port = self
            .env
            .get_email_smtp_port()
            .parse()
            .expect("invalid port");

        let host = self.env.get_email_smtp_host();

        info!("Sending email to {}...", to);
        tracing::info!("Initializing email...");
        tokio::spawn({
            let host = host.to_string();
            let port = port; // Assuming `port` is already a copyable type like `u16`
            let smtp_username = self.env.get_email_smtp_username().to_string();
            let smtp_password = self.env.get_email_smtp_password().to_string();
            let subject = subject.to_string(); // Clone the message if it's not `Copy`
            let body = body.to_string();

            let email_from = self.env.get_email_from().to_string();
            let email_from_email = self.env.get_email_from_email().to_string();
            let to = to.to_string();
            let to_email = to_email.to_string();

            async move {
                info!("Sending email...");

                // Connect to the SMTP server and authenticate
                let message = MessageBuilder::new()
                    .from((email_from, email_from_email))
                    .to(vec![(to, to_email)])
                    .subject(subject)
                    .html_body(body.clone())
                    .text_body(body);

                match SmtpClientBuilder::new(host, port)
                    .implicit_tls(false)
                    .credentials((smtp_username, smtp_password))
                    .connect()
                    .await
                {
                    Ok(mut client) => {
                        if let Err(e) = client.send(message).await {
                            error!("Error sending email {}", e);
                        } else {
                            info!("Email sent successfully");
                        }
                    }
                    Err(e) => {
                        error!("Error connecting to SMTP server: {}", e);
                    }
                }
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Mail;
    use crate::SendEmail;
    use persistence::env::myenv::EnvInterface;
    use persistence::Env;
    use std::sync::Arc;

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
        assert!(result.is_ok());
    }
}
