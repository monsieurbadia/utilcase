use unitest::testing::{must, test, unit};

/// returns `true` if this char is lowercase
#[inline]
pub fn is_lowercase(c: char) -> bool {
  match c {
    'a'..='z' => true,
    _ => false,
  }
}

/// lowercased a char
#[inline]
pub fn to_lowercase(c: char) -> char {
  c.to_ascii_lowercase()
}

unit!(
  test!(should_be_true_if_lowercase, must!(truthy: is_lowercase('e')));
  test!(should_be_false_if_lowercase, must!(falsy: is_lowercase('E')));
  test!(should_be_lowercased, must!(eq: 'e', to_lowercase('E')));
);
