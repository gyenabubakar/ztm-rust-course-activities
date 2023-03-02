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

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Bill {
  name: String,
  amount: f64,
}

enum MenuChoice {
  Add,
  View,
  Remove,
  Update,
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
  use MenuChoice::*;
  match input {
    "1" => Some(Add),
    "2" => Some(View),
    "3" => Some(Remove),
    "4" => Some(Update),
    _ => None,
  }
}

fn add_bill(bills: &mut HashMap<String, Bill>) {
  let mut name = String::new();
  let mut amount_input = String::new();

  println!("Bill name:");
  let mut result = io::stdin().read_line(&mut name);

  if result.is_ok() {
    name = name.trim().to_owned();

    if bills.contains_key(&name) {
      println!("❌ The bill {:?} already exists.", name);
      return;
    }

    if !name.is_empty() {
      println!("Amount:");
      result = io::stdin().read_line(&mut amount_input);

      if result.is_ok() {
        amount_input = amount_input.trim().to_owned();

        if !amount_input.is_empty() {
          if let Ok(amount) = amount_input.trim().parse::<f64>() {
            let name_cloned = name.clone();
            bills.insert(name_cloned, Bill { name, amount });
          } else {
            println!("Enter a valid number.\n")
          }
        }
      }
    }
  }
}

fn view_bills(bills: &HashMap<String, Bill>) {
  for (_, bill) in bills.iter() {
    println!("👉 name: {:?}, amount: \"{:?}\"", bill.name, bill.amount);
  }
}

fn remove_bill(bills: &mut HashMap<String, Bill>) {
  let mut name = String::new();
  println!("Enter bill name to remove:");
  let result = io::stdin().read_line(&mut name);
  name = name.trim().to_owned();

  if result.is_ok() && !name.is_empty() {
    if let Some(bill) = bills.remove(&name) {
      println!("✅ Deleted {:?} successfully.", bill.name)
    } else {
      println!("❌ Found no bill named {:?}.", name)
    }
  }
}

fn update_bill(bills: &mut HashMap<String, Bill>) {
  println!();
  view_bills(bills);

  let mut name = String::new();
  let mut new_name = String::new();
  let mut amount_input = String::new();

  println!("Enter bill to update:");

  if let Ok(_) = io::stdin().read_line(&mut name) {
    name = name.trim().to_owned();
    if !bills.contains_key(&name) {
      if !name.is_empty() {
        println!("❌ {:?} does not exist!", name);
      }
      return;
    }

    let target_bill = bills.get(&name).unwrap();
    let mut new_bill = Bill {
      name: target_bill.name.clone(),
      amount: target_bill.amount,
    };

    println!("Enter new name:");
    if let Ok(_) = io::stdin().read_line(&mut new_name) {
      new_name = new_name.trim().to_owned();

      if !new_name.is_empty() {
        new_bill.name = new_name.clone();
      } else {
        println!("⚠️ No new name entered. Keeping current name.\n");
      }

      println!("Enter new amount:");
      if let Ok(_) = io::stdin().read_line(&mut amount_input) {
        amount_input = amount_input.trim().to_owned();

        if let Ok(amount) = amount_input.parse::<f64>() {
          new_bill.amount = amount;
        } else {
          println!(
            "⚠️ {:?} is not a valid number. Keeping current amount.",
            amount_input
          );
        }
      }

      bills.insert(new_name, new_bill.clone());
      bills.remove(&name);

      println!("✅ Updated bill successfully:");
      println!(
        "👉 name: {:?}, amount: \"{:?}\"",
        new_bill.name, new_bill.amount
      );
    }
  }
}

fn main() {
  let mut bills: HashMap<String, Bill> = HashMap::new();

  loop {
    println!("\n== Manage Bills ==");
    println!("1. Add bill");
    println!("2. View bills");
    println!("3. Remove bill");
    println!("4. Update bill\n");
    println!("Enter selection:");

    let mut buff = String::new();
    let result = io::stdin().read_line(&mut buff);
    let trimmed = buff.trim().to_owned();

    if trimmed.parse::<u8>().is_ok() && result.is_ok() {
      if let Some(choice) = get_menu_choice(&trimmed) {
        use MenuChoice::*;
        match choice {
          Add => add_bill(&mut bills),
          View => view_bills(&bills),
          Remove => remove_bill(&mut bills),
          Update => update_bill(&mut bills),
        }
      }
      continue;
    }
    break;
  }
}
