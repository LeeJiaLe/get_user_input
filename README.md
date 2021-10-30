# get_user_input

Simple library allow to get user input from console with few line of code,

Example usage:
```
use get_user_input::{get_input, set_from_input};
fn main() {
  let mut a = 0;
  let mut b = 1.0;
  let mut c = 2;
  let mut d = 3.0;

  print!("Enter value for A ({}):", a);
  //directly set from user input, nothing will set if user input invalid value
  set_from_input!(&mut a);

  //apply message by putting arguments after variable
  set_from_input!(&mut b, "Enter value for B ({}):", b);

  print!("Enter value for C ({}):", c);
  //do something when user input invalid value
  if let Ok(i) = get_input!(i32) {
    c = i;
  } else {
    print!("You MUST input valid vaue for C");
    return;
  }

  print!("Enter value for D ({}):", d);
  //apply message by putting arguments after type
  if let Ok(i) = get_input!(f64) {
    d = i;
  } else {
    print!("You MUST input valid vaue for D");
    return;
  }

  println!("{}", a);
  println!("{}", b);
  println!("{}", c);
  println!("{}", d);
}
```