#![expect(unused_imports, dead_code)]
use std::{
  env, fs,
  path::{Path, PathBuf},
  process::Command,
};
// get all .cpp file in glob method
fn cpp_files(search_paths: &[PathBuf]) -> Vec<String> {
  search_paths
    .iter()
    .flat_map(|search_path| fs::read_dir(search_path).unwrap())
    .filter_map(|entry| {
      let entry = entry.unwrap();
      let path = entry.path();
      if path.extension().and_then(|s| s.to_str()) == Some("cpp") {
        Some(path.display().to_string())
      } else {
        None
      }
    })
    .collect()
}
#[cfg(all(target_os = "linux", feature = "bench"))]
fn main() {
  let binding = env::var("OUT_DIR").unwrap();
  let out_dir = Path::new(&binding);
  let build_dir = out_dir.join("build");
  let projs_dir =
    Path::new(&env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set"))
      .join("include");

  if build_dir.exists() {
    fs::remove_dir_all(&build_dir).expect("failed to clean");
  }
  fs::create_dir(&build_dir).expect("failed to mkdir");

  if !Command::new("make")
    .arg("-f")
    .arg(projs_dir.join("si2libertyParser/Makefile").display().to_string())
    .current_dir(&build_dir)
    .status()
    .expect("fail to build si2libertyParser")
    .success()
  {
    panic!("fail to build si2libertyParser");
  }
  // g++ -std=c++17 -fPIC -shared -I. -o libprojs.so projs.cpp ot/liberty/*.cpp ot/unit/*.cpp -lstdc++fs
  let status = Command::new("g++")
    .args([
      "-std=c++17",
      "-fPIC",
      "-shared",
      &format!("-I{}", projs_dir.display()),
      "-O3",
      "-o",
      &out_dir.join("libprojs.so").display().to_string(),
      &format!("{}", projs_dir.join("projs.cpp").display()),
    ])
    .args(cpp_files(&[projs_dir.join("ot/liberty"), projs_dir.join("ot/unit")]))
    .args([
      // add build libsi2dr_liberty.a
      &format!("-L{}", build_dir.display()),
      "-lsi2dr_liberty",
      "-lstdc++fs",
    ])
    .status()
    .expect("failed to compile C++ code");

  if !status.success() {
    panic!("g++ compilation failed");
  }

  println!("cargo:rerun-if-changed={}", projs_dir.join("projs.cpp").display());
  println!("cargo:rustc-link-search=native={}", out_dir.display());
  println!("cargo:rustc-link-lib=dylib=projs");
}

#[cfg(any(not(target_os = "linux"), not(feature = "bench")))]
fn main() {}
