#![allow(unused)]
mod parser_bench;

use std::{ffi::OsStr, fs::metadata, path::PathBuf};

use walkdir::WalkDir;

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
