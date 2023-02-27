#![allow(dead_code)]

// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::io;

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

enum MenuChoice {
    Add,
    View,
}

impl Bill {
    fn new(name: &str, amount: &f64) -> Self {
        Bill {
            name: name.to_owned(),
            amount: amount.to_owned(),
        }
    }
}

fn get_menu_choice(input: &str) -> Option<MenuChoice> {
    if input == "1" {
        Some(MenuChoice::Add)
    } else if input == "2" {
        Some(MenuChoice::View)
    } else {
        None
    }
}

fn add_bill() -> io::Result<Option<Bill>> {
    let mut name = String::new();
    let mut amount_input = String::new();
    let mut bill = Bill::new(&name, &0.0);

    println!("Bill name:");
    io::stdin().read_line(&mut name)?;

    if !name.trim().is_empty() {
        bill.name = name.trim().to_owned();

        println!("Amount:");
        io::stdin().read_line(&mut amount_input)?;

        if !amount_input.trim().is_empty() {
            if let Ok(amt) = amount_input.trim().parse::<f64>() {
                bill.amount = amt;
                return Ok(Some(bill));
            } else {
                println!("Enter a valid number.\n")
            }
        }
    }

    Ok(None)
}

fn view_bills(bills: &Vec<Bill>) {
    for bill in bills.iter() {
        println!("Bill – {:?}", bill);
    }
}

fn main() {
    let mut bills: Vec<Bill> = vec![];

    'start: loop {
        println!("\n== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills\n");
        println!("Enter selection:");

        let mut buff = String::new();
        let result = io::stdin().read_line(&mut buff);

        if result.is_ok() {
            let trimmed = buff.trim().to_owned();
            if let Some(choice) = get_menu_choice(&trimmed) {
                use MenuChoice::{Add, View};
                match choice {
                    Add => {
                        if let Ok(bill) = add_bill() {
                            if bill.is_some() {
                                bills.push(bill.unwrap())
                            }
                        }
                    }
                    View => view_bills(&bills),
                }
                continue 'start;
            }
        }
        return;
    }
}
