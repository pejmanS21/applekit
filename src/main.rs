mod cli;
mod error;
mod notes;
mod reminders;

use clap::Parser;

use crate::cli::{Cli, CommandGroup, NoteCommand, ReminderCommand};
use crate::error::Result;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(error.exit_code());
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CommandGroup::Note(command) => match command {
            NoteCommand::Create(args) => {
                notes::create_note(&args)?;
                println!(
                    "Created note \"{}\" in folder \"{}\" of account \"{}\".",
                    args.title, args.folder, args.account
                );
            }
        },
        CommandGroup::Reminder(command) => match command {
            ReminderCommand::Create(args) => {
                reminders::create_reminder(&args)?;
                println!("Created reminder \"{}\".", args.title);
            }
        },
    }

    Ok(())
}
