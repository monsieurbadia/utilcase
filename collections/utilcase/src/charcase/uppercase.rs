use unitest::testing::{must, test, unit};

/// returns `true` if this char is uppercase
#[inline]
pub fn is_uppercase(c: char) -> bool {
  match c {
    'A'..='Z' => true,
    _ => false,
  }
}

/// uppercases a char
#[inline]
pub fn to_uppercase(c: char) -> char {
  c.to_ascii_uppercase()
}

unit!(
  test!(
    should_be_true_if_uppercase,
    must!(truthy: is_uppercase('O'))
  );
  test!(
    should_be_false_if_uppercase,
    must!(falsy: is_uppercase('p'))
  );
  test!(
    should_be_uppercased,
    must!(eq: 'Z', to_uppercase('z'))
  );
);
