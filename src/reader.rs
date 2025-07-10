// This is free and unencumbered software released into the public domain.

use super::{MaildirError, MaildirIterator, MaildirMessage};
use know::datatypes::EmailMessageId;
use maildir::Maildir;
use std::{io, path::Path};

pub struct MaildirReader {
    maildir: Maildir,
}

impl MaildirReader {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            maildir: Maildir::from(path.as_ref().to_owned()),
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = Result<MaildirMessage, MaildirError>> {
        MaildirIterator::new(self.maildir.list_new())
            .chain(MaildirIterator::new(self.maildir.list_cur()))
    }

    pub fn fetch(
        &self,
        message_id: &EmailMessageId,
    ) -> Result<Option<MaildirMessage>, MaildirError> {
        for entry in self.iter() {
            let message = entry?;
            if let Some(mid) = message.headers.id.as_ref() {
                if mid == message_id {
                    return Ok(Some(message));
                }
            }
        }
        Ok(None)
    }
}
