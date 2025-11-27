use core::fmt;

/// See more at [`DefaultIndentation`]
pub trait Indentation {
  fn new() -> Self;
  fn indent(&mut self);
  fn dedent(&mut self);
  fn indentation(&self) -> &str;
}

/// The Default Indentation implemt
#[derive(Debug)]
pub struct DefaultIndentation(String);
impl Indentation for DefaultIndentation {
  #[inline]
  fn new() -> Self {
    Self(String::with_capacity(10))
  }
  #[inline]
  fn indent(&mut self) {
    #[cfg(test)]
    {
      self.0 += "| ";
    }
    #[expect(clippy::cfg_not_test)]
    #[cfg(not(test))]
    {
      self.0.push('\t');
    }
  }
  #[inline]
  fn dedent(&mut self) {
    #[cfg(test)]
    {
      _ = self.0.pop();
      _ = self.0.pop();
    }
    #[expect(clippy::cfg_not_test)]
    #[cfg(not(test))]
    {
      _ = self.0.pop();
    }
  }
  #[inline]
  fn indentation(&self) -> &str {
    &self.0
  }
}
/// `CodeFormatter` with indent
#[expect(missing_debug_implementations)]
pub struct CodeFormatter<'a, F, I> {
  f: &'a mut F,
  indent: I,
  buffer: [u8; lexical_core::BUFFER_SIZE],
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
      indent: I::new(),
      buffer: [b'0'; lexical_core::BUFFER_SIZE],
    }
  }
  #[inline]
  #[expect(clippy::unwrap_used, clippy::unwrap_in_result)]
  pub(crate) fn write_num<N: lexical_core::ToLexical>(&mut self, n: N) -> fmt::Result {
    let bytes = lexical_core::write(n, &mut self.buffer);
    let s = core::str::from_utf8(bytes).unwrap();
    self.f.write_str(s)
  }

  /// Increase the indentation level by `inc`
  #[inline]
  pub fn indent(&mut self) {
    self.indent.indent();
  }

  /// Decrease the indentation level by `inc`
  #[inline]
  pub fn dedent(&mut self) {
    self.indent.dedent();
  }

  /// Get indentation
  #[inline]
  pub fn write_new_line_indentation(&mut self) -> fmt::Result {
    self.f.write_char('\n')?;
    self.f.write_str(self.indent.indentation())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::{DefaultCtx, common::items::DummyGroup};
  #[test]
  fn more_than_10_indent() {
    crate::ast::test_parse_fmt::<DummyGroup<DefaultCtx>>(
      r#"(0){
        * comment1
        level(1){
          * comment2
          level(2){
            * comment3
            level(3){
              * comment4
              level(4){
                * comment5
                level(5){
                  * comment6
                  level(6){
                    * comment7
                    level(7){
                      * comment8
                      level(8){
                        * comment9
                        level(9){
                          * comment10
                          level(10){
                            * comment11
                            level(11){
                              * comment12
                              level(12){
                                * comment13
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
| | | | | | | | | | level (10) {  /* user defined attribute */
| | | | | | | | | | | level (11) {  /* user defined attribute */
| | | | | | | | | | | | level (12) {  /* user defined attribute */
| | | | | | | | | | | | | level (13) {  /* user defined attribute */
| | | | | | | | | | | | | }
| | | | | | | | | | | | }
| | | | | | | | | | | }
| | | | | | | | | | }
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
    crate::ast::test_parse_fmt::<DummyGroup<DefaultCtx>>(
      r#"(){
        unknown_complex (1,2,3,4,5);
        unknown_complex (1,2,3, \
          4,5);
        unknown_complex (1,2,3,4,5);
        unknown_complex (1,2,\
          3, 4,5);
      }"#,
      r#"
liberty_db::common::items::DummyGroup () {
| unknown_complex (1, 2, 3, 4, 5); /* user defined attribute */
| unknown_complex (1, 2, 3, 4, 5); /* user defined attribute */
| unknown_complex (1, 2, 3, 4, 5); /* user defined attribute */
| unknown_complex (1, 2, 3, 4, 5); /* user defined attribute */
}"#,
    );
  }
}
