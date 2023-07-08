use anyhow::Context;
use imap::types::{Fetch, Mailbox, ZeroCopy};

use super::{ImapMailboxName, ImapSession};

pub struct ImapMailbox<S: AsRef<str>> {
    pub(crate) session: ImapSession,
    pub(crate) mailbox: Mailbox,

    pub name: ImapMailboxName<S>,
}

impl<S: AsRef<str>> ImapMailbox<S> {
    pub fn message_count(&self) -> u32 {
        self.mailbox.exists
    }

    /// Fetches a range of messages from the mailbox. The range is inclusive and starts at 0.
    pub fn fetch_message_range(
        &mut self,
        from: u32,
        to: u32,
    ) -> anyhow::Result<ZeroCopy<Vec<Fetch>>> {
        // IMAP message numbers start at 1
        let from = from + 1;
        let to = to + 1;

        self.session
            .session
            .fetch(&format!("{}:{}", from, to), "RFC822")
            .with_context(|| format!("Failed to fetch messages in range {} to {}", from, to))
    }
}
