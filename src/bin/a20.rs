// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

#![allow(dead_code)]

use std::io;

#[derive(Copy, Clone)]
enum PowerOption {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerOption {
    fn new(keyword: &str) -> Option<Self> {
        let owned = keyword.to_owned().to_lowercase();
        let trimmed = owned.trim();
        match trimmed {
            "off" => Some(Self::Off),
            "sleep" => Some(Self::Sleep),
            "reboot" => Some(Self::Reboot),
            "shutdown" => Some(Self::Shutdown),
            "hibernate" => Some(Self::Hibernate),
            _ => None
        }
    }
}

fn get_input() -> io::Result<Option<PowerOption>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let keyword = buffer.trim();
    let option = PowerOption::new(keyword);
    Ok(option)
}

fn print_power_message(opt: Option<PowerOption>) {
    use PowerOption::*;

    match opt {
        None => println!("You entered an invalid keyword."),
        Some(option) => {
            let message: &str;
            match option {
                Off => message = "going off...",
                Sleep => message = "sleeping...",
                Reboot => message = "rebooting",
                Shutdown => message = "shutting down",
                Hibernate => message = "hibernating"
            }
            println!("{message}")
        }
    }
}

fn main() {
    println!("Enter keyword: [off, sleep, reboot, shutdown, hibernate] 👇");

    let option = get_input();
    if option.is_err() {
        println!("ERR: {:?}", option.err());
        return;
    }

    print_power_message(option.unwrap());
}
