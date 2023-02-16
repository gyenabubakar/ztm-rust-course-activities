#![allow(dead_code)]

// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod a22 {
    use crate::*;

    #[test]
    fn clamp_returns_n() {
        let result = clamp(100, 100, 200);
        let expected = 100;
        assert_eq!(result, expected, "should be 100")
    }

    #[test]
    fn clamp_returns_lower() {
        let result = clamp(100, 150, 200);
        let expected = 150;
        assert_eq!(result, expected, "should be 150")
    }

    #[test]
    fn clamp_returns_upper() {
        let result = clamp(100, 50, 80);
        let expected = 80;
        assert_eq!(result, expected, "should be 80")
    }

    #[test]
    fn div_returns_2() {
        let result = div(4, 2);
        let expected = 2;
        assert_eq!(result.unwrap(), expected, "should be 2")
    }

    #[test]
    fn div_returns_none() {
        let result = div(4, 0);
        let expected: Option<i32> = None;
        assert_eq!(result, expected, "should be none")
    }

    #[test]
    fn concat_works() {
        let result = concat("Hello", "World");
        let expected = "HelloWorld";
        assert_eq!(result, expected, "should be HelloWorld")
    }
}