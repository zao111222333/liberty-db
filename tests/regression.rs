#![cfg(test)]
use liberty_db::{ast::Group, Library};
use std::{
  ffi::OsStr,
  fs::{metadata, read_to_string, File},
  io::{BufWriter, Write},
  path::{Path, PathBuf},
};

fn all_files() -> impl Iterator<Item = PathBuf> {
  walkdir::WalkDir::new("tests/tech")
    .into_iter()
    .filter_map(|e| match e {
      Ok(entry) => {
        let path = entry.path();
        let extension = path.extension().and_then(OsStr::to_str);
        let md = metadata(path).unwrap();
        if md.is_file() && extension == Some("lib") {
          Some(entry.into_path())
        } else {
          None
        }
      }
      Err(_) => None,
    })
}

fn text_diff(old: &str, new: &str) {
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

fn golden_path(test_lib_path: &Path) -> PathBuf {
  test_lib_path.with_file_name(
    test_lib_path
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .replace(".lib", ".lib_golden"),
  )
}

#[allow(dead_code)]
// open `#[test]` only when we need to re-golden
// #[test]
fn make_golden() {
  for test_lib_path in all_files() {
    let golden_lib_path = golden_path(&test_lib_path);
    let library =
      Library::parse_lib(read_to_string(test_lib_path).unwrap().as_str()).unwrap();
    let golden_lib = File::create(golden_lib_path).unwrap();
    let mut writer = BufWriter::new(golden_lib);
    _ = write!(writer, "{}", library.display());
  }
}

#[test]
fn regression() {
  _ = simple_logger::SimpleLogger::new().init();
  for test_lib_path in all_files() {
    println!("================\n{}", test_lib_path.display());
    let golden_lib_path = golden_path(&test_lib_path);
    let library =
      Library::parse_lib(read_to_string(test_lib_path).unwrap().as_str()).unwrap();
    let golden = read_to_string(golden_lib_path).unwrap();
    let new = library.display().to_string();
    crate::text_diff(golden.as_str(), new.as_str());
  }
}
