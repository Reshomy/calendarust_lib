use chrono::{NaiveDate, NaiveTime};

use calendarust_lib::{
    items::{Anniversary, ItemKind, Task},
    planning::Planning,
};

fn main() {
    println!("==================================");
    println!("       CALENDARUST LIB SHOWCASE");
    println!("==================================\n");

    // --------------------------------------------------
    // 1. Create a planning
    // --------------------------------------------------

    let mut planning = Planning::new();

    // --------------------------------------------------
    // 2. Create anniversaries
    // --------------------------------------------------

    let birthday =
        Anniversary::new(String::from("Alice's birthday"), 11, 19).expect("Invalid birthday");

    let company_anniversary = Anniversary::new(String::from("Company anniversary"), 6, 15)
        .expect("Invalid company anniversary");

    let birthday_id = planning.add_item(ItemKind::Anniversary(birthday));

    let company_id = planning.add_item(ItemKind::Anniversary(company_anniversary));

    println!(
        "Created anniversary IDs: {} and {}",
        birthday_id, company_id
    );

    // --------------------------------------------------
    // 3. Create tasks
    // --------------------------------------------------

    let meeting = Task::new(
        String::from("Team meeting"),
        String::from("Discuss the next project milestones."),
        NaiveDate::from_ymd_opt(2026, 11, 19).unwrap(),
        NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
    )
    .expect("Invalid meeting");

    let dentist = Task::new(
        String::from("Dentist appointment"),
        String::from("Regular dental check-up."),
        NaiveDate::from_ymd_opt(2026, 11, 20).unwrap(),
        NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
        NaiveTime::from_hms_opt(15, 30, 0).unwrap(),
    )
    .expect("Invalid dentist appointment");

    let project_work = Task::new(
        String::from("Project work"),
        String::from("Finish the calendar engine."),
        NaiveDate::from_ymd_opt(2026, 11, 19).unwrap(),
        NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
        NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
    )
    .expect("Invalid project task");

    let meeting_id = planning.add_item(ItemKind::Task(meeting));

    let dentist_id = planning.add_item(ItemKind::Task(dentist));

    let project_id = planning.add_item(ItemKind::Task(project_work));

    println!(
        "Created task IDs: {}, {}, {}",
        meeting_id, dentist_id, project_id
    );

    // --------------------------------------------------
    // 4. Display the complete planning
    // --------------------------------------------------

    println!("\nSTORED ITEMS");
    println!("------------");

    for (index, item) in planning.items().iter().enumerate() {
        match item.kind() {
            ItemKind::Anniversary(anniversary) => {
                println!(
                    "{}. [ID: {}] ANNIVERSARY: {} ({:02}/{:02})",
                    index + 1,
                    item.id(),
                    anniversary.name(),
                    anniversary.day(),
                    anniversary.month()
                );
            }

            ItemKind::Task(task) => {
                println!("{}. [ID: {}] TASK: {}", index + 1, item.id(), task.title());
                println!("   Date: {}", task.date());
                println!("   Time: {} - {}", task.start_time(), task.end_time());
                println!("   Description: {}", task.description());
            }
        }
    }

    // --------------------------------------------------
    // 5. Retrieve items for a specific date
    // --------------------------------------------------

    let selected_date = NaiveDate::from_ymd_opt(2026, 11, 19).unwrap();

    println!("\nDAILY VIEW: {}", selected_date);
    println!("----------------------------");

    let daily_items = planning.items_on(selected_date);

    if daily_items.is_empty() {
        println!("Nothing scheduled for this date.");
    } else {
        for item in daily_items {
            match item.kind() {
                ItemKind::Anniversary(anniversary) => {
                    println!("[ID: {}] [ANNIVERSARY] {}", item.id(), anniversary.name());
                }

                ItemKind::Task(task) => {
                    println!(
                        "[ID: {}] [{} - {}] {}",
                        item.id(),
                        task.start_time(),
                        task.end_time(),
                        task.title()
                    );

                    if !task.description().is_empty() {
                        println!("    {}", task.description());
                    }
                }
            }
        }
    }

    // --------------------------------------------------
    // 6. Demonstrate yearly recurrence
    // --------------------------------------------------

    let next_year = NaiveDate::from_ymd_opt(2027, 11, 19).unwrap();

    println!("\nYEARLY RECURRENCE");
    println!("-----------------");
    println!("Looking at: {}", next_year);

    let next_year_items = planning.items_on(next_year);

    for item in next_year_items {
        match item.kind() {
            ItemKind::Anniversary(anniversary) => {
                println!("[ID: {}] [ANNIVERSARY] {}", item.id(), anniversary.name());
            }

            ItemKind::Task(task) => {
                println!("[ID: {}] [TASK] {}", item.id(), task.title());
            }
        }
    }

    // --------------------------------------------------
    // 7. Demonstrate leap-day anniversary
    // --------------------------------------------------

    let leap_day = Anniversary::new(String::from("Leap day birthday"), 2, 29)
        .expect("Invalid leap-day anniversary");

    let leap_day_id = planning.add_item(ItemKind::Anniversary(leap_day));

    let leap_date = NaiveDate::from_ymd_opt(2028, 2, 29).unwrap();

    println!("\nLEAP DAY");
    println!("--------");
    println!("Looking at: {}", leap_date);

    for item in planning.items_on(leap_date) {
        if let ItemKind::Anniversary(anniversary) = item.kind() {
            println!("[ID: {}] [ANNIVERSARY] {}", item.id(), anniversary.name());
        }
    }

    println!("Leap-day anniversary ID: {}", leap_day_id);

    // --------------------------------------------------
    // 8. Demonstrate item removal
    // --------------------------------------------------

    println!("\nITEM REMOVAL");
    println!("------------");

    println!("Items before removal:");

    for (index, item) in planning.items().iter().enumerate() {
        match item.kind() {
            ItemKind::Anniversary(anniversary) => {
                println!("{}. [ID: {}] {}", index + 1, item.id(), anniversary.name());
            }

            ItemKind::Task(task) => {
                println!("{}. [ID: {}] {}", index + 1, item.id(), task.title());
            }
        }
    }

    // For now, removal still uses a zero-based index.
    // We will replace this with ID-based removal later.

    let displayed_number = 2;

    println!("\nRemoving item number {}...", displayed_number);

    if displayed_number > 0 && displayed_number <= planning.items().len() {
        let index = displayed_number - 1;

        match planning.remove_item(index) {
            Some(item) => match item.kind() {
                ItemKind::Anniversary(anniversary) => {
                    println!(
                        "Removed anniversary (ID: {}): {}",
                        item.id(),
                        anniversary.name()
                    );
                }

                ItemKind::Task(task) => {
                    println!("Removed task (ID: {}): {}", item.id(), task.title());
                }
            },

            None => {
                println!("Item not found.");
            }
        }
    } else {
        println!("Invalid item number.");
    }

    println!("\nItems after removal:");

    for (index, item) in planning.items().iter().enumerate() {
        match item.kind() {
            ItemKind::Anniversary(anniversary) => {
                println!("{}. [ID: {}] {}", index + 1, item.id(), anniversary.name());
            }

            ItemKind::Task(task) => {
                println!("{}. [ID: {}] {}", index + 1, item.id(), task.title());
            }
        }
    }

    // --------------------------------------------------
    // 9. Final summary
    // --------------------------------------------------

    println!("\nSUMMARY");
    println!("-------");
    println!("Total stored items: {}", planning.items().len());
    println!("Showcase completed.");
}
