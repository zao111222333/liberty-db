#![cfg(test)]
use console::{style, Style};
use core::fmt;
use liberty_db::{ast::TestWrapper, Library};
use similar::{ChangeTag, TextDiff};
use std::{
  fs::{read_to_string, File},
  io::{BufWriter, Write},
  path::{Path, PathBuf},
};

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

// open `#[test]` only when we need to re-golden
#[test]
fn make_golden() {
  for test_lib_path in crate::all_lib_files() {
    let golden_lib_path = golden_path(&test_lib_path);
    let library =
      Library::parse_lib(read_to_string(test_lib_path).unwrap().as_str()).unwrap();
    let wrapper = TestWrapper { inner: library, line_count: 0 };
    let golden_lib = File::create(golden_lib_path).unwrap();
    let mut writer = BufWriter::new(golden_lib);
    write!(writer, "{wrapper}");
  }
}

#[test]
fn regression() {
  for test_lib_path in crate::all_lib_files() {
    println!("================\n{}", test_lib_path.display());
    let golden_lib_path = golden_path(&test_lib_path);
    let library =
      Library::parse_lib(read_to_string(test_lib_path).unwrap().as_str()).unwrap();
    let wrapper = TestWrapper { inner: library, line_count: 0 };
    let golden = read_to_string(golden_lib_path).unwrap();
    let new = wrapper.to_string();
    crate::text_diff(golden.as_str(), new.as_str());
  }
}
