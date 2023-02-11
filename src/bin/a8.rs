// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavour {
    Almond,
    Apple,
    Apricot,
}

struct Drink {
    flavour: Flavour,
    fluid_ounces: f32,
}

fn display_drink(drink: Drink) {
    let mut flavour;

    match drink.flavour {
        Flavour::Almond => flavour = "Almond",
        Flavour::Apple => flavour = "Apple",
        Flavour::Apricot => flavour = "Apricot",
    }

    println!("Flavour: {:?}. Ounces: {:?}.", flavour, drink.fluid_ounces);
}

fn main() {
    let new_drink = Drink {
        fluid_ounces: 5.0,
        flavour: Flavour::Apricot,
    };

    display_drink(new_drink);
}
