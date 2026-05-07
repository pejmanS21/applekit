# applekit

`applekit` is a macOS-only command-line tool for creating Apple Notes and Apple Reminders from the terminal.

- Notes are created through AppleScript executed by `/usr/bin/osascript`.
- Reminders are created by a Swift helper that uses EventKit.
- Data stays on the local Mac and is passed only to Apple apps and frameworks.

## Requirements

- macOS
- Rust toolchain (`cargo`)
- Xcode Command Line Tools (`xcode-select --install`)
- Notes.app and Reminders.app

## Build

Build the Rust CLI:

```sh
cargo build --release
```

Build the Swift Reminders helper:

```sh
./scripts/build-swift-helper.sh
```

The helper is written to:

```text
target/helper/ReminderHelper
```

Run from the repository:

```sh
./target/release/applekit --help
```

## Usage

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
  --body "Worked on applekit" \
  --account "iCloud" \
  --folder "Notes"
```

Create a reminder:

```sh
applekit reminder create \
  --title "Call doctor" \
  --due "2026-05-05 09:00"
```

Create a reminder with notes, list, tags, and priority:

```sh
applekit reminder create \
  --title "Submit report" \
  --due "2026-05-05 17:30" \
  --notes "Attach final PDF" \
  --list "Work" \
  --tags finance,urgent \
  --priority 5
```

Reminder due dates use local time in this exact format:

```text
YYYY-MM-DD HH:MM
```

## Helper Lookup

For reminder commands, `applekit` looks for `ReminderHelper` in this order:

1. `APPLEKIT_REMINDER_HELPER`
2. `./target/helper/ReminderHelper`
3. `target/helper/ReminderHelper` next to the `applekit` binary
4. `ReminderHelper` next to the `applekit` binary

Example with a custom helper:

```sh
APPLEKIT_REMINDER_HELPER=/path/to/ReminderHelper applekit reminder create --title "Call doctor"
```

## Permissions

macOS may prompt the first time each integration runs.

- Notes requires Automation permission for the terminal app to control Notes.app.
- Reminders requires Reminders access for EventKit.

If access is denied, enable it in System Settings:

- Privacy & Security > Automation
- Privacy & Security > Reminders

## Development

Common checks:

```sh
cargo fmt --all -- --check
cargo test --all-targets
cargo build --release
./scripts/build-swift-helper.sh
```

GitHub Actions builds native macOS artifacts for arm64 and amd64 from `.github/workflows/macos-build.yml`.

## Troubleshooting

### Reminder helper not found

Run:

```sh
./scripts/build-swift-helper.sh
```

Or set `APPLEKIT_REMINDER_HELPER`.

### Notes account or folder not found

Confirm names in Notes.app. Defaults are `iCloud` for account and `Notes` for folder.

### Permission denied

Open System Settings and grant Automation or Reminders access, then rerun the command.

## Privacy

`applekit` does not send note or reminder content to a server. Notes content goes to local `osascript` and Notes.app. Reminder content goes to the local Swift helper and EventKit.
