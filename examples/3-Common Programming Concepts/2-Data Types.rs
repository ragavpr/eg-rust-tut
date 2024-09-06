fn main() {
  let i = 5; //i32

  let x = 2.0; // f64
  let y: f32 = 3.0; // f32

  let t = true;
  let f: bool = false; // with explicit type annotation

  let c = 'z';
  let z: char = 'ℤ'; // with explicit type annotation
  let heart_eyed_cat = '😻';

  // Compound types
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("The value of y is: {y}");

  let five_hundred = tup.0;
  let six_point_four = tup.1;
  let one = tup.2;

  let a = [1, 2, 3, 4, 5];
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  let a: [i32; 5] = [3; 5]; // [3, 3, 3, 3, 3]

  // addition
  let sum = 5 + 10;

  // subtraction
  let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;
  println!("The value of divison is: {quotient}");
  let truncated = -5 / 3; // Results in -1
  println!("The value of truncated division is: {truncated}");

  // remainder
  let remainder = 43 % 5;


  println!("Please enter an array index.");

  let mut index = String::new();

  std::io::stdin()
      .read_line(&mut index)
      .expect("Failed to read line");

  let index: usize = index
      .trim()
      .parse()
      .expect("Index entered was not a number");

  let element = a[index];

  println!("The value of the element at index {index} is: {element}");
}