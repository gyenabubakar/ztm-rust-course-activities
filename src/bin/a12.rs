// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

#[allow(dead_code)]
enum Colour {
    Red,
    Yellow,
    Blue,
}

impl Colour {
    fn stringify(&self) -> &str {
        match self {
            Colour::Red => "Red",
            Colour::Yellow => "Yellow",
            Colour::Blue => "Blue"
        }
    }
}

struct Box {
    width: f32,
    height: f32,
    length: f32,
    weight: f32,
    colour: Colour,
}

impl Box {
    fn new() -> Self {
        Box {
            colour: Colour::Red,
            weight: 0.0,
            height: 0.0,
            length: 0.0,
            width: 0.0,
        }
    }

    fn print(&self) {
        println!("Weight: {} lbs", self.weight);
        println!("Colour: {}", self.colour.stringify());
        println!("Dimensions: {}cm x {}cm x {}cm", self.length, self.width, self.height);
    }
}

fn main() {
    let mut _box = Box::new();
    _box.colour = Colour::Blue;
    _box.weight = 30.0;
    _box.width = 16.0;
    _box.length = 26.0;
    _box.height = 13.0;
    _box.print()
}
