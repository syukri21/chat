use mail_send::mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;
use persistence::Env;
use std::sync::Arc;

pub struct Mail<'a> {
    mail_config: MailConfig<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailConfig<'a> {
    from: &'a str,
    from_email: &'a str,
    smtp_username: &'a str,
    smtp_password: &'a str,
    smtp_host: &'a str,
    smtp_port: &'a str,
}

impl<'a> MailConfig<'a> {
    pub fn from_env(env: &'a Env) -> MailConfig<'a> {
        Self {
            from: &env.email_from,
            from_email: &env.email_from_email,
            smtp_username: &env.email_smtp_username,
            smtp_password: &env.email_smtp_password,
            smtp_host: &env.email_smtp_host,
            smtp_port: &env.email_smtp_port,
        }
    }
}

impl<'a> Mail<'a> {
    pub fn new(env: &'a Env) -> Self {
        Self {
            mail_config: MailConfig::from_env(&env),
        }
    }

    pub fn new_arc(env: &'a Env) -> Arc<dyn SendEmail + Send + Sync + 'a> {
        Arc::new(Mail::new(env))
    }
}

#[async_trait::async_trait]
pub trait SendEmail {
    async fn send_email(
        &self,
        to: &str,
        to_email: &str,
        subject: &str,
        body: &str,
    ) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl SendEmail for Mail<'_> {
    async fn send_email(
        &self,
        to: &str,
        to_email: &str,
        subject: &str,
        body: &str,
    ) -> anyhow::Result<()> {
        // Build a simple multipart message
        let message = MessageBuilder::new()
            .from((self.mail_config.from, self.mail_config.from_email))
            .to(vec![(to, to_email)])
            .subject(subject)
            .html_body(body)
            .text_body(body);

        // Connect to the SMTP submissions port, upgrade to TLS and
        // authenticate using the provided credentials.
        SmtpClientBuilder::new(
            self.mail_config.smtp_host,
            self.mail_config.smtp_port.parse()?,
        )
        .implicit_tls(false)
        .credentials((
            self.mail_config.smtp_username,
            self.mail_config.smtp_password,
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
    use crate::Mail;
    use crate::SendEmail;
    use persistence::Env;

    #[tokio::test]
    async fn test_send_email() {
        let env = Env {
            db_url: "".to_string(),
            email_from: "admin".to_string(),
            email_from_email: "syukrihsbofficial@gmail.com".to_string(),
            email_smtp_username: "syukrihsbofficial@gmail.com".to_string(),
            email_smtp_password: "admin".to_string(),
            email_smtp_host: "smtp.gmail.com".to_string(),
            email_smtp_port: "587".to_string(),
        };
        let mail = Mail::new(&env);
        let result = mail
            .send_email("test", "syukrihsb148@gmail.com", "test", "test")
            .await;
        assert!(!result.is_ok());
    }
}
