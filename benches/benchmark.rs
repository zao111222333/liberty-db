use criterion::{black_box, Criterion};
use itertools::Itertools;
use std::{
  ffi::{c_char, c_void, CString, OsStr},
  fs::{metadata, read_to_string, File},
  io::{BufWriter, Cursor, Write},
  panic,
  path::{Path, PathBuf},
  str::FromStr,
  time::Duration,
};
use strum::IntoEnumIterator;

type OPenTimerLibraryPtr = *mut c_void;
extern "C" {
  fn ot_parse_lib(s: *const c_char) -> OPenTimerLibraryPtr;
  fn ot_write_lib(ptr: OPenTimerLibraryPtr);
  fn ot_drop_lib(ptr: OPenTimerLibraryPtr);
}

fn all_files() -> impl Iterator<Item = PathBuf> {
  walkdir::WalkDir::new("tests/tech")
    .into_iter()
    .filter_map(|e| match e {
      Ok(entry) => {
        let path = entry.path();
        let extension = path.extension().and_then(OsStr::to_str);
        let md = metadata(path).unwrap();
        if md.is_file() && extension == Some("lib") {
          Some(entry.into_path())
        } else {
          None
        }
      }
      Err(_) => None,
    })
}

enum TypedSupport {
  AllTyped,
  PartialTyped,
  AstOnly,
}
struct ProjInfo {
  name: &'static str,
  url: &'static str,
  lang: &'static str,
  typed_support: TypedSupport,
  json_support: bool,
  parsed_boolexpr: bool,
  other: &'static str,
}
trait ProjLibrary: Sized {
  const INFO: ProjInfo;
  fn parse(s: &str) -> Result<Self, ()>;
  fn write(&self) -> Result<(), ()> {
    Err(())
  }
  fn drop(self) {
    drop(self)
  }
  fn fail(file_path: &str, parse_or_write: bool) -> BenchResult {
    println!(
      "\n{}{}",
      console::style(format!(
        "[{}] {file_path} / {}",
        if parse_or_write { "parse" } else { "write" },
        Self::INFO.name
      ))
      .red(),
      console::style(" [FAIL] ").bold().red()
    );
    BenchResult::Fail
  }
  fn id() -> String {
    format!(" {}", Self::INFO.name)
  }
  fn collect_result(group_path: &str) -> BenchResult {
    let data = read_to_string(
      Path::new("target/criterion")
        .join(group_path)
        .join(Self::id())
        .join("new/estimates.json"),
    )
    .unwrap();
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    if let serde_json::Value::Number(n) = &v["mean"]["point_estimate"] {
      if let Some(f) = n.as_f64() {
        return BenchResult::Ok {
          run_time: Duration::from_secs_f64(f / 1e9),
          path: Path::new(group_path)
            .join(Self::id())
            .join("report/")
            .display()
            .to_string(),
        };
      }
    }
    BenchResult::Fail
  }
  fn parse_bench(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    file_path: &str,
    group_path: &str,
  ) -> BenchResult {
    let library_string = read_to_string(file_path).unwrap();
    let s = library_string.as_str();
    if let Ok(Ok(try_l)) = panic::catch_unwind(|| (Self::parse)(black_box(s))) {
      (Self::drop)(try_l);
      group.bench_function(Self::id(), |b| {
        b.iter(|| (Self::parse)(black_box(s)).map(|l| (Self::drop)(l)))
      });
      Self::collect_result(group_path)
    } else {
      Self::fail(file_path, true)
    }
  }
  fn write_bench(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    file_path: &str,
    group_path: &str,
  ) -> BenchResult {
    let library_string = read_to_string(file_path).unwrap();
    let s = library_string.as_str();
    if let Ok(Ok(l)) = panic::catch_unwind(|| (Self::parse)(black_box(s))) {
      if (Self::write)(black_box(&l)).is_ok() {
        group.bench_function(Self::id(), |b| b.iter(|| (Self::write)(black_box(&l))));
        (Self::drop)(l);
        return Self::collect_result(group_path);
      }
      (Self::drop)(l);
    }
    Self::fail(file_path, false)
  }
}

#[derive(Debug, strum_macros::EnumIter)]
enum ProjUnderTest {
  LibertyDb,
  LibertyIo,
  LibertyParse,
  OPenTimer,
  Liberty2json,
}

