//cargo add chrono in terminal to add a dependency

use chrono::{Local, Utc};

fn get_time() {
    //Get the current date and time in UTC
    let now = Utc::now();
    println!("Current date and time in UTC: {}", now);

    //Format the date and time
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("The curren time formatted: {}", formatted);

    //Get local time
    let local = Local::now();
    println!("Current date and time in local: {}", local);
}

fn main() {
    get_time();
}
