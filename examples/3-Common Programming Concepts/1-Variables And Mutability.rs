fn main() {
  let x = 5;
  println!("The value of x is: {x}");
  
  let x = x + 1; //Allowed because x is shadowed
  {
      let x = x * 2;
      println!("The value of x in the inner scope is: {x}");
  }
  println!("The value of x is: {x}");

  x = 6; // This will cause an error because x is immutable
  println!("The value of x is: {x}");

  let mut y = 5;
  println!("The value of y is: {y}");
  y = 6;
  println!("The value of y after mutation is: {y}");
}