use crate::ArcStr;
use std::fmt::Write;

const N: usize = 10;
/// CodeFormatter with indent
#[derive(Debug)]
pub struct CodeFormatter<'a, F> {
  f: &'a mut F,
  level: usize,
  // repeat: ArcStr,
  indentation_lut: [ArcStr; N],
}

impl<F: Write> Write for CodeFormatter<'_, F> {
  #[inline]
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    write!(
      self.f,
      "{}",
      s.replace(
        '\n',
        format!(
          "\n{}",
          if self.level >= N {
            &self.indentation_lut[N - 1]
          } else {
            &self.indentation_lut[self.level]
          }
        )
        .as_str()
      )
    )
  }
}

impl<'a, T: Write> CodeFormatter<'a, T> {
  /// Wrap the formatter `f`, use `indentation` as base string indentation and return a new
  /// formatter that implements `std::fmt::Write` that can be used with the macro `write!()`
  #[inline]
  pub fn new<S: Into<ArcStr>>(f: &'a mut T, indentation: S) -> Self {
    let indentation: ArcStr = indentation.into();
    Self {
      f,
      level: 0,
      indentation_lut: [
        indentation.repeat(0).into(),
        indentation.repeat(1).into(),
        indentation.repeat(2).into(),
        indentation.repeat(3).into(),
        indentation.repeat(4).into(),
        indentation.repeat(5).into(),
        indentation.repeat(6).into(),
        indentation.repeat(7).into(),
        indentation.repeat(8).into(),
        indentation.repeat(9).into(),
      ],
      // (0..N)
      //   .into_iter()
      //   .map(|i| indentation.repeat(i).into())
      //   .collect::<Vec<_>>()
      //   .try_into()
      //   .unwrap(),
    }
  }

  /// Set the indentation level to a specific value
  #[inline]
  pub fn set_level(&mut self, level: usize) {
    self.level = level;
    // self.repeat = self.indentation.repeat(self.level);
  }

  /// Increase the indentation level by `inc`
  #[inline]
  pub fn indent(&mut self, inc: usize) {
    self.level = self.level.saturating_add(inc);
    // self.repeat += &self.indentation.repeat(inc);
  }

  /// Decrease the indentation level by `inc`
  #[inline]
  pub fn dedent(&mut self, inc: usize) {
    self.level = self.level.saturating_sub(inc);
    // self.repeat = self.indentation.repeat(self.level);
  }
}
