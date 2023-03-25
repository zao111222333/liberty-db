use std::{fs, str::FromStr};

use crate::parser::wrapper::library_wrapper;

use super::wrapper::Library;

#[test]
fn lib_file_to_wrapper() {
  let paths = fs::read_dir("./tests/tech").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
  // let filepath = "/Users/junzhuo/Downloads/debug_liberate_INV0_hspice.lib";
  // match fs::read_to_string(filepath){
  //   Ok(data) => {
  //     println!("{:?}", library_wrapper(&data));
  //     // println!("{:?}", Library::from_str(&data));
  //   },
  //   Err(_) => todo!(),
  // };
}