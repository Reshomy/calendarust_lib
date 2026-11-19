# calendarust_lib

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue.svg)](#license)
[![GitHub Repository](https://img.shields.io/badge/GitHub-Repository-black?logo=github)](https://github.com/Reshomy/calendarust_lib)
[![Status](https://img.shields.io/badge/status-Early%20Development-yellow.svg)](#project-status)

**A reusable Rust library for calendar, anniversary, and task management.**

`calendarust_lib` is a calendar and task management engine written in Rust. It provides the core functionality for managing calendar items, recurring anniversaries, and scheduled tasks.

Designed to be independent of any particular user interface, the library can serve as a foundation for different types of applications:

- Desktop applications
- Web applications
- Command-line applications
- Other Rust projects

The goal is to provide a flexible and extensible calendar and planning engine while leaving user interfaces and application-specific functionality to the projects that use it.

---

## Features

### Implemented

| Feature | Description |
| --- | --- |
| Anniversaries | Store a name, month, and day |
| Recurrence | Automatically recognize recurring anniversaries each year |
| Leap years | Support February 29 anniversaries |
| Tasks | Store titles, descriptions, dates, start times, and end times |
| Validation | Validate task time ranges |
| Planning | Manage anniversaries and tasks in a single collection |
| Unique IDs | Automatically assign a unique ID to each item |
| Item types | Distinguish between anniversaries and tasks using `ItemKind` |
| Retrieval | Retrieve all calendar items |
| Date queries | Retrieve items occurring on a specific date |
| Removal | Remove items by their current position |

### Planned features

- [ ] Find items by ID
- [ ] Edit and remove items by ID
- [ ] Sorting and improved error handling
- [ ] Date-range queries and calendar views
- [ ] Events and recurrence management
- [ ] Task statuses, priorities, deadlines, and categories
- [ ] JSON-based local data persistence
- [ ] Additional tests and documentation

---

## Installation

Add `calendarust_lib` directly from GitHub to your project's `Cargo.toml`:

```toml
[dependencies]
calendarust_lib = { git = "https://github.com/Reshomy/calendarust_lib.git" }
```

Alternatively, if you are working with a local copy:

```toml
[dependencies]
calendarust_lib = { path = "../calendarust_lib" }
```

### Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)

---

## Usage

The following example demonstrates how to create a planning instance, add an anniversary and a task, and retrieve calendar items for a specific date.

```rust
use chrono::{NaiveDate, NaiveTime};

use calendarust_lib::{
    items::{Anniversary, ItemKind, Task},
    planning::Planning,
};

fn main() {
    let mut planning = Planning::new();

    // Create an anniversary
    let birthday = Anniversary::new(
        String::from("Birthday"),
        5,
        15,
    )
    .expect("Invalid anniversary");

    // Add the anniversary and retrieve its automatically assigned ID
    let birthday_id = planning.add_item(ItemKind::Anniversary(birthday));

    println!("Birthday ID: {}", birthday_id);

    // Create a task
    let task = Task::new(
        String::from("Finish project"),
        String::from("Complete the Rust project"),
        NaiveDate::from_ymd_opt(2026, 11, 25).unwrap(),
        NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
    )
    .expect("Invalid task");

    // Add the task and retrieve its automatically assigned ID
    let task_id = planning.add_item(ItemKind::Task(task));

    println!("Task ID: {}", task_id);

    // Retrieve items for a specific date
    let date = NaiveDate::from_ymd_opt(2026, 11, 25).unwrap();
    let items = planning.items_on(date);

    println!("Items found: {}", items.len());
}
```

---

## Project Structure

```text
calendarust_lib/
├── Cargo.toml
├── README.md
├── LICENSE-APACHE
├── LICENSE-MIT
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── planning.rs
│   └── items/
│       ├── mod.rs
│       ├── item.rs
│       ├── anniversary.rs
│       └── task.rs
└── tests/
    └── planning_tests.rs
```

---

## Dependencies

The project currently uses:

- [Chrono](https://crates.io/crates/chrono) — Date and time handling.

---

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)

### Clone the repository

```bash
git clone https://github.com/Reshomy/calendarust_lib.git
cd calendarust_lib
```

### Run tests

```bash
cargo test
```

### Run the showcase

```bash
cargo run
```

### Build the library

```bash
cargo build
```

### Build an optimized release

```bash
cargo build --release
```

---

## Project Goals

`calendarust_lib` aims to provide a reusable foundation for calendar and task management applications.

The project follows several principles:

- **Modularity:** Keep calendar logic independent of application interfaces.
- **Reusability:** Make the library suitable for different types of applications.
- **Extensibility:** Allow new calendar item types and planning functionality to be introduced over time.
- **Reliability:** Provide validation and predictable behavior for calendar operations.

JSON is planned as the initial persistence format, with the possibility of supporting additional storage implementations in the future.

---

## Contributing

Contributions, suggestions, bug reports, and feature requests are welcome!

If you would like to contribute:

1. Fork the repository.
2. Create a branch for your changes.
3. Implement your changes and add relevant tests.
4. Run the test suite using `cargo test`.
5. Submit a pull request.

For significant changes, consider opening an issue first to discuss the proposed functionality.

Unless explicitly stated otherwise, contributions intentionally submitted for inclusion in this project are licensed under the same dual-license terms as the project, as described in the Apache License 2.0 and MIT License.

---

## Contributors

Thanks to everyone who has contributed to `calendarust_lib`!

<a href="https://github.com/Reshomy/calendarust_lib/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Reshomy/calendarust_lib" />
</a>

Contributions, suggestions, and improvements are always welcome!

---

## License

This project is **dual-licensed** under your choice of either:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

You may choose either license when using, modifying, or distributing this project.

For more information, see the full license texts:

- [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](https://opensource.org/license/mit/)

---

## Project Status

**Early development — APIs and features may change.**

`calendarust_lib` is actively being developed. Its API, internal structure, and functionality may evolve as the project grows.

Some features described in this README are planned and have not yet been implemented.

Feedback, suggestions, and contributions are appreciated!
