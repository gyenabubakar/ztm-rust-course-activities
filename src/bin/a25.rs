// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
  fn get_perimeter(&self) -> f64;
}

struct Square {
  side: f64,
}

impl Perimeter for Square {
  fn get_perimeter(&self) -> f64 {
    self.side * 4.0
  }
}

struct Triangle {
  side_a: f64,
  side_b: f64,
  side_c: f64,
}

impl Perimeter for Triangle {
  fn get_perimeter(&self) -> f64 {
    self.side_a + self.side_b + self.side_c
  }
}

fn print_perimeter_of(shape: &impl Perimeter, _type: &str) {
  println!("Perimeter of {:?} is {}", _type, shape.get_perimeter())
}

fn main() {
  let triangle = Triangle {
    side_a: 10.0,
    side_b: 13.5,
    side_c: 20.0,
  };
  let square = Square { side: 45.0 };

  print_perimeter_of(&triangle, "Triangle");
  print_perimeter_of(&square, "Square");
}
