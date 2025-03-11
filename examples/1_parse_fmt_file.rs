use liberty_db::{DefaultCtx, Library};
use std::{
  env,
  fs::{File, read_to_string},
  io::{BufWriter, Write},
  path::Path,
  time::Instant,
};

// cargo run /path/to/xxx.lib
// Use the first arg as input file name,
// parse it, and then write it into example1_xxx.lib
fn main() {
  simple_logger::SimpleLogger::new().init().unwrap();
  let args: Vec<String> = env::args().collect();

  let input_lib = Path::new(&args[1]);
  log::info!("Parsing [file] {} ...", input_lib.display());
  let now = Instant::now();
  let library =
    Library::<DefaultCtx>::parse_lib(read_to_string(input_lib).unwrap().as_str())
      .unwrap();
  let elapsed_parse = now.elapsed();
  let out_file_name =
    format!("example1_{}", input_lib.file_name().unwrap().to_str().unwrap());
  log::info!("Output to [file] {} ...", out_file_name);
  let out_file = File::create(out_file_name).unwrap();
  let mut writer = BufWriter::new(out_file);
  let now = Instant::now();
  write!(&mut writer, "{}", library).unwrap();
  let elapsed_write = now.elapsed();
  log::info!("DONE");
  log::info!("parse: {elapsed_parse:?}");
  log::info!("write: {elapsed_write:?}");
}
