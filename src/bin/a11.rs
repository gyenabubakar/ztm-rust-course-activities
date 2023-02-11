// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn display_quantity(item: &GroceryItem) {
    print!("Quantity: {}\n", item.quantity)
}

fn display_id(item: &GroceryItem) {
    print!("ID: {}\n", item.id)
}

fn main() {
    let grocery_item = GroceryItem {
        id: 1,
        quantity: 45,
    };

    display_id(&grocery_item);
    display_quantity(&grocery_item);
}
