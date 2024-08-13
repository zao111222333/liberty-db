use core::fmt::Write;

const N: usize = 10;
const INDENTATION_LUT: [&str; 10] = [
  "",
  "  ",
  "    ",
  "      ",
  "        ",
  "          ",
  "            ",
  "              ",
  "                ",
  "                  ",
];
/// CodeFormatter with indent
#[derive(Debug)]
pub struct CodeFormatter<'a, F> {
  f: &'a mut F,
  level: usize,
}

impl<F: Write> Write for CodeFormatter<'_, F> {
  #[inline]
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    write!(
      self.f,
      "{}",
      // TODO: donot use replace
      s.replace(
        '\n',
        format!(
          "\n{}",
          if self.level >= N {
            &INDENTATION_LUT[N - 1]
          } else {
            &INDENTATION_LUT[self.level]
          }
        )
        .as_str()
      )
    )
  }
}

impl<'a, T: Write> CodeFormatter<'a, T> {
  /// Wrap the formatter `f`, use `indentation` as base string indentation and return a new
  /// formatter that implements `core::fmt::Write` that can be used with the macro `write!()`
  #[inline]
  pub fn new(f: &'a mut T) -> Self {
    Self { f, level: 0 }
  }

  /// Set the indentation level to a specific value
  #[inline]
  pub fn set_level(&mut self, level: usize) {
    self.level = level;
  }

  /// Increase the indentation level by `inc`
  #[inline]
  pub fn indent(&mut self, inc: usize) {
    self.level = self.level.saturating_add(inc);
  }

  /// Decrease the indentation level by `inc`
  #[inline]
  pub fn dedent(&mut self, inc: usize) {
    self.level = self.level.saturating_sub(inc);
  }
}
