use serde_json::Value;

use {MailingListID, TemplateID};

/// The body of a POST to `/api/mail/enqueue`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MailEnqueueRequest {
    /// The ID of the mailing list as an integer.
    pub mailing_list: MailingListID,

    /// The ID of the template as an integer.
    pub template: TemplateID,

    /// The data to render into the template as a JSON value.
    pub data: Value,

    /// The email address to send to as a string.
    pub email: String,

    /// The subject line of the email as a string.
    pub subject: String,
}
