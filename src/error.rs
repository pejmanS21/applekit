use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(
        "Notes.app is not available or could not be started. Confirm Notes is installed and try opening it once before running applekit."
    )]
    NotesUnavailable,

    #[error(
        "Notes Automation permission was denied. Allow your terminal app to control Notes in System Settings > Privacy & Security > Automation."
    )]
    NotesAutomationDenied,

    #[error("Notes account \"{account}\" was not found. Check the account name in Notes.app.")]
    NotesAccountNotFound { account: String },

    #[error("Notes folder \"{folder}\" was not found in account \"{account}\".")]
    NotesFolderNotFound { account: String, folder: String },

    #[error("osascript failed: {0}")]
    OsascriptFailed(String),

    #[error("failed to execute osascript: {0}")]
    OsascriptIo(#[source] std::io::Error),

    #[error(
        "Reminder helper not found. Build it with ./scripts/build-swift-helper.sh or set APPLEKIT_REMINDER_HELPER."
    )]
    ReminderHelperNotFound,

    #[error("failed to execute reminder helper at {path}: {source}")]
    ReminderHelperIo {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Reminders access was denied. Allow access in System Settings > Privacy & Security > Reminders.")]
    RemindersAccessDenied,

    #[error("Reminder list \"{0}\" was not found.")]
    ReminderListNotFound(String),

    #[error("invalid reminder due datetime; expected format YYYY-MM-DD HH:MM")]
    InvalidReminderDate,

    #[error("reminder helper failed: {0}")]
    ReminderHelperFailed(String),
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::NotesUnavailable
            | AppError::NotesAutomationDenied
            | AppError::NotesAccountNotFound { .. }
            | AppError::NotesFolderNotFound { .. }
            | AppError::RemindersAccessDenied
            | AppError::ReminderListNotFound(_)
            | AppError::InvalidReminderDate => 2,
            AppError::OsascriptFailed(_)
            | AppError::OsascriptIo(_)
            | AppError::ReminderHelperNotFound
            | AppError::ReminderHelperIo { .. }
            | AppError::ReminderHelperFailed(_) => 1,
        }
    }
}
