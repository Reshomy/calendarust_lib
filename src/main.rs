use calendar_tasks::items::Anniversary;

fn main() {
    let anniversary = Anniversary::new(String::from("Alice's birthday"), 2, 29).unwrap();

    println!("{}", anniversary.name());
}
