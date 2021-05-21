use unitest::testing::{must, test, unit};

/// returns `true` if this char is a whitespace
#[inline]
pub fn is_whitespace(c: char) -> bool {
  match c {
    | '\u{0009}' // \t
    | '\u{000A}' // \n
    | '\u{000B}' // vertical tab
    | '\u{000C}' // form feed
    | '\u{000D}' // \r
    | '\u{0020}' // space
    | '\u{0085}' // next line from latin1
    | '\u{200E}' // left-to-right mark
    | '\u{200F}' // right-to-left mark
    | '\u{2028}' // line seprarator
    | '\u{2029}' // paragraph seprarator
    => true,
    _ => false,
  }
}

unit!(
  test!(
    should_be_true_if_whitespace,
    must!(truthy: is_whitespace(' '))
  );
  test!(
    should_be_false_if_whitespace,
    must!(falsy: is_whitespace('-'))
  );
);
