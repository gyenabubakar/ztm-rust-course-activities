// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn printNames(firstName: String, lastName: String) {
    println!("Your first name is {:?}", firstName);
    println!("Your last name is {:?}", lastName);
}

fn main() {
    printNames("Gyen", "Abubakar")
}
