// This is free and unencumbered software released into the public domain.

use know::classes::EmailMessage;
use maildir::{MailEntry, MailEntryError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaildirMessage {
    pub id: String,
    pub flags: String,
    pub message: EmailMessage,
}

impl TryFrom<MailEntry> for MaildirMessage {
    type Error = MailEntryError;

    fn try_from(mut input: MailEntry) -> Result<Self, Self::Error> {
        Ok(Self {
            id: input.id().to_string(),
            flags: input.flags().to_string(),
            message: (&mut input).try_into()?,
        })
    }
}
