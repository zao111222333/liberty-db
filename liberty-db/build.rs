use dircpy::copy_dir;


use std::env;
use std::path::PathBuf;

fn main() {
    match copy_dir("docs/static", target_dir().join("doc/static")){
        Ok(_) => (),
        Err(e) => println!("{:?}",e),
    }
}

/// Find the location of the `target/` directory. Note that this may be
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
/// variable.
fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}