// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with ti me
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats
use chrono::{Datelike, Local};

fn main() {
    let date = Local::now();
    let readable = date.format("%A, %d %B, %Y");
    println!("{}", readable.to_string());
    println!("date–{} month–{} year–{}", date.day(), date.month(), date.year())
}
