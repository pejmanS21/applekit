use chrono::NaiveDateTime;
use clap::{Args, Parser, Subcommand};

const DUE_FORMAT: &str = "%Y-%m-%d %H:%M";

#[derive(Debug, Parser)]
#[command(name = "applekit")]
#[command(about = "Create Apple Notes and Apple Reminders from the command line.")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CommandGroup,
}

#[derive(Debug, Subcommand)]
pub enum CommandGroup {
    #[command(subcommand)]
    Note(NoteCommand),
    #[command(subcommand)]
    Reminder(ReminderCommand),
}

#[derive(Debug, Subcommand)]
pub enum NoteCommand {
    #[command(about = "Create a note in Apple Notes.")]
    Create(NoteCreateArgs),
}

#[derive(Debug, Args)]
pub struct NoteCreateArgs {
    #[arg(short, long, help = "Note title.", value_parser = non_empty_string)]
    pub title: String,
    #[arg(short, long, help = "Plain text note body.", value_parser = non_empty_string)]
    pub body: String,
    #[arg(long, default_value = "iCloud", help = "Notes account name.", value_parser = non_empty_string)]
    pub account: String,
    #[arg(long, visible_alias = "list", default_value = "Notes", help = "Folder/list name inside the selected account.", value_parser = non_empty_string)]
    pub folder: String,
    #[arg(long = "tags", value_name = "TAG", value_delimiter = ',', help = "Comma-separated or repeated tags. A leading # is optional.", value_parser = tag_value)]
    pub tags: Vec<String>,
}

#[derive(Debug, Subcommand)]
pub enum ReminderCommand {
    #[command(about = "Create a reminder in Apple Reminders.")]
    Create(ReminderCreateArgs),
}

#[derive(Debug, Args)]
pub struct ReminderCreateArgs {
    #[arg(short, long, help = "Reminder title.", value_parser = non_empty_string)]
    pub title: String,
    #[arg(short, long, help = "Local due datetime in YYYY-MM-DD HH:MM format.", value_parser = validate_due)]
    pub due: Option<String>,
    #[arg(short, long, help = "Reminder notes/body.", value_parser = non_empty_string)]
    pub notes: Option<String>,
    #[arg(short = 'l', long = "list", help = "Reminder list name. Uses the default list when omitted.", value_parser = non_empty_string)]
    pub list: Option<String>,
    #[arg(long = "tags", value_name = "TAG", value_delimiter = ',', help = "Comma-separated or repeated tags. A leading # is optional.", value_parser = tag_value)]
    pub tags: Vec<String>,
    #[arg(long, help = "EventKit priority, from 0 to 9.", value_parser = clap::value_parser!(u8).range(0..=9))]
    pub priority: Option<u8>,
}

fn non_empty_string(value: &str) -> std::result::Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err("value must not be empty".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

fn validate_due(value: &str) -> std::result::Result<String, String> {
    NaiveDateTime::parse_from_str(value, DUE_FORMAT)
        .map(|_| value.to_string())
        .map_err(|_| "invalid due datetime; expected format YYYY-MM-DD HH:MM".to_string())
}

fn tag_value(value: &str) -> std::result::Result<String, String> {
    let trimmed = value.trim().trim_start_matches('#');
    if trimmed.is_empty() {
        return Err("tag must not be empty".to_string());
    }

    if trimmed
        .chars()
        .any(|ch| ch.is_whitespace() || ch.is_control())
    {
        return Err("tag must not contain whitespace".to_string());
    }

    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_title_values() {
        assert!(non_empty_string("   ").is_err());
    }

    #[test]
    fn validates_due_format() {
        assert!(validate_due("2026-05-04 09:00").is_ok());
        assert!(validate_due("2026-05-04T09:00:00Z").is_err());
    }

    #[test]
    fn normalizes_tags() {
        assert_eq!(tag_value("#work").unwrap(), "work");
        assert_eq!(tag_value(" urgent ").unwrap(), "urgent");
        assert!(tag_value("two words").is_err());
        assert!(tag_value("#").is_err());
    }
}
