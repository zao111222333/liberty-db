use core::{fmt, marker::PhantomData, ops::Deref};

/// See more at [`DefaultIndentation`]
pub trait Indentation {
  const LUT: &'static [&'static str];
  const LAST_LINE: &'static str = match Self::LUT.last() {
    Some(s) => s,
    None => "",
  };
  /// Get indentation
  #[must_use]
  #[inline]
  fn indentation(level: usize) -> &'static str {
    Self::LUT.get(level).map_or(Self::LAST_LINE, Deref::deref)
  }
}

/// The Default Indentation implemt
#[derive(Debug, Clone, Copy)]
pub struct DefaultIndentation;
impl Indentation for DefaultIndentation {
  cfg_if::cfg_if! {
    if #[cfg(test)]{
      const LUT: &'static [&'static str] =  &[
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
    }else{
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
  }
}
/// `CodeFormatter` with indent
#[expect(missing_debug_implementations)]
pub struct CodeFormatter<'a, F, I> {
  f: &'a mut F,
  level: usize,
  buff_f: ryu::Buffer,
  buff_i: itoa::Buffer,
  __i: PhantomData<I>,
}

pub type DefaultCodeFormatter<'a, F> = CodeFormatter<'a, F, DefaultIndentation>;
impl<F: fmt::Write, I> fmt::Write for CodeFormatter<'_, F, I> {
  #[inline]
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.f.write_str(s)
  }
}

impl<'a, T: fmt::Write, I: Indentation> CodeFormatter<'a, T, I> {
  /// Wrap the formatter `f`, use `indentation` as base string indentation and return a new
  /// formatter that implements `fmt::Write` that can be used with the macro `write!()`
  #[inline]
  pub fn new(f: &'a mut T) -> Self {
    Self {
      f,
      level: 0,
      buff_f: ryu::Buffer::new(),
      buff_i: itoa::Buffer::new(),
      __i: PhantomData,
    }
  }
  #[inline]
  pub(crate) fn write_float<F: ryu::Float>(&mut self, float: F) -> fmt::Result {
    let s = self.buff_f.format(float);
    self.f.write_str(s)
  }
  #[inline]
  pub(crate) fn write_int<Int: itoa::Integer>(&mut self, int: Int) -> fmt::Result {
    let s = self.buff_i.format(int);
    self.f.write_str(s)
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
  fn more_than_10_indent() {
    crate::ast::test_parse_fmt::<DummyGroup>(
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
      r#"
liberty_db::common::items::DummyGroup (0) {
| level (1) {  /* user defined attribute */
| | level (2) {  /* user defined attribute */
| | | level (3) {  /* user defined attribute */
| | | | level (4) {  /* user defined attribute */
| | | | | level (5) {  /* user defined attribute */
| | | | | | level (6) {  /* user defined attribute */
| | | | | | | level (7) {  /* user defined attribute */
| | | | | | | | level (8) {  /* user defined attribute */
| | | | | | | | | level (9) {  /* user defined attribute */
| | | | | | | | | level (10) {  /* user defined attribute */
| | | | | | | | | level (11) {  /* user defined attribute */
| | | | | | | | | level (12) {  /* user defined attribute */
| | | | | | | | | level (13) {  /* user defined attribute */
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
}"#,
    );
  }
  #[test]
  fn unknown_complex() {
    crate::ast::test_parse_fmt::<DummyGroup>(
      r#"(){
        unknown_complex (1,2,3,4,5);
        unknown_complex (1,2,3, \
          4,5);
        unknown_complex ("1,2",3,4,5);
        unknown_complex (1,2,\
          3, "4,5");
      }"#,
      r#"
liberty_db::common::items::DummyGroup () {
| unknown_complex (1, 2, 3, 4, 5); /* user defined attribute */
| unknown_complex ("1, 2, 3", \
| "4, 5");  /* user defined attribute */
| unknown_complex (1, 2, 3, 4, 5); /* user defined attribute */
| unknown_complex ("1, 2", \
| "3, 4, 5");  /* user defined attribute */
}"#,
    );
  }
}
