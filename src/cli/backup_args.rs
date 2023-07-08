use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Args)]
pub struct BackupArgs {
    #[arg(short, long)]
    /// The domain of the IMAP server
    pub domain: String,

    #[arg(short, long)]
    /// The port of the IMAP server
    pub port: u16,

    #[arg(short, long)]
    /// The mailbox to backup (defaults to the INBOX)
    pub mailbox: Option<String>,

    #[arg(short, long, default_value = "5")]
    /// The number of emails to download per batch
    pub batch_size: usize,

    #[arg(short, long)]
    /// The directory to store the backups in (defaults to the current directory)
    pub output_directory: Option<PathBuf>,

    #[arg(short, long, default_value = "0")]
    /// The message number to start backing up from
    pub from: u32,

    #[arg(short, long)]
    /// The number of messages to backup (defaults to all messages)
    pub count: Option<u32>,
}
