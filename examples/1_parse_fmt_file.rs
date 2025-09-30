use liberty_db::{DefaultCtx, Library};
use std::{env, path::Path, process::ExitCode, time::Instant};

const PROFILE: bool = false;
// cargo run /path/to/xxx.lib
// Use the first arg as input file name,
// parse it, and then write it into example1_xxx.lib
fn main() -> ExitCode {
  let guard = PROFILE.then(|| {
    pprof::ProfilerGuardBuilder::default()
      .frequency(1000)
      .blocklist(&["libc", "libgcc", "pthread", "vdso"])
      .build()
      .unwrap()
  });

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
  let now = Instant::now();
  library.write_lib_file(&out_file_name).unwrap();
  let elapsed_write = now.elapsed();
  log::info!("DONE");
  log::info!("parse: {elapsed_parse:.2?}");
  log::info!("write: {elapsed_write:.2?}");
  guard.inspect(|guard| {
    if let Ok(report) = guard.report().build() {
      let file = std::fs::File::create("example1_flamegraph.svg").unwrap();
      let mut options = pprof::flamegraph::Options::default();
      options.image_width = Some(10000);
      report.flamegraph_with_options(file, &mut options).unwrap();
    }
  });
  ExitCode::SUCCESS
}
