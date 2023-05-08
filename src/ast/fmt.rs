use std::fmt::Write;
#[derive(Debug)]
pub struct CodeFormatter<'a, F> {
  f: &'a mut F,
  level: usize,
  repeat: String,
  indentation: String,
}

impl<'a,F: Write> Write for CodeFormatter<'a,F> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        write!(self.f,
            "{}", s.replace('\n', format!("\n{}",self.repeat).as_str())
        )
    }
}

impl<'a, T: Write> CodeFormatter<'a, T> {
    /// Wrap the formatter `f`, use `indentation` as base string indentation and return a new
    /// formatter that implements `std::fmt::Write` that can be used with the macro `write!()`
    pub fn new<S: Into<String>>(f: &'a mut T, indentation: S) -> Self {
        Self {
            f,
            level: 0,
            indentation: indentation.into(),
            repeat: "".into(),
        }
    }

    /// Set the indentation level to a specific value
    pub fn set_level(&mut self, level: usize) {
        self.level = level;
        self.repeat = self.indentation.repeat(self.level);
    }

    /// Increase the indentation level by `inc`
    pub fn indent(&mut self, inc: usize) {
        self.level = self.level.saturating_add(inc);
        self.repeat += &self.indentation.repeat(inc);
    }

    /// Decrease the indentation level by `inc`
    pub fn dedent(&mut self, inc: usize) {
        self.level = self.level.saturating_sub(inc);
        self.repeat = self.indentation.repeat(self.level);
    }
}