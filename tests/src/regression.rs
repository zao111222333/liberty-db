#![cfg(test)]
use liberty_db::{ast::Group, Library};
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
// #[test]
fn make_golden() {
  for test_lib_path in crate::all_lib_files() {
    let golden_lib_path = golden_path(&test_lib_path);
    let library =
      Library::parse_lib(read_to_string(test_lib_path).unwrap().as_str()).unwrap();
    let golden_lib = File::create(golden_lib_path).unwrap();
    let mut writer = BufWriter::new(golden_lib);
    write!(writer, "{}", library.display());
  }
}

#[test]
fn regression() {
  simple_logger::SimpleLogger::new().init();
  for test_lib_path in crate::all_lib_files() {
    println!("================\n{}", test_lib_path.display());
    let golden_lib_path = golden_path(&test_lib_path);
    let library =
      Library::parse_lib(read_to_string(test_lib_path).unwrap().as_str()).unwrap();
    let golden = read_to_string(golden_lib_path).unwrap();
    let new = library.display().to_string();
    crate::text_diff(golden.as_str(), new.as_str());
  }
}
