use crate::model::GateKeeperModel;
use crate::tokens::Token;
use crate::verification::message::{Mail, Mailer};
use crate::GateKeeperResult;

pub struct UserVerificationMail {
    from: Option<String>,
    to: String,
    to_name: String,
    subject: Option<String>,
    html_body: Option<String>,
    text_body: Option<String>,
}

#[cfg(not(test))]
impl Default for UserVerificationMail {
    fn default() -> Self {
        Self {
            from: Some("ShaZone <no-reply@sha.zone>".to_string()),
            to: "".to_string(),
            to_name: "".to_string(),
            subject: Some("Verify your email address".to_string()),
            html_body: None,
            text_body: None,
        }
    }
}

#[cfg(test)]
impl Default for UserVerificationMail {
    fn default() -> Self {
        Self {
            from: Some("no-reply@resend.dev".to_string()),
            to: "".to_string(),
            to_name: "".to_string(),
            subject: Some("Verify your email address".to_string()),
            html_body: None,
            text_body: None,
        }
    }
}

impl UserVerificationMail {
    pub fn generate_html_body(link: &str) -> GateKeeperResult<String> {
        Ok(format!(
            "<p>Hi!</p>
        <p>Please click on the following link to verify your email address:</p>
        <p><a href=\"{link}\">{link}</a></p>
        <p>If you did not request this, please ignore this email.</p>",
        ))
    }

    pub fn generate_text_body(link: &str) -> GateKeeperResult<String> {
        Ok(format!(
            "Hi!
            Please click on the following link to verify your email address:
            \"{link}\"

            If you did not request this, please ignore this email."
        ))
    }
}

impl Mail for UserVerificationMail {
    fn try_from_user<M: GateKeeperModel>(user: M) -> GateKeeperResult<Self>
    where
        Self: Sized,
    {
        let link_prefix = std::env::var("VERIFICATION_LINK_PREFIX")?;
        let link = format!(
            "{link_prefix}{}",
            user.verification_token().unwrap().get_encoded()
        );

        Ok(Self {
            to: user.email().to_string(),
            to_name: user.first_name().to_string(),
            html_body: Some(UserVerificationMail::generate_html_body(link.as_str())?),
            text_body: Some(UserVerificationMail::generate_text_body(link.as_str())?),
            ..Default::default()
        })
    }

    fn email_from(&self) -> &str {
        self.from.as_ref().unwrap()
    }

    fn email_to_name(&self) -> &str {
        self.to_name.as_str()
    }

    fn email_to(&self) -> &str {
        self.to.as_str()
    }

    fn email_subject(&self) -> &str {
        self.subject.as_ref().unwrap()
    }

    fn email_html_body(&self) -> &str {
        self.html_body.as_ref().unwrap()
    }

    fn email_text_body(&self) -> &str {
        self.text_body.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{Claims, VerificationToken};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    struct TestUser;

    impl GateKeeperModel for TestUser {
        fn id(&self) -> Uuid {
            uuid::Uuid::new_v4()
        }

        fn secret(&self) -> &'static str {
            "some-awesome-secret"
        }

        fn first_name(&self) -> &str {
            "Hugo"
        }

        fn last_name(&self) -> &str {
            "Boss"
        }

        fn email(&self) -> &str {
            "hugo.boss@gmail.com"
        }

        fn verification_token(&self) -> Option<VerificationToken> {
            let claims = Claims {
                exp: 1,
                iat: 1,
                sub: self.id().to_string(),
            };
            let token = VerificationToken::encode(&claims, self.secret().to_string()).unwrap();
            let token = VerificationToken::new(token, claims);

            Some(token)
        }
    }

    #[test]
    fn test_generate_body() {
        let link = "http://localhost:8000/verify/1234567890";
        let body_html = UserVerificationMail::generate_html_body(link).unwrap();
        let body_text = UserVerificationMail::generate_text_body(link).unwrap();

        assert!(body_html.contains(link));
        assert!(body_text.contains(link));
    }

    #[tokio::test]
    async fn test_send_verification_mail() -> anyhow::Result<()> {
        std::env::set_var(
            "VERIFICATION_LINK_PREFIX",
            "http://localhost:8000/v1/auth/verify?token=",
        );

        let user = TestUser::default();
        let mail: UserVerificationMail = UserVerificationMail::try_from_user(user)?;
        let mailer = Mailer::try_new()?;

        match mailer.send::<UserVerificationMail>(mail).await {
            Ok(_) => (),
            Err(e) => panic!("Error sending mail: {}", e),
        }

        Ok(())
    }
}
