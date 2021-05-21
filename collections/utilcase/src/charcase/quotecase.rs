use unitest::testing::{must, test, unit};

/// returns `true` if this char is a quote
#[inline]
pub fn is_quote(c: char) -> bool {
  is_quote_double(c) || is_quote_single(c)
}

/// returns `true` if this char is a single quote
#[inline]
pub fn is_quote_single(c: char) -> bool {
  c == '\u{0027}'
}

/// returns `true` if this char is a double quote
#[inline]
pub fn is_quote_double(c: char) -> bool {
  c == '\u{0022}'
}

unit!(
  test!(
    should_be_true_if_quote,
    must!(truthy: is_quote('"'))
  );
  test!(
    should_be_false_if_quote,
    must!(falsy: is_quote('.'))
  );
  test!(
    should_be_true_if_quote_single,
    must!(truthy: is_quote_single('\''))
  );
  test!(
    should_be_false_if_quote_single,
    must!(falsy: is_quote_single('_'))
  );
  test!(
    should_be_true_if_quote_double,
    must!(truthy: is_quote_double('"'))
  );
  test!(
    should_be_false_if_quote_double,
    must!(falsy: is_quote_double(')'))
  );
);
