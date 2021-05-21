use unitest::testing::{must, test, unit};

/// returns `true` if this char is a end of file
#[inline]
pub fn is_end_of_file(c: char) -> bool {
  c == '\u{0}'
}

/// returns `true` if this char is a end of line
#[inline]
pub fn is_end_of_line(c: char) -> bool {
  c == '\u{000A}'
}

unit!(
  test!(should_be_true_if_eof, must!(truthy: is_end_of_file('\0')));
  test!(should_be_false_if_eof, must!(falsy: is_end_of_file(' ')));
  test!(should_be_true_if_eol, must!(truthy: is_end_of_line('\n')));
  test!(should_be_false_if_eol, must!(falsy: is_end_of_line('u')));
);
