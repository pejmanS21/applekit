use std::env;
use std::path::PathBuf;
use std::process::Command;

use crate::cli::ReminderCreateArgs;
use crate::error::{AppError, Result};

pub fn create_reminder(args: &ReminderCreateArgs) -> Result<()> {
    let helper = find_helper().ok_or(AppError::ReminderHelperNotFound)?;
    let output = build_helper_command(&helper, args)
        .output()
        .map_err(|source| AppError::ReminderHelperIo {
            path: helper.clone(),
            source,
        })?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(classify_helper_error(&stderr))
}

fn build_helper_command(helper: &PathBuf, args: &ReminderCreateArgs) -> Command {
    let mut command = Command::new(helper);
    command.arg("--title").arg(&args.title);

    if let Some(due) = &args.due {
        command.arg("--due").arg(due);
    }
    if let Some(notes) = &args.notes {
        command.arg("--notes").arg(notes);
    }
    if let Some(list) = &args.list {
        command.arg("--list").arg(list);
    }
    if let Some(priority) = args.priority {
        command.arg("--priority").arg(priority.to_string());
    }

    command
}

fn find_helper() -> Option<PathBuf> {
    if let Some(path) = env::var_os("APPLEKIT_REMINDER_HELPER") {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Some(path);
        }
    }

    let cwd_helper = PathBuf::from("target/helper/ReminderHelper");
    if cwd_helper.is_file() {
        return Some(cwd_helper);
    }

    if let Ok(exe) = env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            let sibling_target_helper = exe_dir.join("target/helper/ReminderHelper");
            if sibling_target_helper.is_file() {
                return Some(sibling_target_helper);
            }

            let sibling_helper = exe_dir.join("ReminderHelper");
            if sibling_helper.is_file() {
                return Some(sibling_helper);
            }
        }
    }

    None
}

fn classify_helper_error(stderr: &str) -> AppError {
    let trimmed = stderr.trim();

    if trimmed.contains("APPLEKIT_REMINDERS_ACCESS_DENIED") {
        AppError::RemindersAccessDenied
    } else if let Some(list) = trimmed.strip_prefix("APPLEKIT_LIST_NOT_FOUND:") {
        AppError::ReminderListNotFound(list.trim().to_string())
    } else if trimmed.contains("APPLEKIT_INVALID_DATE") {
        AppError::InvalidReminderDate
    } else if trimmed.is_empty() {
        AppError::ReminderHelperFailed("helper exited with no error output".to_string())
    } else {
        AppError::ReminderHelperFailed(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::ReminderCreateArgs;

    #[test]
    fn classifies_helper_errors() {
        assert!(matches!(
            classify_helper_error("APPLEKIT_REMINDERS_ACCESS_DENIED"),
            AppError::RemindersAccessDenied
        ));
        assert!(matches!(
            classify_helper_error("APPLEKIT_LIST_NOT_FOUND: Work"),
            AppError::ReminderListNotFound(list) if list == "Work"
        ));
        assert!(matches!(
            classify_helper_error("APPLEKIT_INVALID_DATE"),
            AppError::InvalidReminderDate
        ));
    }

    #[test]
    fn builds_helper_command_arguments() {
        let args = ReminderCreateArgs {
            title: "Submit report".to_string(),
            due: Some("2026-05-05 17:30".to_string()),
            notes: Some("Attach final PDF".to_string()),
            list: Some("Work".to_string()),
            priority: Some(5),
        };

        let command = build_helper_command(&PathBuf::from("/tmp/ReminderHelper"), &args);
        let rendered: Vec<String> = command
            .get_args()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect();

        assert_eq!(
            rendered,
            vec![
                "--title",
                "Submit report",
                "--due",
                "2026-05-05 17:30",
                "--notes",
                "Attach final PDF",
                "--list",
                "Work",
                "--priority",
                "5"
            ]
        );
    }
}
