# Repository Guidelines

## Project Structure & Module Organization

`applekit` is a macOS-focused Rust CLI for creating Apple Notes and Reminders. Rust source lives in `src/`: `main.rs` wires CLI dispatch, `cli.rs` defines Clap arguments and validation, `notes.rs` runs Notes AppleScript, `reminders.rs` invokes the Reminders helper, and `error.rs` centralizes user-facing errors and exit codes. The Swift EventKit helper lives at `swift/ReminderHelper.swift` and is built by `scripts/build-swift-helper.sh` into `target/helper/ReminderHelper`. Unit tests are inline under each Rust module’s `#[cfg(test)]` block. Build outputs stay under `target/`.

## Build, Test, and Development Commands

- `cargo check`: type-check the Rust CLI quickly.
- `cargo test`: run all Rust unit tests.
- `cargo fmt`: format Rust code using rustfmt defaults.
- `cargo clippy --all-targets --all-features`: run Rust lints before larger changes.
- `cargo build`: build the `applekit` binary.
- `./scripts/build-swift-helper.sh`: compile the Swift Reminders helper; requires Xcode Command Line Tools.
- `cargo run -- note create --title "Hello" --body "Body"`: exercise Notes creation locally.
- `cargo run -- reminder create --title "Pay rent" --due "2026-05-05 09:00"`: exercise Reminders creation after building the helper.

## Coding Style & Naming Conventions

Use Rust 2021 idioms and rustfmt formatting with four-space indentation. Keep modules small and aligned to product areas: CLI parsing in `cli.rs`, OS integration in `notes.rs` or `reminders.rs`, and reusable errors in `error.rs`. Prefer explicit, user-actionable error messages through `AppError`. Function and module names use `snake_case`; structs and enums use `UpperCamelCase`; constants use `SCREAMING_SNAKE_CASE`.

## Testing Guidelines

Add focused unit tests beside the code they validate. Existing tests cover CLI value parsing, AppleScript/helper error classification, tag formatting, and helper command construction. Name tests as behavior descriptions, for example `validates_due_format` or `classifies_helper_errors`. Run `cargo test` before submitting changes. For macOS integration behavior, manually verify Notes/Reminders permissions because unit tests do not launch Apple apps.

## Commit & Pull Request Guidelines

The current Git history only contains `Initial Commit`, so no detailed commit convention is established. Use concise imperative subjects such as `Add reminder priority validation`. Pull requests should describe the user-visible change, list commands run, note any macOS permission or helper-build requirements, and include screenshots or terminal output only when behavior is visual or interactive.

## Security & Configuration Tips

Do not commit local helper binaries or `target/` contents. Use `APPLEKIT_REMINDER_HELPER=/path/to/ReminderHelper` when testing alternate helper builds. Avoid logging note bodies, reminder notes, or other personal user content in diagnostics.
