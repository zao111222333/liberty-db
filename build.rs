use std::path::{Path, PathBuf};
use std::process::Command;

use std::{env, fs};
// 用 glob 模式获取所有的 .cpp 文件
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
  // 获取构建目录
  let out_dir = env::var("OUT_DIR").unwrap();
  let projs_dir = Path::new("benches/projs");
  // g++ -std=c++17 -fPIC -shared -I. -o libprojs.so projs.cpp ot/liberty/*.cpp ot/unit/*.cpp -lstdc++fs
  let status = Command::new("g++")
    .args([
      "-std=c++17",                          // 使用 C++17 标准
      "-fPIC",                               // 生成位置无关代码
      "-shared",                             // 编译为共享库
      &format!("-I{}", projs_dir.display()), // 当前目录作为头文件路径
      "-o",
      &format!("{}/libprojs.so", out_dir), // 输出路径
      &format!("{}", projs_dir.join("projs.cpp").display()), // 主源文件
    ])
    .args(cpp_files(&[projs_dir.join("ot/liberty"), projs_dir.join("ot/unit")])) // 添加源文件
    .arg("-lstdc++fs") // 链接文件系统库
    .status()
    .expect("failed to compile C++ code");

  if !status.success() {
    panic!("g++ compilation failed");
  }

  // 告诉 cargo 链接到编译生成的共享库
  println!("cargo::rerun-if-changed={}", projs_dir.join("projs.cpp").display());
  println!("cargo:rustc-link-search=native={out_dir}");
  println!("cargo:rustc-env=LD_LIBRARY_PATH={out_dir}");
  println!("cargo:rustc-link-lib=dylib=projs");
}
