use core::fmt::Write;
use core::marker::PhantomData;

pub trait Indentation {
  const LUT: &'static [&'static str];

  #[allow(clippy::indexing_slicing)]
  #[allow(clippy::arithmetic_side_effects)]
  #[must_use]
  #[inline]
  fn indentation(level: usize) -> &'static str {
    if let Some(&i1) = Self::LUT.get(level) {
      i1
    } else if let Some(&i2) = Self::LUT.last() {
      i2
    } else {
      ""
    }
  }
}
#[derive(Debug, Clone, Copy)]
pub struct DefaultIndentation;
impl Indentation for DefaultIndentation {
  const LUT: &'static [&'static str] = &[
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
}
#[derive(Debug, Clone, Copy)]
pub struct TestIndentation;
impl Indentation for TestIndentation {
  const LUT: &'static [&'static str] = &[
    "",
    "| ",
    "| | ",
    "| | | ",
    "| | | | ",
    "| | | | | ",
    "| | | | | | ",
    "| | | | | | | ",
    "| | | | | | | | ",
    "| | | | | | | | | ",
  ];
}
/// `CodeFormatter` with indent
#[derive(Debug)]
pub struct CodeFormatter<'a, F, I> {
  f: &'a mut F,
  level: usize,
  __i: PhantomData<I>,
}

pub type DefaultCodeFormatter<'a, F> = CodeFormatter<'a, F, DefaultIndentation>;
pub type TestCodeFormatter<'a, F> = CodeFormatter<'a, F, TestIndentation>;

impl<F: Write, I> Write for CodeFormatter<'_, F, I> {
  #[inline]
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    self.f.write_str(s)
  }
}

impl<'a, T: Write, I: Indentation> CodeFormatter<'a, T, I> {
  /// Wrap the formatter `f`, use `indentation` as base string indentation and return a new
  /// formatter that implements `core::fmt::Write` that can be used with the macro `write!()`
  #[inline]
  pub fn new(f: &'a mut T) -> Self {
    Self { f, level: 0, __i: PhantomData }
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

  /// Get indentation
  #[must_use]
  #[inline]
  pub fn indentation(&self) -> &'static str {
    I::indentation(self.level)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::common::items::DummyGroup;
  #[test]
  fn more_than_10() {
    let (_, fmt_str, _) = crate::ast::test_parse_group::<DummyGroup>(
      r#"(0){
        /* comment1 */
        level(1){
          /* comment2 */
          level(2){
            /* comment3 */
            level(3){
              /* comment4 */
              level(4){
                /* comment5 */
                level(5){
                  /* comment6 */
                  level(6){
                    /* comment7 */
                    level(7){
                      /* comment8 */
                      level(8){
                        /* comment9 */
                        level(9){
                          /* comment10 */
                          level(10){
                            /* comment11 */
                            level(11){
                              /* comment12 */
                              level(12){
                                /* comment13 */
                                level(13){
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }"#,
    );
    assert_eq!(
      fmt_str,
      r#"
liberty_db::common::items::DummyGroup (0) {
| /* Undefined attributes from here */
| level (1) {
| | level (2) {
| | | level (3) {
| | | | level (4) {
| | | | | level (5) {
| | | | | | level (6) {
| | | | | | | level (7) {
| | | | | | | | level (8) {
| | | | | | | | | level (9) {
| | | | | | | | | level (10) {
| | | | | | | | | level (11) {
| | | | | | | | | level (12) {
| | | | | | | | | level (13) {
| | | | | | | | | }
| | | | | | | | | }
| | | | | | | | | }
| | | | | | | | | }
| | | | | | | | | }
| | | | | | | | }
| | | | | | | }
| | | | | | }
| | | | | }
| | | | }
| | | }
| | }
| }
| /* Undefined attributes end here */
}"#
    );
  }
}
