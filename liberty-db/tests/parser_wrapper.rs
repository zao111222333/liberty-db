#[cfg(test)]
use liberty_db::parser::wrapper::library_wrapper;
use colored::Colorize;
use walkdir::WalkDir;
use std::path::PathBuf;
use std::fs;
use std::fs::metadata;
use std::ffi::OsStr;
use std::time::{Instant, Duration};


fn all_lib_files() -> Vec<PathBuf>{
    WalkDir::new("tests/tech").into_iter().filter(|e|
        match e {
            Ok(entry) => {
                let path = entry.path();
                let extension = path.extension().and_then(OsStr::to_str);
                let md = metadata(path).unwrap();
                md.is_file() && extension==Some("lib")
            },
            Err(_) => false,
        }
    ).map(|e| e.unwrap().into_path()).collect::<Vec<PathBuf>>()
}

#[test]
fn testttt(){
    let filepath = "tests/tech/nangate/test.lib";
    let data = fs::read_to_string(filepath.clone()).unwrap();
    println!("{:?}",library_wrapper(&data));
}

#[test]
fn test_all_lib_files(){
    let binding = all_lib_files();
    let summary = binding.iter().map(|filepath|
        {
            let data = fs::read_to_string(filepath.clone()).unwrap();
            let bgn = Instant::now();
            let result = library_wrapper(&data);
            (filepath.to_str().unwrap(),result.is_ok(),bgn.elapsed())
        }
    ).collect::<Vec<(&str,bool,Duration)>>();
    let mut all_pass = true;
    for (file,pass,duration) in summary{
        println!("{}\t{:.2?}\t{}",
        (if pass{"PASS".green()}else{all_pass=false;"FAIL".red()}).bold(),
        duration,
        file);
    }
    assert!(all_pass);
}