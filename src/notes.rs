use std::process::Command;

use crate::cli::NoteCreateArgs;
use crate::error::{AppError, Result};

const NOTES_SCRIPT: &str = r#"
on run argv
    set noteTitle to item 1 of argv
    set noteBody to item 2 of argv
    set accountName to item 3 of argv
    set folderName to item 4 of argv

    tell application "Notes"
        if not (exists account accountName) then
            error "APPLEKIT_ACCOUNT_NOT_FOUND"
        end if

        tell account accountName
            if not (exists folder folderName) then
                error "APPLEKIT_FOLDER_NOT_FOUND"
            end if

            tell folder folderName
                make new note with properties {name:noteTitle, body:noteBody}
            end tell
        end tell
    end tell
end run
"#;

pub fn create_note(args: &NoteCreateArgs) -> Result<()> {
    let body = body_with_tags(&args.body, &args.tags);
    let output = run_osascript([
        args.title.as_str(),
        body.as_str(),
        args.account.as_str(),
        args.folder.as_str(),
    ])?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(classify_osascript_error(
        &stderr,
        &args.account,
        &args.folder,
    ))
}

fn body_with_tags(body: &str, tags: &[String]) -> String {
    if tags.is_empty() {
        return body.to_string();
    }

    format!("{body}\n\n{}", hashtag_line(tags))
}

fn hashtag_line(tags: &[String]) -> String {
    tags.iter()
        .map(|tag| format!("#{tag}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn run_osascript<const N: usize>(args: [&str; N]) -> Result<std::process::Output> {
    let mut command = Command::new("/usr/bin/osascript");
    command.arg("-e").arg(NOTES_SCRIPT);
    for arg in args {
        command.arg(arg);
    }
    command.output().map_err(AppError::OsascriptIo)
}

fn classify_osascript_error(stderr: &str, account: &str, folder: &str) -> AppError {
    let normalized = stderr.to_lowercase();

    if stderr.contains("APPLEKIT_NOTES_UNAVAILABLE")
        || normalized.contains("can't get application")
        || normalized.contains("can’t get application")
        || normalized.contains("application isn't running")
        || normalized.contains("application isn’t running")
    {
        AppError::NotesUnavailable
    } else if stderr.contains("APPLEKIT_ACCOUNT_NOT_FOUND") {
        AppError::NotesAccountNotFound {
            account: account.to_string(),
        }
    } else if stderr.contains("APPLEKIT_FOLDER_NOT_FOUND") {
        AppError::NotesFolderNotFound {
            account: account.to_string(),
            folder: folder.to_string(),
        }
    } else if normalized.contains("not authorized")
        || normalized.contains("not permitted")
        || normalized.contains("automation")
        || normalized.contains("privilege")
        || normalized.contains("1743")
    {
        AppError::NotesAutomationDenied
    } else {
        AppError::OsascriptFailed(stderr.trim().to_string())
    }
}

#[cfg(test)]
fn escape_applescript_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    escaped.push('"');
    for ch in value.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }
    escaped.push('"');
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_applescript_strings() {
        assert_eq!(escape_applescript_string("plain"), "\"plain\"");
        assert_eq!(
            escape_applescript_string("quote \" slash \\"),
            "\"quote \\\" slash \\\\\""
        );
        assert_eq!(escape_applescript_string("a\nb\rc\td"), "\"a\\nb\\rc\\td\"");
    }

    #[test]
    fn classifies_known_notes_errors() {
        assert!(matches!(
            classify_osascript_error("APPLEKIT_ACCOUNT_NOT_FOUND", "iCloud", "Notes"),
            AppError::NotesAccountNotFound { .. }
        ));
        assert!(matches!(
            classify_osascript_error("Not authorized to send Apple events", "iCloud", "Notes"),
            AppError::NotesAutomationDenied
        ));
    }

    #[test]
    fn appends_tags_as_hashtags() {
        let tags = vec!["work".to_string(), "urgent".to_string()];
        assert_eq!(body_with_tags("Body", &tags), "Body\n\n#work #urgent");
        assert_eq!(body_with_tags("Body", &[]), "Body");
    }
}
