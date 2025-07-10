// This is free and unencumbered software released into the public domain.

use super::MaildirError;
use know::classes::EmailMessage;
use mail_parser::MessageParser;
use maildir::MailEntry;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaildirMessage {
    pub id: String,
    pub flags: String,
    pub headers: EmailMessage,
    pub body: Option<String>,
}

impl TryFrom<MailEntry> for MaildirMessage {
    type Error = MaildirError;

    fn try_from(mut input: MailEntry) -> Result<Self, Self::Error> {
        let id = input.id().to_string();
        let flags = input.flags().to_string();
        let bytes = input.parsed()?.raw_bytes;
        let message = MessageParser::default()
            .parse(bytes)
            .ok_or(MaildirError::InvalidMessage)?;

        Ok(Self {
            id,
            flags,
            headers: (&message)
                .try_into()
                .map_err(|_| MaildirError::InvalidHeaders)?,
            body: message.body_text(0).map(|s| s.into_owned()),
        })
    }
}
