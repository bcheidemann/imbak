use std::net::TcpStream;

use anyhow::Context;
use imap::Session;
use native_tls::TlsStream;

use super::{ImapMailbox, ImapMailboxName};

pub struct ImapSession {
    pub(crate) session: Session<TlsStream<TcpStream>>,
}

impl ImapSession {
    pub fn select_mailbox<S: AsRef<str>>(
        mut self,
        mailbox_name: ImapMailboxName<S>,
    ) -> anyhow::Result<ImapMailbox<S>> {
        let mailbox = self
            .session
            .select(mailbox_name.as_ref())
            .context("Failed to select mailbox")?;

        Ok(ImapMailbox {
            session: self,
            mailbox,
            name: mailbox_name,
        })
    }
}
