import EventKit
import Foundation

enum HelperError: Error {
    case missingValue(String)
    case unknownArgument(String)
    case emptyTitle
    case invalidPriority(String)
    case invalidDate
    case accessDenied
    case listNotFound(String)
    case noDefaultCalendar
    case saveFailed(Error)
}

struct ReminderInput {
    var title: String?
    var due: String?
    var notes: String?
    var list: String?
    var priority: Int?
}

let dueFormatter: DateFormatter = {
    let formatter = DateFormatter()
    formatter.locale = Locale(identifier: "en_US_POSIX")
    formatter.dateFormat = "yyyy-MM-dd HH:mm"
    formatter.timeZone = TimeZone.current
    return formatter
}()

func parseArguments(_ arguments: [String]) throws -> ReminderInput {
    var input = ReminderInput()
    var index = 0

    while index < arguments.count {
        let argument = arguments[index]

        func nextValue() throws -> String {
            let valueIndex = index + 1
            guard valueIndex < arguments.count else {
                throw HelperError.missingValue(argument)
            }
            index += 2
            return arguments[valueIndex]
        }

        switch argument {
        case "--title":
            input.title = try nextValue()
        case "--due":
            input.due = try nextValue()
        case "--notes":
            input.notes = try nextValue()
        case "--list":
            input.list = try nextValue()
        case "--priority":
            let raw = try nextValue()
            guard let priority = Int(raw), (0...9).contains(priority) else {
                throw HelperError.invalidPriority(raw)
            }
            input.priority = priority
        default:
            throw HelperError.unknownArgument(argument)
        }
    }

    if input.title?.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty != false {
        throw HelperError.emptyTitle
    }

    return input
}

func requestRemindersAccess(store: EKEventStore) async throws -> Bool {
    if #available(macOS 14.0, *) {
        return try await store.requestFullAccessToReminders()
    } else {
        return try await withCheckedThrowingContinuation { continuation in
            store.requestAccess(to: .reminder) { granted, error in
                if let error {
                    continuation.resume(throwing: error)
                } else {
                    continuation.resume(returning: granted)
                }
            }
        }
    }
}

func authorizationAllowsReminders(_ status: EKAuthorizationStatus) -> Bool {
    if #available(macOS 14.0, *) {
        return status == .authorized || status == .fullAccess
    }

    switch status {
    case .authorized:
        return true
    default:
        return false
    }
}

func targetCalendar(store: EKEventStore, listName: String?) throws -> EKCalendar {
    if let listName, !listName.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
        if let calendar = store.calendars(for: .reminder).first(where: { $0.title == listName }) {
            return calendar
        }
        throw HelperError.listNotFound(listName)
    }

    guard let calendar = store.defaultCalendarForNewReminders() else {
        throw HelperError.noDefaultCalendar
    }
    return calendar
}

func createReminder(input: ReminderInput) async throws {
    let store = EKEventStore()
    let initialStatus = EKEventStore.authorizationStatus(for: .reminder)

    if !authorizationAllowsReminders(initialStatus) {
        let granted = try await requestRemindersAccess(store: store)
        guard granted else {
            throw HelperError.accessDenied
        }
    }

    let reminder = EKReminder(eventStore: store)
    reminder.title = input.title!.trimmingCharacters(in: .whitespacesAndNewlines)
    reminder.notes = input.notes
    reminder.calendar = try targetCalendar(store: store, listName: input.list)

    if let due = input.due {
        guard let date = dueFormatter.date(from: due) else {
            throw HelperError.invalidDate
        }

        let calendar = Calendar.current
        reminder.dueDateComponents = calendar.dateComponents(
            [.year, .month, .day, .hour, .minute],
            from: date
        )
    }

    if let priority = input.priority {
        reminder.priority = priority
    }

    do {
        try store.save(reminder, commit: true)
    } catch {
        throw HelperError.saveFailed(error)
    }
}

func printError(_ error: Error) {
    switch error {
    case HelperError.missingValue(let argument):
        fputs("APPLEKIT_MISSING_VALUE: \(argument)\n", stderr)
    case HelperError.unknownArgument(let argument):
        fputs("APPLEKIT_UNKNOWN_ARGUMENT: \(argument)\n", stderr)
    case HelperError.emptyTitle:
        fputs("APPLEKIT_EMPTY_TITLE\n", stderr)
    case HelperError.invalidPriority(let raw):
        fputs("APPLEKIT_INVALID_PRIORITY: \(raw)\n", stderr)
    case HelperError.invalidDate:
        fputs("APPLEKIT_INVALID_DATE\n", stderr)
    case HelperError.accessDenied:
        fputs("APPLEKIT_REMINDERS_ACCESS_DENIED\n", stderr)
    case HelperError.listNotFound(let list):
        fputs("APPLEKIT_LIST_NOT_FOUND: \(list)\n", stderr)
    case HelperError.noDefaultCalendar:
        fputs("APPLEKIT_NO_DEFAULT_CALENDAR\n", stderr)
    case HelperError.saveFailed(let saveError):
        fputs("APPLEKIT_SAVE_FAILED: \(saveError.localizedDescription)\n", stderr)
    default:
        fputs("APPLEKIT_UNEXPECTED_ERROR: \(error.localizedDescription)\n", stderr)
    }
}

@main
struct ReminderHelper {
    static func main() async {
        do {
            let input = try parseArguments(Array(CommandLine.arguments.dropFirst()))
            try await createReminder(input: input)
            print("{\"ok\":true}")
            exit(0)
        } catch {
            printError(error)
            exit(1)
        }
    }
}
