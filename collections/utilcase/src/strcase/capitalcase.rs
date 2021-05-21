use std::fmt::Write;

use unitest::testing::{must, test, unit};

/// returns true if a string ref is capitalize
#[inline]
pub fn is_capitalcase(s: &str) -> bool {
  match s.chars().next() {
    Some(c) => c.is_uppercase(),
    None => false,
  }
}

/// capitalized a string ref
#[inline]
pub fn to_capitalcase(s: &str) -> String {
  let mut buf = String::with_capacity(s.len());

  s.char_indices().for_each(|(i, c)| {
    let c = if i == 0 { c.to_ascii_uppercase() } else { c };
    write!(buf, "{}", c).ok();
  });

  buf
}

unit!(
  test!(
    should_returns_true_if_capitalcase,
    must!(truthy: is_capitalcase(&String::from("Hello")))
  );
  test!(
    should_returns_false_if_not_capitalcase,
    must!(falsy: is_capitalcase(&String::from("hello")))
  );
  test!(
    should_be_capitalcased,
    must!(eq: to_capitalcase(&String::from("hello")), String::from("Hello"))
  );
);
