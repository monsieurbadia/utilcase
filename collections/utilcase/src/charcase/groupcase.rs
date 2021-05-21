use unitest::testing::{must, test, unit};

/// returns true if is a group char
#[inline]
pub fn is_group(c: char) -> bool {
  match c {
    '[' | ']' | '(' | ')' | '{' | '}' => true,
    _ => false,
  }
}

unit!(
  test!(should_be_true_if_group, must!(truthy: is_group('[')));
  test!(should_be_false_if_group, must!(falsy: is_group('\0')));
);
