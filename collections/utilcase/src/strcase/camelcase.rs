use std::fmt::Write;

use crate::charcase;

use unitest::testing::{must, test, unit};

/// returns true if camelcase
#[inline]
pub fn is_camelcase(s: &str) -> bool {
  match s.chars().next() {
    Some(c) => !charcase::uppercase::is_uppercase(c),
    None => !s.contains('_'),
  }
}

/// camelcased a s ref
#[inline]
pub fn to_camelcase(s: &str) -> String {
  let mut buf = String::with_capacity(s.len());
  let mut step_word = false;

  s.chars().for_each(|c| {
    if charcase::underscorecase::is_underscore(c) {
      step_word = true;
    } else if step_word {
      write!(buf, "{}", charcase::uppercase::to_uppercase(c)).ok();
      step_word = false;
    } else {
      write!(buf, "{}", c).ok();
    }
  });

  buf
}

unit!(
  test!(
    should_returns_true_if_camelcase,
    must!(truthy: is_camelcase(&String::from("hello")))
  );
  test!(
    should_returns_false_if_not_camelcase,
    must!(falsy: is_camelcase(&String::from("Hello")))
  );
  test!(
    should_be_camelcased,
    must!(eq: to_camelcase(&String::from("hello_ok")), String::from("helloOk"))
  );
);
