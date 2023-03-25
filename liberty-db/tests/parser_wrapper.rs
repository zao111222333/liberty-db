#[cfg(test)]
use liberty_db::parser::wrapper::library_wrapper;
use colored::Colorize;
use walkdir::WalkDir;
use std::fmt::Display;
use std::path::PathBuf;
use std::{fs, default};
use std::fs::metadata;
use std::ffi::OsStr;
use std::time::{Instant, Duration};
use liberty_io;
use std::fs::File;
use std::io::BufReader;
use std::panic;

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

#[derive(Default)]
enum ReturnState {
    PASS(Duration),
    #[default]
    FAIL,
    PANIC,
}

impl Display for ReturnState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnState::PASS(d) => write!(f,"{}",
                                format!("{:.2?}",d).green().bold()),
            ReturnState::FAIL => write!(f,"{}",
                                        "FAIL".bold().bright_red()),
            ReturnState::PANIC => write!(f,"{}",
                                        "PANIC".red().bold()),
        }
    }
}

#[derive(Default)]
struct TestResult{
    file_in: String, 
    state: ReturnState, 
    duration: Duration,
}
impl Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}\t{}",self.state,self.file_in)
        // match self.state {
        //     ReturnState::PASS => write!(f,"{}\t{:.2?}\t{}",
        //                                 "PASS".green().bold(),
        //                                 self.duration,
        //                                 self.file_in),
        //     ReturnState::FAIL => write!(f,"{}\tNaN\t{}",
        //                                 "FAIL".bold().bright_red(),
        //                                 self.file_in),
        //     ReturnState::PANIC => write!(f,"{}\tNaN\t{}",
        //                                 "PANIC".red().bold(),
        //                                 self.file_in),
        // }
        
        // let mut all_pass = true;
    // for (file,pass,duration) in summary{
    //     println!("{}\t{:.2?}\t{}",
    //     (if pass{"PASS".green()}else{all_pass=false;"FAIL".red()}).bold(),
    //     duration,
    //     file);
    // }
    }
}

fn parse_lib_files(
    lib_files: Vec<PathBuf>, 
    parser: fn(PathBuf)->Result<(),std::fmt::Error>,
) -> Vec<TestResult> {
    lib_files.iter().map(|filepath|
        {
            let bgn = Instant::now();
            let mut out = TestResult::default();
            out.file_in = String::from(filepath.to_str().unwrap());
            let panic_result = panic::catch_unwind(|| {
                let result = parser(filepath.to_path_buf());
                return (result,bgn.elapsed());
            });
            match panic_result{
                Ok((r,d)) => match r {
                    Ok(_) => {
                        out.state = ReturnState::PASS(bgn.elapsed());
                    },
                    Err(_) => {
                        out.state = ReturnState::FAIL;
                    },
                },
                Err(_) => out.state = ReturnState::PANIC,
            }
            println!("{}",out);
            out
        }
    ).collect::<Vec<TestResult>>()
    // let mut all_pass = true;
    // for (file,pass,duration) in summary{
    //     println!("{}\t{:.2?}\t{}",
    //     (if pass{"PASS".green()}else{all_pass=false;"FAIL".red()}).bold(),
    //     duration,
    //     file);
    // }
}
fn parser_liberty_db(filepath: PathBuf)->Result<(),std::fmt::Error>{
    let data = fs::read_to_string(filepath.clone()).expect("Failed to open file.");
    let result = library_wrapper(&data);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(std::fmt::Error),
    }
}

fn parser_liberty_io(filepath: PathBuf)->Result<(),std::fmt::Error>{
    let f = File::open(filepath).expect("Failed to open file.");
    let mut buf = BufReader::new(f);
    // Read the file.
    let result = liberty_io::read_liberty_bytes(&mut buf);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(std::fmt::Error),
    }
}

fn parser_libertyparse(filepath: PathBuf)->Result<(),std::fmt::Error>{
    use libertyparse::*;
    let data = fs::read_to_string(filepath.clone()).expect("Failed to open file.");
    let parsed = Liberty::parse_str(&data);
    match parsed {
        Ok(_) => Ok(()),
        Err(_) => Err(std::fmt::Error),
    }
}

#[test]
fn test_all_lib_files(){
    let all_parser:Vec<(&str,fn(PathBuf)->Result<(),std::fmt::Error>)> = vec![
        ("This crate, https://crates.io/crates/liberty-db",parser_liberty_db),
        ("liberty-io, https://crates.io/crates/liberty-io",parser_liberty_io),
        ("libertyparse, https://crates.io/crates/libertyparse",parser_libertyparse),
        ];
    let pass_list: Vec<Vec<TestResult>> = all_parser.iter().map(
        |(title, parser)|{
            println!("");
            println!("{}:",title);
            parse_lib_files(all_lib_files(), *parser)
        }
    ).collect();
}