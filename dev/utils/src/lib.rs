// https://github.com/mitsuhiko/similar/blob/main/examples/terminal-inline.rs
pub fn text_diff(old: &str, new: &str) {
  use console::{Style, style};
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

pub fn all_files(root: &str) -> impl Iterator<Item = (bool, std::path::PathBuf)> {
  walkdir::WalkDir::new(root).into_iter().filter_map(|res| {
    res.ok().and_then(|entry| {
      let path = entry.path();
      let file_name = path.file_name().unwrap().to_str().unwrap();
      let md = std::fs::metadata(path).unwrap();
      if md.is_file() && file_name.ends_with("lib") && !file_name.ends_with("golden.lib")
      {
        Some((
          !entry.file_name().to_string_lossy().starts_with("error"),
          entry.into_path(),
        ))
      } else {
        None
      }
    })
  })
}

pub fn init_logger() {
  #[cfg(not(feature = "tracing"))]
  simple_logger::SimpleLogger::new().init().unwrap();
  #[cfg(feature = "tracing")]
  {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
      // .with_ansi(colored::control::SHOULD_COLORIZE.should_colorize())
      .with_max_level(tracing::Level::DEBUG)
      .with_target(false)
      .with_file(true)
      .with_line_number(true)
      .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new("%FT%T".to_owned()))
      .finish();
    _ = tracing::subscriber::set_global_default(subscriber);
  }
}
