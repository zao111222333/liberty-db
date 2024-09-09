use benches::{run, BenchResult, OPenTimerLibraryPtr, Proj, ProjInfo, ProjLibrary};
use std::{
  fs::File,
  io::{BufWriter, Write},
};
use strum::IntoEnumIterator;
#[derive(Debug, strum_macros::EnumIter)]
enum Projs {
  LibertyDb,
  LibertyIo,
  LibertyParse,
  OPenTimer,
  Liberty2json,
}
#[derive(Debug, strum_macros::EnumIter)]
enum Versions {
  LibertyDbLatest,
  LibertyDb0p6p3,
  // LibertyDb0p6p2,
  // LibertyDb0p6p1,
  // LibertyDb0p6p0,
  LibertyDb0p5,
  LibertyDb0p4,
  LibertyDb0p3,
}
#[rustfmt::skip]
impl Proj for Versions {
  fn info(&self) -> ProjInfo {
    match self {
      Self::LibertyDbLatest => liberty_db_latest::Library::INFO,
      Self::LibertyDb0p6p3 => liberty_db_0p6p3::Library::INFO,
      // Self::LibertyDb0p6p2 => liberty_db_0p6p2::Library::INFO,
      // Self::LibertyDb0p6p1 => liberty_db_0p6p1::Library::INFO,
      // Self::LibertyDb0p6p0 => liberty_db_0p6p0::Library::INFO,
      Self::LibertyDb0p5 => liberty_db_0p5p9::Library::INFO,
      Self::LibertyDb0p4 => liberty_db_0p4p13::Library::INFO,
      Self::LibertyDb0p3 => liberty_db_0p3p1::library::Library::INFO,
    }
  }
  fn parse_bench(
    &self,
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    file_path: &str,
    group_path: &str,
  ) -> BenchResult {
    match self {
      Self::LibertyDbLatest => liberty_db_latest::Library::parse_bench(group, file_path, group_path),
      Self::LibertyDb0p6p3 => liberty_db_0p6p3::Library::parse_bench(group, file_path, group_path),
      // Self::LibertyDb0p6p2 => liberty_db_0p6p2::Library::parse_bench(group, file_path, group_path),
      // Self::LibertyDb0p6p1 => liberty_db_0p6p1::Library::parse_bench(group, file_path, group_path),
      // Self::LibertyDb0p6p0 => liberty_db_0p6p0::Library::parse_bench(group, file_path, group_path),
      Self::LibertyDb0p5 => liberty_db_0p5p9::Library::parse_bench(group, file_path, group_path),
      Self::LibertyDb0p4 => liberty_db_0p4p13::Library::parse_bench(group, file_path, group_path),
      Self::LibertyDb0p3 => liberty_db_0p3p1::library::Library::parse_bench(group, file_path, group_path),
    }
  }
  fn write_bench(
    &self,
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    file_path: &str,
    group_path: &str,
  ) -> BenchResult {
    match self {
      Self::LibertyDbLatest => liberty_db_latest::Library::write_bench(group, file_path, group_path),
      Self::LibertyDb0p6p3 => liberty_db_0p6p3::Library::write_bench(group, file_path, group_path),
      // Self::LibertyDb0p6p2 => liberty_db_0p6p2::Library::write_bench(group, file_path, group_path),
      // Self::LibertyDb0p6p1 => liberty_db_0p6p1::Library::write_bench(group, file_path, group_path),
      // Self::LibertyDb0p6p0 => liberty_db_0p6p0::Library::write_bench(group, file_path, group_path),
      Self::LibertyDb0p5 => liberty_db_0p5p9::Library::write_bench(group, file_path, group_path),
      Self::LibertyDb0p4 => liberty_db_0p4p13::Library::write_bench(group, file_path, group_path),
      Self::LibertyDb0p3 => liberty_db_0p3p1::library::Library::write_bench(group, file_path, group_path),
    }
  }
}
#[rustfmt::skip]
impl Proj for Projs {
  fn info(&self) -> ProjInfo {
    match self {
      Self::LibertyDb => liberty_db_latest::Library::INFO,
      Self::LibertyIo => liberty_io::Group::INFO,
      Self::LibertyParse => libertyparse::Liberty::INFO,
      Self::OPenTimer => OPenTimerLibraryPtr::INFO,
      Self::Liberty2json => liberty2json::Liberty::INFO,
    }
  }
  fn parse_bench(
    &self,
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    file_path: &str,
    group_path: &str,
  ) -> BenchResult {
    match self {
      Self::LibertyDb => liberty_db_latest::Library::parse_bench(group, file_path, group_path),
      Self::LibertyIo => liberty_io::Group::parse_bench(group, file_path, group_path),
      Self::LibertyParse => libertyparse::Liberty::parse_bench(group, file_path, group_path),
      Self::OPenTimer => OPenTimerLibraryPtr::parse_bench(group, file_path, group_path),
      Self::Liberty2json => liberty2json::Liberty::parse_bench(group, file_path, group_path),
    }
  }
  fn write_bench(
    &self,
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    file_path: &str,
    group_path: &str,
  ) -> BenchResult {
    match self {
      Self::LibertyDb => liberty_db_latest::Library::write_bench(group, file_path, group_path),
      Self::LibertyIo => liberty_io::Group::write_bench(group, file_path, group_path),
      Self::LibertyParse => libertyparse::Liberty::write_bench(group, file_path, group_path),
      Self::OPenTimer => OPenTimerLibraryPtr::write_bench(group, file_path, group_path),
      Self::Liberty2json => liberty2json::Liberty::write_bench(group, file_path, group_path),
    }
  }
}
fn main() {
  let projs_table = run(Projs::iter(), false);
  let regress_table = run(Versions::iter(), true);

  let out_file = File::create("../target/criterion/index.html").unwrap();
  let mut writer = BufWriter::new(out_file);
  let ccs_text = "body {font: 14px Helvetica Neue;text-rendering: optimizelegibility;}.body {width: 960px;margin: auto;}th {font-weight: 200}th,td {padding-right: 3px;padding-bottom: 3px;}a:link {color: #1F78B4;text-decoration: none;}th.ci-bound {opacity: 0.6}td.ci-bound {opacity: 0.5}.stats {width: 80%;margin: auto;display: flex;}.additional_stats {flex: 0 0 60%}.additional_plots {flex: 1}h2 {font-size: 36px;font-weight: 300;}h3 {font-size: 24px;font-weight: 300;}#footer {height: 40px;background: #888;color: white;font-size: larger;font-weight: 300;}#footer a {color: white;text-decoration: underline;}#footer p {text-align: center}.info-table table, .info-table th, .info-table td {border: 1px solid black;border-collapse: collapse;padding-left: 5px;padding-right: 5px;}";
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
<title>Benchmark Result</title>
<style type="text/css">
{ccs_text}
</style>
</head>
<body><div class="body"><h2>Benchmark Result</h2>
<p>Platform: {cpu_info}</p>
<p>{date_info}</p>
<div class="absolute">
<h2>Project Comparison Benchmark</h2>
{projs_table}
<h2>Self Regression</h2>
{regress_table}
</div><div id="footer"><p>This report was generated by <a href="https://github.com/bheisler/criterion.rs">Criterion.rs</a>, a statistics-driven benchmarking library in Rust.</p></div></body></html>"#
  );
}
