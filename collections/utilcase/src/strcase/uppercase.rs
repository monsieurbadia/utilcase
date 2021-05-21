use std::fmt::Write;

use crate::charcase;

use unitest::testing::{must, test, unit};

/// is_uppercase

/// uppercased a string ref
#[inline]
pub fn to_uppercase(s: &str) -> String {
  let mut buf = String::with_capacity(s.len());

  s.chars().for_each(|c| {
    write!(buf, "{}", charcase::uppercase::to_uppercase(c)).ok();
  });

  buf
}

/// transforms the first letter of a string ref to uppercase
#[inline]
pub fn to_uppercase_first(s: &str) -> String {
  let mut buf = String::with_capacity(s.len());

  if s.is_empty() {
    return buf;
  }

  let (head, tail) = s.split_at(1);
  let head = to_uppercase(head);

  write!(buf, "{}{}", head, tail).ok();

  buf
}

unit!(
  test!(
    should_be_uppercased,
    must!(eq: to_uppercase("hello"), String::from("HELLO"))
  );
  test!(
    should_be_uppercased_first,
    must!(eq: to_uppercase_first("hello"), String::from("Hello"))
  );
);
