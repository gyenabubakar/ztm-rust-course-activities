// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker_assignment: Option<i32>,
}

fn main() {
    let students = vec![
        Student {
            name: "Gyen".to_owned(),
            locker_assignment: Some(30),
        },
        Student {
            name: "Kwame".to_owned(),
            locker_assignment: None,
        },
    ];

    for stud in students {
        println!("Student name: {}", stud.name);
        print!("Locker assignment: ");
        match stud.locker_assignment {
            Some(value) => println!("{}\n", value),
            None => println!("Has none\n")
        }
    }
}
