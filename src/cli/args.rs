use clap::{Parser, Subcommand};

use super::BackupArgs;

#[derive(Debug, Parser)]
#[command(author, version)]
#[command(
    about = "imbak - A command line tool for backing up IMAP mailboxes",
    long_about = "A command line tool for backing up IMAP mailboxes"
)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub subcommand: SubcommandArgs,
}

#[derive(Debug, Subcommand)]
pub enum SubcommandArgs {
    /// Backup a mailbox
    Backup(BackupArgs),
}
