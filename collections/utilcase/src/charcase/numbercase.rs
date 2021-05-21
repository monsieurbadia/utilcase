use unitest::testing::{must, test, unit};

/// returns `true` if this char is a number
#[inline]
pub fn is_number(c: char) -> bool {
  c.is_digit(10)
}

/// returns `true` if this char is a number zero
#[inline]
pub fn is_number_zero(c: char) -> bool {
  c == '0'
}

/// returns `true` if this char is a number continue (1 to 0)
#[inline]
pub fn is_number_continue(c: char) -> bool {
  match c {
    '1' ..= '9' => true,
    _ => false,
  }
}

/// returns `true` if this char is a hex number
#[inline]
pub fn is_number_hex(c: char) -> bool {
  match c {
    '0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F' => true,
    _ => false,
  }
}

unit!(
  test!(
    should_be_true_if_number,
    must!(truthy: is_number('9'))
  );
  test!(
    should_be_false_if_number,
    must!(falsy: is_number('*'))
  );
  test!(
    should_be_true_if_number_zero,
    must!(truthy: is_number_zero('0'))
  );
  test!(
    should_be_false_if_number_zero,
    must!(falsy: is_number_zero('#'))
  );
  test!(
    should_be_true_if_number_continue,
    must!(truthy: is_number_continue('7'))
  );
  test!(
    should_be_false_if_number_continue,
    must!(falsy: is_number_continue('%'))
  );
  test!(
    should_be_true_if_number_hex,
    must!(truthy: is_number_hex('F'))
  );
  test!(
    should_be_false_if_number_hex,
    must!(falsy: is_number_hex('$'))
  );
);
