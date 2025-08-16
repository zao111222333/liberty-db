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
  init_logger();
  for (is_good, test_lib_path) in all_files("dev/tech") {
    println!("================\n{}", test_lib_path.display());
    let res = Library::<DefaultCtx>::parse_lib_file(&test_lib_path);
    if is_good {
      let golden_lib_path = golden_path(&test_lib_path);
      let mut library = res.unwrap();
      library.comments_this_entry().insert_entry("golden".to_string());
      library.write_lib_file(&golden_lib_path).unwrap();
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
    let res = Library::<DefaultCtx>::parse_lib_file(&test_lib_path);
    if is_good {
      let library = res.unwrap();
      let golden_lib_path = golden_path(&test_lib_path);
      let golden = read_to_string(golden_lib_path).unwrap();
      let new = library.display_name("library").to_string();
      text_diff(golden.as_str(), new.as_str());
    } else {
      assert!(res.is_err())
    }
  }
}