#[rustfmt::skip]
impl ProjUnderTest {
  fn info(&self) -> ProjInfo {
    match self {
      Self::LibertyDb => liberty_db::Library::INFO,
      Self::LibertyIo => liberty_io::Group::INFO,
      Self::LibertyParse => libertyparse::Liberty::INFO,
      Self::OPenTimer => OPenTimerLibraryPtr::INFO,
      Self::Liberty2json => liberty2json::Liberty::INFO,
    }
  }
  fn info_html(&self) -> String {
    let info = self.info();
    format!(
      "<tr><th style=\"text-align:left;padding-left:5px\"><a href=\"{}\">{}</a></th><th>{}</th>{}<th>{}</th><th>{}</th><th>{}</th></tr>", 
      info.url,
      info.name,
      info.lang,
      match info.typed_support{
        TypedSupport::AllTyped => "<th>&#10003</th><th></th><th></th>",
        TypedSupport::PartialTyped => "<th></th><th>&#10003</th><th></th>",
        TypedSupport::AstOnly => "<th></th><th></th><th>&#10003</th>",
      },
      if info.parsed_boolexpr{"&#10003"}else{""},
      if info.json_support{"&#10003"}else{""},
      info.other
    )
  }
  fn html(&self) -> String {
    let info = self.info();
    format!("<th><a href=\"{}\">{}</a></th>", info.url, info.name)
  }
  fn parse_bench(
    &self,
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    file_path: &str,
    group_path: &str,
  ) -> BenchResult {
    match self {
      Self::LibertyDb => liberty_db::Library::parse_bench(group, file_path, group_path),
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
      Self::LibertyDb => liberty_db::Library::write_bench(group, file_path, group_path),
      Self::LibertyIo => liberty_io::Group::write_bench(group, file_path, group_path),
      Self::LibertyParse => libertyparse::Liberty::write_bench(group, file_path, group_path),
      Self::OPenTimer => OPenTimerLibraryPtr::write_bench(group, file_path, group_path),
      Self::Liberty2json => liberty2json::Liberty::write_bench(group, file_path, group_path),
    }
  }
}
impl ProjLibrary for liberty_db::Library {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-db",
    url: "https://crates.io/crates/liberty-db",
    lang: "rust",
    typed_support: TypedSupport::AllTyped,
    json_support: true,
    parsed_boolexpr: true,
    other: "this project",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    liberty_db::Library::parse_lib(s).map_err(|_| ())
  }
  fn write(&self) -> Result<(), ()> {
    _ = black_box(self.to_string());
    Ok(())
  }
}
impl ProjLibrary for liberty_io::Group {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-io",
    url: "https://crates.io/crates/liberty-io",
    lang: "rust",
    typed_support: TypedSupport::AstOnly,
    json_support: false,
    parsed_boolexpr: false,
    other: "",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    let mut cursor = Cursor::new(s.as_bytes());
    liberty_io::read_liberty_bytes(&mut cursor).map_err(|_| ())
  }
}
impl ProjLibrary for libertyparse::Liberty {
  const INFO: ProjInfo = ProjInfo {
    name: "libertyparse",
    url: "https://crates.io/crates/libertyparse",
    lang: "rust",
    typed_support: TypedSupport::PartialTyped,
    json_support: false,
    parsed_boolexpr: true,
    other: "",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::parse_str(s).map_err(|_| ())
  }
}
impl ProjLibrary for OPenTimerLibraryPtr {
  const INFO: ProjInfo = ProjInfo {
    name: "OpenTimer",
    url: "https://github.com/OpenTimer/OpenTimer",
    lang: "c++17",
    typed_support: TypedSupport::PartialTyped,
    json_support: false,
    parsed_boolexpr: true,
    other: "",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    let cstr = CString::new(s).unwrap();
    Ok(unsafe { ot_parse_lib(cstr.as_ptr()) })
  }
  fn write(&self) -> Result<(), ()> {
    unsafe { ot_write_lib(*self) };
    Ok(())
  }
  fn drop(self) {
    unsafe { ot_drop_lib(self) }
  }
}
impl ProjLibrary for liberty2json::Liberty {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty2json",
    url: "https://github.com/erihsu/liberty2json",
    lang: "rust",
    typed_support: TypedSupport::AstOnly,
    json_support: true,
    parsed_boolexpr: false,
    other: "",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::from_str(s).map_err(|_| ())
  }
}

#[derive(Debug, Default)]
enum BenchResult {
  #[default]
  Fail,
  Ok {
    run_time: Duration,
    path: String,
  },
}
impl BenchResult {
  fn html(&self) -> String {
    fn format_duration(duration: &Duration) -> String {
      let nanos = duration.as_nanos();
      let seconds = duration.as_secs_f64();
      if nanos >= 1_000_000_000 {
        format!("{:.2}  s", seconds)
      } else if nanos >= 1_000_000 {
        let millis = seconds * 1e3;
        format!("{:.2} ms", millis)
      } else if nanos >= 1_000 {
        let micros = seconds * 1e6;
        format!("{:.2} µs", micros)
      } else {
        let nanos = seconds * 1e9;
        format!("{:.2} ns", nanos)
      }
    }
    match self {
      Self::Fail => {
        "<td style=\"color:Red;text-align:right;padding-right:10px;\">FAIL</td>"
          .to_string()
      }
      Self::Ok { run_time, path } => {
        format!(
          "<td style=\"text-align:right;padding-right:10px;\"><a href=\"./{path}\" style=\"color:MediumSeaGreen;\">{}</a></td>",
          format_duration(run_time)
        )
      }
    }
  }
}

