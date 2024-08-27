#![allow(clippy::non_ascii_literal)]
use prettytable::format::{FormatBuilder, LinePosition, LineSeparator, TableFormat};

lazy_static::lazy_static! {
/// A table with delimiters made with box characters
///
/// Reference: [FORMAT_BOX_CHARS](prettytable::format::consts::FORMAT_BOX_CHARS)
///
/// # Example
/// ```text
///  A │ B │ !(A&B)
/// ───┼───┼───────
///  1 │ Z │ X
///  1 │ X │ X
///  1 │ R │ F
///  1 │ 1 │ 0
///  1 │ 0 │ 1
///  1 │ F │ R
/// ```
pub static ref FORMAT_NO_BORDER_BOX_CHARS: TableFormat = FormatBuilder::new()
    .column_separator('│')
    .separators(&[LinePosition::Title],
      LineSeparator::new('─','┼','┼','┼'))
    .padding(1, 1)
    .build();
}

// https://github.com/mitsuhiko/similar/blob/main/examples/terminal-inline.rs
#[cfg(test)]
pub fn text_diff(old: &str, new: &str) {
  use console::{style, Style};
  use core::fmt;
  use similar::{ChangeTag, TextDiff};
  struct Line(Option<usize>);

  impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self.0 {
        None => write!(f, "    "),
        Some(idx) => write!(f, "{:<4}", idx + 1),
      }
    }
  }
  let diff = TextDiff::from_lines(old, new);
  let mut has_diff = false;
  for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
    has_diff = true;
    if idx > 0 {
      println!("{:-^1$}", "-", 80);
    }
    for op in group {
      for change in diff.iter_inline_changes(op) {
        let (sign, s) = match change.tag() {
          ChangeTag::Delete => ("-", Style::new().red()),
          ChangeTag::Insert => ("+", Style::new().green()),
          ChangeTag::Equal => (" ", Style::new().dim()),
        };
        print!(
          "{}{} |{}",
          style(Line(change.old_index())).dim(),
          style(Line(change.new_index())).dim(),
          s.apply_to(sign).bold(),
        );
        for (emphasized, value) in change.iter_strings_lossy() {
          if emphasized {
            print!("{}", s.apply_to(value).underlined().on_black());
          } else {
            print!("{}", s.apply_to(value));
          }
        }
        if change.missing_newline() {
          println!();
        }
      }
    }
  }
  assert!(!has_diff, "has different!");
}
