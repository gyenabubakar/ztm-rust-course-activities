// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Standard(f32),
    Backstage(f32, String),
    VIP(f32, String),
}

fn main() {
    let tickets = vec![
        Ticket::Standard(99.99),
        Ticket::Backstage(199.99, String::from("Kofi")),
        Ticket::VIP(499.99, String::from("Ben")),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Standard(p) => println!("Standard @ £{}", p),
            Ticket::Backstage(p, h) => println!("Backstage @ £{}, holder: {}", p, h),
            Ticket::VIP(p, h) => println!("VIP @ £{}, holder: {}", p, h),
        }
    }
}
