use std::fmt::Write;

use crate::charcase;

use unitest::testing::{must, test, unit};

/// returns true if snakecase
#[inline]
pub fn is_snakecase(s: &str) -> bool {
  match s.chars().next() {
    Some(c) => charcase::lowercase::is_lowercase(c),
    None => false,
  }
}

/// snakecased a string ref
#[inline]
pub fn to_snakecase(s: &str) -> String {
  let mut buf = String::with_capacity(s.len());

  s.char_indices().for_each(|(i, c)| {
    if charcase::uppercase::is_uppercase(c) {
      if i != 0 {
        write!(buf, "_").ok();
      }

      write!(buf, "{}", charcase::lowercase::to_lowercase(c)).ok();
    } else {
      write!(buf, "{}", c).ok();
    }
  });

  buf
}

unit!(
  test!(
    should_returns_true_if_snakecase,
    must!(truthy: is_snakecase(&String::from("hello_world")))
  );
  test!(
    should_returns_false_if_not_snakecase,
    must!(falsy: is_snakecase(&String::from("Hello")))
  );
  test!(
    should_be_snakecased,
    must!(eq: to_snakecase("helloWorld"), String::from("hello_world"))
  );
);
