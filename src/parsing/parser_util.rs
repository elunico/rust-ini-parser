#[derive(Debug)]
pub struct ParserLocation {
  pub pos: usize,
  pub line: usize,
  pub col: usize,
}

pub fn inc_char(loc: &mut ParserLocation) {
  loc.pos += 1;
  loc.col += 1;
}

pub fn inc_line(loc: &mut ParserLocation) {
  loc.pos += 1;
  loc.col = 1;
  loc.line += 1;
}
