// Topic: Iterator
//
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.
//
// Notes:
// * Use an iterator chain to accomplish the task.

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let over_10: Vec<_> = data.iter()
        .map(|n| n * 3)
        .filter(|n| n > &10)
        .collect();

    for num in over_10 {
        println!("{num}")
    }
}
