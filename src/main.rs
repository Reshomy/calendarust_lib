use chrono::{NaiveDate, NaiveTime};

use calendar_tasks::{
    items::{Anniversary, Item, Task},
    planning::Planning,
};

fn main() {
    println!("==================================");
    println!("       CALENDAR TASKS SHOWCASE");
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

    planning.add_item(Item::Anniversary(birthday));
    planning.add_item(Item::Anniversary(company_anniversary));

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

    planning.add_item(Item::Task(meeting));
    planning.add_item(Item::Task(dentist));
    planning.add_item(Item::Task(project_work));

    // --------------------------------------------------
    // 4. Display the complete planning
    // --------------------------------------------------

    println!("STORED ITEMS");
    println!("------------");

    for (index, item) in planning.items().iter().enumerate() {
        match item {
            Item::Anniversary(anniversary) => {
                println!(
                    "{}. ANNIVERSARY: {} ({:02}/{:02})",
                    index + 1,
                    anniversary.name(),
                    anniversary.day(),
                    anniversary.month()
                );
            }

            Item::Task(task) => {
                println!("{}. TASK: {}", index + 1, task.title());
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
            match item {
                Item::Anniversary(anniversary) => {
                    println!("[ANNIVERSARY] {}", anniversary.name());
                }

                Item::Task(task) => {
                    println!(
                        "[{} - {}] {}",
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
        match item {
            Item::Anniversary(anniversary) => {
                println!("[ANNIVERSARY] {}", anniversary.name());
            }

            Item::Task(task) => {
                println!("[TASK] {}", task.title());
            }
        }
    }

    // --------------------------------------------------
    // 7. Demonstrate leap-day anniversary
    // --------------------------------------------------

    let leap_day = Anniversary::new(String::from("Leap day birthday"), 2, 29)
        .expect("Invalid leap-day anniversary");

    planning.add_item(Item::Anniversary(leap_day));

    let leap_date = NaiveDate::from_ymd_opt(2028, 2, 29).unwrap();

    println!("\nLEAP DAY");
    println!("--------");
    println!("Looking at: {}", leap_date);

    for item in planning.items_on(leap_date) {
        if let Item::Anniversary(anniversary) = item {
            println!("[ANNIVERSARY] {}", anniversary.name());
        }
    }

    // --------------------------------------------------
    // 8. Final summary
    // --------------------------------------------------

    println!("\nSUMMARY");
    println!("-------");
    println!("Total stored items: {}", planning.items().len());
    println!("Showcase completed.");
}
