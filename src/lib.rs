use std::io::{stdin, stdout, Write};
use std::str::FromStr;

#[doc(hidden)]
pub fn get__<F: FromStr>() -> Result<F, <F as FromStr>::Err> {
  let mut s = String::new();
  let _ = stdout().flush();

  stdin()
    .read_line(&mut s)
    .expect("user did not enter correct string");

  if let Some('\n') = s.chars().next_back() {
    s.pop();
  }

  if let Some('\r') = s.chars().next_back() {
    s.pop();
  }

  s.parse::<F>()
}

#[doc(hidden)]
pub fn set__<F: FromStr>(value: &mut F) {
  if let Ok(i) = get__::<F>() {
    *value = i;
  }
}

#[macro_export]
macro_rules! get_input {
  ($generic: ty) => {
    $crate::get__::<$generic>()
  };

  ($generic: ty, $($arg:tt)*) => {{
    print!("{}",format_args!($($arg)*));
    $crate::get__::<$generic>()
  }};
}

#[macro_export]
macro_rules! set_from_input {
  ($value: expr) => {
    $crate::set__($value)
  };

  ($value: expr, $($arg:tt)*) => {{
    print!("{}",format_args!($($arg)*));
    $crate::set__($value)
  }};
}
