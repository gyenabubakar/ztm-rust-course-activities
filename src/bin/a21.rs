// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    let u = find_user("katie").map(|user_id| User {
        user_id,
        name: "katie".to_owned(),
    });
    match u {
        Some(user) => println!("{user:?}"),
        None => println!("not found")
    }

    let u2 = find_user("joseph").map(|user_id| User{
        user_id,
        name: "joseph".to_owned()
    });
    match u2 {
        Some(user) => println!("{user:?}"),
        None => println!("not found")
    }
}
