//! # Mailer provides traits for sending E-Mails
//!
//! Use it for sending user verification emails

use crate::model::GateKeeperModel;
use crate::GateKeeperResult;

/// `Mailer` provides a email sending client
pub trait Mailer {
    /// Try creating a new mailer instance
    fn try_new() -> GateKeeperResult<Self>
    where
        Self: Sized;

    /// Actually send the email
    async fn send<M: Mail>(&self, mail: M) -> GateKeeperResult<()>;
}

/// Trait `Mail` represents an instance of an actual email to be sent
pub trait Mail {
    /// ## Create new mail from given user
    ///
    /// ### Parameters:
    /// - ***user*** The model instance to read necessary values from
    fn try_from_user<M: GateKeeperModel>(user: M) -> GateKeeperResult<Self>
    where
        Self: Sized;

    /// Sender email address
    fn email_from(&self) -> &str;

    /// Receiver name
    fn email_to_name(&self) -> &str;

    /// Receiver email address
    fn email_to(&self) -> &str;

    /// Mail subject
    fn email_subject(&self) -> &str;

    /// Mail HTML body
    fn email_html_body(&self) -> &str;

    /// Mail TEXT body
    fn email_text_body(&self) -> &str;
}
