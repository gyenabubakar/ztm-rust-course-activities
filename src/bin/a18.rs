// Topic: Result
//
// Requirements:
// * Create a structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: i16,
}

impl Adult {
    fn new(name: &str, age: i16) -> Result<Self, String> {
        if age >= 21 {
            return Ok(Self {
                name: name.to_owned(),
                age,
            });
        }
        Err("Age must be at least 21".to_owned())
    }
}

fn main() {
    let person = Adult::new("Kofi", 25);
    let person2 = Adult::new("Abena", 18);

    match person {
        Ok(adult) => println!("Adult, name – {}... age – {}.", adult.name, adult.age),
        Err(msg) => println!("{}", msg)
    }
    match person2 {
        Ok(adult) => println!("Adult, name – {}... age – {}.", adult.name, adult.age),
        Err(msg) => println!("{}", msg)
    }
}
