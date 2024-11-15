#![cfg(test)]
use dev::all_files;
use dev_utils::text_diff;
use liberty_db_latest::{ast::Group, Library};
use std::{
  fs::read_to_string,
  path::{Path, PathBuf},
};

fn golden_path(test_lib_path: &Path) -> PathBuf {
  test_lib_path.with_file_name(
    test_lib_path
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .replace(".lib", ".golden.lib"),
  )
}

/// when we need to re-golden
/// cargo test --package dev --features __dbg -- regression make_golden
#[cfg(feature = "__dbg")]
#[allow(dead_code)]
#[test]
fn make_golden() {
  use std::{
    fs::File,
    io::{BufWriter, Write},
  };
  for test_lib_path in all_files() {
    let golden_lib_path = golden_path(&test_lib_path);
    let library =
      Library::parse_lib(read_to_string(test_lib_path).unwrap().as_str()).unwrap();
    let golden_lib = File::create(golden_lib_path).unwrap();
    let mut writer = BufWriter::new(golden_lib);
    _ = write!(writer, "{}", library.display());
  }
}

#[cfg(test)]
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
    text_diff(golden.as_str(), new.as_str());
  }
}
