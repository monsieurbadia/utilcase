use super::underscorecase::is_underscore;

use unitest::testing::{must, test, unit};

/// returns `true` if this char is an identifier
#[inline]
pub fn is_ident(c: char) -> bool {
  c.is_alphabetic() || is_underscore(c)
}

unit!(
  test!(should_be_true_if_ident, must!(truthy: is_ident('e')));
  test!(should_be_false_if_ident, must!(falsy: is_ident('3')));
);
