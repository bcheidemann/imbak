use std::{io::Write, path::Path};

use anyhow::Context;
use secrecy::Secret;

use crate::{
    simple_imap::{ImapClient, ImapClientCredentials, ImapClientOptions, ImapMailboxName},
    utils::prompt,
};

use super::BackupArgs;

pub fn backup(args: &BackupArgs) -> anyhow::Result<()> {
    let options = ImapClientOptions::<&str> {
        domain: args.domain.as_ref(),
        port: args.port,
    };

    let credentials = ImapClientCredentials {
        username: Secret::new(
            std::env::var("IMAP_USERNAME")
                .or_else(|e| match e {
                    std::env::VarError::NotPresent => prompt("Enter the IMAP username", false),
                    std::env::VarError::NotUnicode(_) => {
                        anyhow::bail!("IMAP_USERNAME is not valid unicode")
                    }
                })
                .context("Failed to determine IMAP username")?
                .trim()
                .to_string(),
        ),
        password: Secret::new(
            std::env::var("IMAP_PASSWORD")
                .or_else(|e| match e {
                    std::env::VarError::NotPresent => prompt("Enter the IMAP password", true),
                    std::env::VarError::NotUnicode(_) => {
                        anyhow::bail!("IMAP_PASSWORD is not valid unicode")
                    }
                })
                .context("Failed to determine IMAP password")?
                .trim()
                .to_string(),
        ),
    };

    let mailbox_name = args
        .mailbox
        .as_ref()
        .map(ImapMailboxName::Other)
        .unwrap_or(ImapMailboxName::Inbox);

    let mut mailbox = ImapClient::from_options(&options)?
        .login(&credentials)?
        .select_mailbox(mailbox_name)?;

    let message_count = mailbox.message_count();
    let last_message = match args.count {
        Some(count) => std::cmp::min(args.from + count, message_count),
        None => message_count,
    };

    for from in (args.from..)
        .take_while(|i| *i <= last_message)
        .step_by(args.batch_size)
    {
        let to = std::cmp::min(from + args.batch_size as u32 - 1, last_message - 1);
        println!(
            "Fetching messages {} to {} of {} in mailbox... ",
            from + 1,
            to + 1,
            message_count
        );
        let messages = mailbox
            .fetch_message_range(from, to)
            .with_context(|| format!("Failed to fetch message batch ({} to {})", from, to))?;

        let directory = match args.output_directory {
            Some(ref directory) => directory.clone(),
            None => std::env::current_dir().context("Failed to determine current directory")?,
        };

        messages
            .iter()
            .enumerate()
            .filter_map(|(idx, message)| Some((from + idx as u32 + 1, message.body()?)))
            .try_for_each(|(uid, body)| save_message(&directory, uid, body))?;

        println!("Saved {} messages", messages.len());
    }

    Ok(())
}

fn save_message(dir: &Path, uid: u32, body: &[u8]) -> anyhow::Result<()> {
    let path = dir.join(format!("{}.eml", uid));
    println!("Saving message {} to {}... ", uid, path.display());
    let mut file = std::fs::File::create(path).context("Failed to create file")?;
    file.write(body).context("Failed to write to file")?;
    Ok(())
}
