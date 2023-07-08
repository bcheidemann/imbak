use clap::Parser;
use cli::{backup, CliArgs};

mod cli;
mod simple_imap;
mod utils;

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    match args.subcommand {
        cli::SubcommandArgs::Backup(backup_args) => backup(&backup_args)?,
    }

    Ok(())
}
