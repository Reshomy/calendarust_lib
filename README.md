# calendarust_lib

A Rust library for calendar and task management.

`calendarust_lib` is a reusable library designed to provide the core functionality for managing calendar items, anniversaries, tasks, and planning.

The goal is to build a flexible calendar and task management engine that can be integrated into different applications, including web, desktop, and CLI applications.

## Features

### Currently implemented

- **Anniversaries**
  - Store a name, month, and day.
  - Automatically recognize recurring anniversaries each year.
  - Support February 29 anniversaries.

- **Tasks**
  - Store a title, description, date, start time, and end time.
  - Validate task time ranges.

- **Planning**
  - Store anniversaries and tasks.
  - Retrieve all items.
  - Retrieve items occurring on a specific date.
  - Remove items by their current position.

### Planned features

- Unique IDs for calendar items.
- Find, edit, and remove items by ID.
- Sorting and improved error handling.
- Date-range queries and calendar views.
- Events and recurrence management.
- Task statuses, priorities, deadlines, and categories.
- JSON-based local data persistence.
- Additional tests and documentation.

## Project Structure

```text
calendarust_lib/
├── Cargo.toml
├── README.md
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

## Installation

Add the library to your project's `Cargo.toml`:

```toml
[dependencies]
calendarust_lib = { git = "https://github.com/Reshomy/calendarust_lib.git" }
```

Alternatively, when working with a local copy:

```toml
[dependencies]
calendarust_lib = { path = "../calendarust_lib" }
```

## Usage

A basic example of creating a planning and adding items:

```rust
use calendarust_lib::{
    items::{Anniversary, Item, Task},
    planning::Planning,
};

use chrono::{NaiveDate, NaiveTime};

fn main() {
    let mut planning = Planning::new();

    // Create an anniversary
    let birthday = Anniversary::new(
        "Birthday".to_string(),
        5,
        15,
    ).expect("Invalid anniversary");

    planning.add_item(Item::Anniversary(birthday));

    // Create a task
    let task = Task::new(
        "Finish project".to_string(),
        "Complete the Rust project".to_string(),
        NaiveDate::from_ymd_opt(2026, 11, 25).unwrap(),
        NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
    ).expect("Invalid task");

    planning.add_item(Item::Task(task));

    // Retrieve items for a specific date
    let date = NaiveDate::from_ymd_opt(2026, 11, 25).unwrap();
    let items = planning.items_on(date);

    println!("Items found: {}", items.len());
}
```

## Dependencies

- [chrono](https://crates.io/crates/chrono) — Date and time handling.

## Development

Run the tests:

```bash
cargo test
```

Run the showcase:

```bash
cargo run
```

## Project Goals

`calendarust_lib` aims to provide a reusable foundation for calendar and task management applications.

The library will focus on the underlying logic and data management, leaving the user interface and application-specific functionality to projects built on top of it.

JSON will be the initial persistence format, with the possibility of supporting additional storage implementations in the future.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Status

**Early development — APIs and features may change.**
