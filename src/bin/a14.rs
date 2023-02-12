// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i16,
    fav_colour: String,
}

fn display_child_info(name: &str, colour: &str) {
    println!("Name: {}\nColour: {}.\n", name, colour)
}

fn main() {
    let people = vec![
        Person {
            name: String::from("Gyen Abubakar"),
            age: 17,
            fav_colour: String::from("Red"),
        },
        Person {
            name: "De-Graft Arthur".to_owned(),
            age: 10,
            fav_colour: "Blue".to_owned(),
        },
        Person {
            name: "Felix Amoako".to_owned(),
            age: 8,
            fav_colour: String::from("Yellow"),
        },
    ];

    for person in people {
        if person.age <= 10 {
            display_child_info(&person.name, &person.fav_colour)
        }
    }
}
