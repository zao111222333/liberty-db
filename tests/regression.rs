#![cfg(test)]

#[expect(unused_imports)]
#[cfg(not(feature = "tracing"))]
use log::{debug, error, info, trace, warn};
#[expect(unused_imports)]
#[cfg(feature = "tracing")]
use tracing::{debug, error, info, trace, warn};

use dev_utils::{all_files, init_logger, text_diff};
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
/// cargo test --package liberty-db --test regression -- make_golden --exact --show-output --ignored
#[test]
#[ignore]
fn make_golden() {
  use std::{
    fs::File,
    io::{BufWriter, Write},
  };
  init_logger();
  for (is_good, test_lib_path) in all_files("dev/tech") {
    println!("================\n{}", test_lib_path.display());
    let golden_lib_path = golden_path(&test_lib_path);
    let res = Library::<DefaultCtx>::parse_lib_file(&test_lib_path);
    if is_good {
      let library = res.unwrap();
      let golden_lib = File::create(golden_lib_path).unwrap();
      let mut writer = BufWriter::new(golden_lib);
      _ = write!(writer, "{}", library.display_name("library"));
    } else {
      assert!(res.is_err())
    }
  }
}

#[test]
fn regression() {
  init_logger();
  for (is_good, test_lib_path) in all_files("dev/tech") {
    println!("================\n{}", test_lib_path.display());
    let golden_lib_path = golden_path(&test_lib_path);
    let res = Library::<DefaultCtx>::parse_lib_file(&test_lib_path);
    if is_good {
      let library = res.unwrap();
      let golden = read_to_string(golden_lib_path).unwrap();
      let new = library.display_name("library").to_string();
      text_diff(golden.as_str(), new.as_str());
    } else {
      assert!(res.is_err())
    }
  }
}
