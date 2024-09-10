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

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();
  let projs_dir = Path::new("include");
  // g++ -std=c++17 -fPIC -shared -I. -o libprojs.so projs.cpp ot/liberty/*.cpp ot/unit/*.cpp -lstdc++fs
  let status = Command::new("g++")
    .args([
      "-std=c++17",
      "-fPIC",
      "-shared",
      &format!("-I{}", projs_dir.display()),
      "-O3",
      "-o",
      &format!("{}/libprojs.so", out_dir),
      &format!("{}", projs_dir.join("projs.cpp").display()),
    ])
    .args(cpp_files(&[projs_dir.join("ot/liberty"), projs_dir.join("ot/unit")]))
    .arg("-lstdc++fs")
    .status()
    .expect("failed to compile C++ code");

  if !status.success() {
    panic!("g++ compilation failed");
  }

  println!("cargo:rerun-if-changed={}", projs_dir.join("projs.cpp").display());
  println!("cargo:rustc-link-search=native={out_dir}");
  println!("cargo:rustc-link-lib=dylib=projs");
}