type ResList = Vec<(String, [(String, Vec<BenchResult>); 2])>;
fn bench_all(c: &mut Criterion) -> ResList {
  let group_path_max_len = 64;
  let group_name2path = |group_name: &String| {
    let mut group_path = group_name.replace('/', "_");
    if group_path.len() > group_path_max_len {
      _ = group_path.split_off(group_path_max_len);
    }
    group_path
  };
  let mut res_list = Vec::new();
  for path in all_files() {
    let file_path = path.display().to_string();
    let parse_res = {
      let group_name = format!("[parse] {file_path} ");
      let group_path = group_name2path(&group_name);
      let mut parse_group = c.benchmark_group(&group_name);
      let res = ProjUnderTest::iter()
        .map(|proj| proj.parse_bench(&mut parse_group, &file_path, &group_path))
        .collect();
      parse_group.finish();
      (group_path, res)
    };
    let write_res = {
      let group_name = format!("[write] {file_path} ");
      let group_path = group_name2path(&group_name);
      let mut write_group = c.benchmark_group(&group_name);
      let res = ProjUnderTest::iter()
        .map(|proj| proj.write_bench(&mut write_group, &file_path, &group_path))
        .collect();
      write_group.finish();
      (group_path, res)
    };
    res_list.push((file_path, [parse_res, write_res]));
  }
  res_list
}

fn info_table() -> String {
  format!(
    "<h3>Projects Under Test</h3><div class=\"info-table\"><table><thead><tr><th style=\"font-weight:bold;\">Name</th><th style=\"font-weight:bold;\">Language</th><th colspan=\"3\" style=\"text-align:center;font-weight:bold;\">Type Support</th><th style=\"font-weight:bold;\">Boolean Expression</th><th style=\"font-weight:bold;\">Json Convert</th><th style=\"font-weight:bold;\">Comment</th></tr><tr><th></th><th></th><th>All</th><th>Partly</th><th>AST only</th><th></th><th></th><th></th></tr></thead><tbody>{}</tbody></table></div>", 
    ProjUnderTest::iter().map(|proj| proj.info_html()).join(""),
  )
}
fn res_table(res_list: ResList) -> String {
  let mut parse_table = format!(
    "<table><thead><tr>{}<th>Test Case</th></tr></thead><tbody>",
    ProjUnderTest::iter().map(|proj| proj.html()).join("")
  );
  let mut write_table = parse_table.clone();
  for (file_path, [(parse_path, parse_res), (write_path, write_res)]) in res_list {
    parse_table += &format!(
      "<tr>{}<td><a href=\"./{parse_path}/report/\">{file_path}</a></td></tr>",
      parse_res.iter().map(|res| res.html()).join("")
    );
    write_table += &format!(
      "<tr>{}<td><a href=\"./{write_path}/report/\">{file_path}</a></td></tr>",
      write_res.iter().map(|res| res.html()).join("")
    );
  }
  parse_table += "</tbody></table>";
  write_table += "</tbody></table>";
  format!("<h3>Parse Performance Comparison</h3>{parse_table}<h3>Write Performance Comparison</h3>{write_table}")
}

fn main() {
  let mut criterion = Criterion::default()
    .sample_size(100)
    .with_output_color(true)
    .warm_up_time(Duration::from_millis(1000))
    .configure_from_args();
  let res_list = bench_all(&mut criterion);
  criterion.final_summary();

  let out_file = File::create("target/criterion/index.html").unwrap();
  let mut writer = BufWriter::new(out_file);
  let ccs_text = "body {font: 14px Helvetica Neue;text-rendering: optimizelegibility;}.body {width: 960px;margin: auto;}th {font-weight: 200}th,td {padding-right: 3px;padding-bottom: 3px;}a:link {color: #1F78B4;text-decoration: none;}th.ci-bound {opacity: 0.6}td.ci-bound {opacity: 0.5}.stats {width: 80%;margin: auto;display: flex;}.additional_stats {flex: 0 0 60%}.additional_plots {flex: 1}h2 {font-size: 36px;font-weight: 300;}h3 {font-size: 24px;font-weight: 300;}#footer {height: 40px;background: #888;color: white;font-size: larger;font-weight: 300;}#footer a {color: white;text-decoration: underline;}#footer p {text-align: center}.info-table table, .info-table th, .info-table td {border: 1px solid black;border-collapse: collapse;padding-left: 5px;padding-right: 5px;}";
  let info_table = info_table();
  let res_table = res_table(res_list);
  let cpu_info = format!(
    "{} # {}Core",
    sysinfo::System::new_all().cpus()[0].brand(),
    sysinfo::System::new_all().cpus().len()
  );
  _ = write!(
    &mut writer,
    r#"<!DOCTYPE html>
  <html>
  <head>
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>Benchmark Result</title>
  <style type="text/css">
  {ccs_text}
  </style>
  </head>
  <body><div class="body"><h2>Benchmark Result</h2><p>Platform: {cpu_info}</p><div class="absolute">
  {info_table}
  {res_table}
  </div><div id="footer"><p>This report was generated by <a href="https://github.com/bheisler/criterion.rs">Criterion.rs</a>, a statistics-driven benchmarking library in Rust.</p></div></body></html>"#
  );
  println!(
    "{}",
    res_table
      .replace("href=\"./", "href=\"https://zao111222333.github.io/liberty-db/bench/"),
  );
}
