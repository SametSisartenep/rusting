fn main() {
  let (x, y) = (5, 3);

  ///////////////
  if x == 5 {
    println!("x is five!");
  }

  ///////////////
  if y == 4 {
    println!("y is four!");
  } else {
    println!("y is not four :(");
  }

  ///////////////
  let y = if x == 5 { 10 } else { 15 };

  println!("The final value of y is: {}", y);
}
