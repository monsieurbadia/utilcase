use std::fmt::Write;

use crate::charcase;

use unitest::testing::{must, test, unit};

// TODO: is_lowercase(s: &str)

/// returns true if a string ref is lowercase
#[inline]
pub fn to_lowercase(s: &str) -> String {
  let mut buf = String::with_capacity(s.len());

  s.chars().for_each(|c| {
    write!(buf, "{}", charcase::lowercase::to_lowercase(c)).ok();
  });

  buf
}

/// transform the first letter of a string ref to lowercase
#[inline]
pub fn to_lowercase_first(s: &str) -> String {
  let mut buf = String::with_capacity(s.len());

  if s.is_empty() {
    return buf;
  }

  let (head, tail) = s.split_at(1);
  let head = to_lowercase(head);

  write!(buf, "{}{}", head, tail).ok();

  buf
}

unit!(
  test!(
    should_be_lowercased,
    must!(eq: to_lowercase("HELLO"), String::from("hello"))
  );
  test!(
    should_be_lowercased_first,
    must!(eq: to_lowercase_first("Hello"), String::from("hello"))
  );
);
