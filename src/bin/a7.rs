// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

#[allow(dead_code)]
enum Colour {
    Red,
    Blue,
    Green,
}

fn display_colour(colour: Colour) {
    match colour {
        Colour::Red => println!("red"),
        Colour::Blue => println!("blue"),
        Colour::Green => println!("green"),
    }
}

fn main() {
    display_colour(Colour::Green);
}
