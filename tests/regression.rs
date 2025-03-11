#![cfg(test)]
use dev_utils::{all_files, text_diff};
use liberty_db::{DefaultCtx, Library, ast::Group};
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
/// cargo test --no-default-features --test regression -- make_golden --exact --show-output
#[cfg(all(not(feature = "fast_hash"), not(feature = "hashmatch"),))]
#[allow(dead_code)]
#[test]
fn make_golden() {
  use std::{
    fs::File,
    io::{BufWriter, Write},
  };
  _ = simple_logger::SimpleLogger::new().init();
  for test_lib_path in all_files("dev/tech") {
    let golden_lib_path = golden_path(&test_lib_path);
    log::info!("{}", test_lib_path.display());
    let library =
      Library::<DefaultCtx>::parse_lib(read_to_string(test_lib_path).unwrap().as_str())
        .unwrap();
    let golden_lib = File::create(golden_lib_path).unwrap();
    let mut writer = BufWriter::new(golden_lib);
    _ = write!(writer, "{}", library.display());
  }
}

#[cfg(test)]
#[test]
fn regression() {
  _ = simple_logger::SimpleLogger::new().init();
  for test_lib_path in all_files("dev/tech") {
    println!("================\n{}", test_lib_path.display());
    let golden_lib_path = golden_path(&test_lib_path);
    let library =
      Library::<DefaultCtx>::parse_lib(read_to_string(test_lib_path).unwrap().as_str())
        .unwrap();
    let golden = read_to_string(golden_lib_path).unwrap();
    let new = library.display().to_string();
    text_diff(golden.as_str(), new.as_str());
  }
}
