// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut items: HashMap<&str, i32> = HashMap::new();
    items.insert("Chairs", 5);
    items.insert("Beds", 3);
    items.insert("Tables", 2);
    items.insert("Couches", 0);

    let mut items_sum = 0;

    for (item, q) in items.iter() {
        let quantity = if *q == 0 {
            "out of stock".to_owned()
        } else {
            q.to_string()
        };
        println!("{} – {}", item, quantity);

        items_sum += q;
    }

    println!();
    println!("Total # of items in stock – {}", items_sum)
}
