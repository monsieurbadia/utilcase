use unitest::testing::{must, test, unit};

/// returns `true` if this char is an underscore
#[inline]
pub fn is_underscore(c: char) -> bool {
  c == '\u{005F}'
}

unit!(
  test!(
    should_be_true_if_underscore,
    must!(truthy: is_underscore('_'))
  );
  test!(
    should_be_false_if_underscore,
    must!(falsy: is_underscore('/'))
  );
);
