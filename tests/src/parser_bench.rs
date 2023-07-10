use colored::Colorize;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{fs, fs::metadata};
use walkdir::WalkDir;

use std::fs::File;
use std::io::BufReader;
use std::panic;

fn all_lib_files() -> Vec<PathBuf> {
  WalkDir::new("tech")
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
    .collect::<Vec<PathBuf>>()
}

#[derive(Default, Clone, Copy, Debug)]
enum ReturnState {
  PASS(Duration),
  #[default]
  FAIL,
  PANIC,
}

impl Display for ReturnState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ReturnState::PASS(d) => write!(f, "{}", format!("{:.2?}", d).green().bold()),
      ReturnState::FAIL => write!(f, "{}", "FAIL".bold().bright_red()),
      ReturnState::PANIC => write!(f, "{}", "PANIC".red().bold()),
    }
  }
}

#[derive(Default, Debug)]
struct TestResult {
  file_in: String,
  state: ReturnState,
  // duration: Duration,
}
impl Display for TestResult {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}\t{}", self.state, self.file_in)
  }
}

type ParserFn = fn(PathBuf) -> Result<(), std::fmt::Error>;
struct ParserCtx {
  name: &'static str,
  info: &'static str,
  parser: ParserFn,
}

impl Display for ParserCtx {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // write!(f,
    //     "\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
    //     self.info, self.name,
    // )
    write!(f, "{}", self.name,)
  }
}

fn parse_lib_files(lib_files: Vec<PathBuf>, parser: ParserFn) -> Vec<TestResult> {
  lib_files
    .iter()
    .map(|filepath| {
      let bgn = Instant::now();
      let mut out = TestResult::default();
      out.file_in = String::from(filepath.to_str().unwrap());
      let panic_result = panic::catch_unwind(|| {
        let result = parser(filepath.to_path_buf());
        return (result, bgn.elapsed());
      });
      match panic_result {
        Ok((r, d)) => match r {
          Ok(_) => {
            out.state = ReturnState::PASS(d);
          }
          Err(_) => {
            out.state = ReturnState::FAIL;
          }
        },
        Err(_) => out.state = ReturnState::PANIC,
      }
      println!("{}", out);
      out
    })
    .collect::<Vec<TestResult>>()
}

const PARSER_LIBERTY_DB: ParserCtx = ParserCtx {
  name: "liberty-db",
  info: "https://crates.io/crates/liberty-db",
  parser: |filepath| {
    let data = fs::read_to_string(filepath.clone()).expect("Failed to open file.");
    let result = liberty_db::library::Library::parse(&data);
    match result {
      Ok(_) => Ok(()),
      Err(_) => Err(std::fmt::Error),
    }
  },
};

const PARSER_LIBERTY_IO: ParserCtx = ParserCtx {
  name: "liberty-io",
  info: "https://crates.io/crates/liberty-io",
  parser: |filepath| {
    let f = File::open(filepath).expect("Failed to open file.");
    let mut buf = BufReader::new(f);
    let result = liberty_io::read_liberty_bytes(&mut buf);
    match result {
      Ok(_) => Ok(()),
      Err(_) => Err(std::fmt::Error),
    }
  },
};

const PARSER_LIBERTYPARSE: ParserCtx = ParserCtx {
  name: "libertyparse",
  info: "https://crates.io/crates/libertyparse",
  parser: |filepath| {
    use libertyparse::*;
    let data = fs::read_to_string(filepath.clone()).expect("Failed to open file.");
    let parsed = Liberty::parse_str(&data);
    match parsed {
      Ok(_) => Ok(()),
      Err(_) => Err(std::fmt::Error),
    }
  },
};

// #[test]
pub fn test_all_lib_files() {
  use prettytable::{Cell, Row, Table};
  let all_parser: Vec<ParserCtx> =
    vec![PARSER_LIBERTY_DB, PARSER_LIBERTY_IO, PARSER_LIBERTYPARSE];
  let results: Vec<Vec<TestResult>> = all_parser
    .iter()
    .map(|ctx| {
      println!("");
      println!("{}:", ctx.name);
      parse_lib_files(all_lib_files(), ctx.parser)
    })
    .collect();
  let mut table = Table::new();
  let mut title = Row::from(all_parser);
  title.add_cell(Cell::new("Test Liberty File"));
  table.set_titles(title);
  for (file_idx, r) in results[0].iter().enumerate() {
    let row = table.add_row(Row::from(
      results
        .iter()
        .map(|result| result[file_idx].state)
        .collect::<Vec<ReturnState>>(),
    ));
    row.add_cell(Cell::new(&r.file_in));
  }
  println!("{}", table);
}
