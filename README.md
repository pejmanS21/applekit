# applekit

`applekit` is a macOS-only command-line tool for creating Apple Notes and Apple Reminders from the terminal.

- Notes are created through AppleScript executed by `/usr/bin/osascript`.
- Reminders are created by a Swift helper that uses EventKit.
- All data is sent only to local Apple apps and frameworks on your Mac.

## Requirements

- macOS
- Rust toolchain (`cargo`)
- Xcode Command Line Tools for `swiftc`
- Apple Notes app
- Apple Reminders app/EventKit

## Build

Build the Rust CLI:

```sh
cargo build --release
```

Build the Swift Reminders helper:

```sh
./scripts/build-swift-helper.sh
```

The helper is compiled to:

```text
target/helper/ReminderHelper
```

The CLI looks for the helper in this order:

1. `APPLEKIT_REMINDER_HELPER`
2. `./target/helper/ReminderHelper` relative to the current working directory
3. `target/helper/ReminderHelper` next to the installed `applekit` binary, when available
4. `ReminderHelper` next to the installed `applekit` binary

Run from the repository after building:

```sh
./target/release/applekit --help
```

## Permissions

macOS may prompt the first time each integration is used.

- Notes: Automation permission is required so `osascript` can control Notes.app.
- Reminders: Reminders access is required so the Swift helper can use EventKit.

If you deny access, enable it later in System Settings:

- Privacy & Security > Automation
- Privacy & Security > Reminders

## Commands

Create a note:

```sh
applekit note create \
  --title "Project idea" \
  --body "Build a Rust automation CLI"
```

Create a note in a specific account and folder:

```sh
applekit note create \
  --title "Daily log" \
  --body "Worked on AppleKit" \
  --account "iCloud" \
  --folder "Notes"
```

Create a reminder:

```sh
applekit reminder create \
  --title "Call doctor" \
  --due "2026-05-04 09:00"
```

Create a reminder with notes, list, and priority:

```sh
applekit reminder create \
  --title "Submit report" \
  --due "2026-05-05 17:30" \
  --notes "Attach final PDF" \
  --list "Work" \
  --priority 5
```

## Date Format

Reminder due dates use local time in this exact format:

```text
YYYY-MM-DD HH:MM
```

Example:

```text
2026-05-04 09:00
```

## Troubleshooting

### `Reminder helper not found`

Compile the helper:

```sh
./scripts/build-swift-helper.sh
```

Or point the CLI at a custom helper:

```sh
APPLEKIT_REMINDER_HELPER=/path/to/ReminderHelper applekit reminder create --title "Call doctor"
```

### Notes account or folder not found

Confirm the account and folder names in Notes.app. The defaults are:

```text
account: iCloud
folder: Notes
```

### Automation permission denied

Open System Settings > Privacy & Security > Automation and allow your terminal app to control Notes.

### Reminders access denied

Open System Settings > Privacy & Security > Reminders and allow the helper or terminal app to access Reminders. You may need to run the command again after changing the setting.

### `osascript` failed

Run the command from Terminal.app and watch for macOS permission prompts. Notes.app must be installed and available.

## Security And Privacy

`applekit` does not send note or reminder content to a server. Note text is passed to local `osascript` and Notes.app. Reminder text is passed to the local Swift helper and EventKit.

