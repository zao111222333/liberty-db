use liberty_db::{DefaultCtx, Library};
use std::{
  env,
  fs::File,
  io::{BufWriter, Write},
  path::Path,
  process::ExitCode,
  time::Instant,
};

// cargo run /path/to/xxx.lib
// Use the first arg as input file name,
// parse it, and then write it into example1_xxx.lib
fn main() -> ExitCode {
  dev_utils::init_logger();
  let args: Vec<String> = env::args().collect();

  let input_lib = Path::new(&args[1]);
  log::info!("Parsing [file] {} ...", input_lib.display());
  let now = Instant::now();
  let library = match Library::<DefaultCtx>::parse_lib_file(input_lib) {
    Ok(l) => l,
    Err(e) => {
      log::error!("{e}");
      return ExitCode::FAILURE;
    }
  };
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
  ExitCode::SUCCESS
}
