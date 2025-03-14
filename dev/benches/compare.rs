// cargo bench --package dev --bench compare --features compare
use dev::{gen_projs, run_bench};
use std::{
  fs::File,
  io::{BufWriter, Write},
};

#[cfg(feature = "compare")]
fn main() {
  let regress_table = run_bench(
    gen_projs![
      (LibertyDbLatest, liberty_db_latest::Library<liberty_db_latest::DefaultCtx>),
      (LibertyDbIncoming, liberty_db_incoming::Library<liberty_db_incoming::DefaultCtx>),
    ],
    true,
  );

  let out_file = File::create("../target/criterion/index.html").unwrap();
  let mut writer = BufWriter::new(out_file);
  let cpu_info = format!(
    "{} # {}Core",
    sysinfo::System::new_all().cpus()[0].brand(),
    sysinfo::System::new_all().cpus().len()
  );
  let date_info = chrono::offset::Utc::now().to_rfc2822();
  _ = write!(
    writer,
    r#"<!DOCTYPE html>
<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
</head>
<body><div class="body">
<p>Platform: {cpu_info}</p>
<p>{date_info}</p>
<div class="absolute">
{regress_table}
</div><div id="footer"><p>This report was generated by <a href="https://github.com/bheisler/criterion.rs">Criterion.rs</a>, a statistics-driven benchmarking library in Rust.</p></div></body></html>"#
  );
}

#[cfg(not(feature = "compare"))]
fn main() {
  panic!("require feature = \"compare\"")
}
