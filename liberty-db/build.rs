// use dircpy::copy_dir;
use std::fs::copy;

use std::env;
use std::path::{PathBuf,Path};

fn main() {
    let root_docs_str = "../docs";
    println!("cargo:rerun-if-changed={root_docs_str}/*");
    let root_docs = Path::new(&root_docs_str);
    let local_docs = Path::new("docs");
    if root_docs.exists() {
        // match copy_dir(root_docs.join("static"), local_docs){
        //     Ok(_) => (),
        //     Err(e) => println!("{:?}",e),
        // }
        if !local_docs.exists(){
            match std::fs::create_dir(local_docs){
                Ok(_) => (),
            Err(e) => println!("{:?}",e),
            }
        }
        match copy(root_docs.join("header.html"), local_docs.join("header.html")){
            Ok(_) => (),
            Err(e) => println!("{:?}",e),
        }
    }
    // let target_docs = target_dir().join("doc");
    // match copy_dir(local_docs, target_docs){
    //     Ok(_) => (),
    //     Err(e) => println!("{:?}",e),
    // }
}

// /// Find the location of the `target/` directory. Note that this may be
// /// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
// /// variable.
// fn target_dir() -> PathBuf {
//     if let Ok(target) = env::var("CARGO_TARGET_DIR") {
//         PathBuf::from(target)
//     } else {
//         PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
//     }
// }