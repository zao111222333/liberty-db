#[cfg(feature = "bench")]
pub mod _impl_bench;
#[cfg(feature = "compare")]
pub mod _impl_compare;
pub mod projs;
use criterion::{black_box, Criterion};
use dev_utils::all_files;
use itertools::Itertools as _;
use serde_json::Value;
use std::{fs::read_to_string, panic, path::Path, time::Duration};

#[cfg(feature = "compare")]
pub const COMPARE: bool = true;
#[cfg(not(feature = "compare"))]
pub const COMPARE: bool = false;

pub enum TypedSupport {
  AllTyped,
  PartialTyped,
  AstOnly,
}
pub struct ProjInfo {
  pub name: &'static str,
  pub url: &'static str,
  pub lang: &'static str,
  pub version: &'static str,
  pub typed_support: TypedSupport,
  pub parsed_boolexpr: bool,
  pub other: &'static str,
}
#[allow(clippy::result_unit_err)]
pub trait ProjLibrary: Sized {
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
        "[{}] {file_path} /{}",
        if parse_or_write { "parse" } else { "write" },
        Self::id()
      ))
      .red(),
      console::style(" [FAIL] ").bold().red()
    );
    BenchResult::Fail
  }
  fn id() -> String {
    format!(" {}-{}", Self::INFO.name, Self::INFO.version)
  }
  fn collect_result(group_path: &str) -> BenchResult {
    read_to_string(
      Path::new("../target/criterion")
        .join(group_path)
        .join(Self::id())
        .join("new/estimates.json"),
    )
    .ok()
    .and_then(|s| serde_json::from_str::<Value>(&s).ok())
    .map(|v| v["mean"]["point_estimate"].clone())
    .and_then(|v| if let Value::Number(n) = v { n.as_f64() } else { None })
    .map(|f| Duration::from_secs_f64(f / 1e9))
    .map_or(BenchResult::Fail, |run_time| {
      let path = Path::new(group_path)
        .join(Self::id())
        .join("report/")
        .display()
        .to_string();
      let change = read_to_string(
        Path::new("../target/criterion")
          .join(group_path)
          .join(Self::id())
          .join("change/estimates.json"),
      )
      .ok()
      .and_then(|s| serde_json::from_str::<Value>(&s).ok())
      .map(|v| v["mean"]["point_estimate"].clone())
      .and_then(|v| if let Value::Number(n) = v { n.as_f64() } else { None })
      .map(|f| f * 100.0);
      BenchResult::Ok { path, run_time, change }
    })
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

#[rustfmt::skip]
pub trait Proj {
  fn info(&self) -> ProjInfo;
  fn info_html(&self) -> String {
    let info = self.info();
    let name = if COMPARE {
      format!("{}", info.name)
    } else {
      format!("<a href=\"{}\">{}</a>",info.url, info.name)
    };
    format!(
      "<tr><th style=\"text-align:left;padding-left:5px\">{name}</th><th>{}</th><th>{}</th>{}<th>{}</th><th>{}</th></tr>", 
      info.lang,
      info.version,
      match info.typed_support{
        TypedSupport::AllTyped => "<th>&#10003</th><th></th><th></th>",
        TypedSupport::PartialTyped => "<th></th><th>&#10003</th><th></th>",
        TypedSupport::AstOnly => "<th></th><th></th><th>&#10003</th>",
      },
      if info.parsed_boolexpr{"&#10003"}else{""},
      info.other
    )
  }
  fn html(&self) -> String {
    let info = self.info();
    if COMPARE {
      format!("<th>{}</th>", info.name)
    } else {
      format!("<th><a href=\"{}\">{}</a></th>", info.url, info.name)
    }
  }
  fn parse_bench(
    &self,
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    file_path: &str,
    group_path: &str,
  ) -> BenchResult;
  fn write_bench(
    &self,
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    file_path: &str,
    group_path: &str,
  ) -> BenchResult;
}

#[derive(Debug, Default)]
pub enum BenchResult {
  #[default]
  Fail,
  Ok {
    path: String,
    run_time: Duration,
    change: Option<f64>,
  },
}
impl BenchResult {
  pub fn html(&self) -> String {
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
      Self::Ok { path, run_time, change: _ } => {
        if COMPARE {
          format!(
            "<td style=\"text-align:right;padding-right:10px;\"><a style=\"color:MediumSeaGreen;\">{}</a></td>",
            format_duration(run_time)
          )
        } else {
          format!(
            "<td style=\"text-align:right;padding-right:10px;\"><a href=\"./{path}\" style=\"color:MediumSeaGreen;\">{}</a></td>",
            format_duration(run_time)
          )
        }
      }
    }
  }
}

pub type ResList = Vec<(String, [(String, Vec<BenchResult>); 2])>;
pub fn bench_all(
  c: &mut Criterion,
  projs: impl Clone + Iterator<Item = impl Proj>,
  regression: bool,
) -> ResList {
  let group_path_max_len = 64;
  let group_name2path = |group_name: &String| {
    let mut group_path = group_name.replace('/', "_");
    if group_path.len() > group_path_max_len {
      _ = group_path.split_off(group_path_max_len);
    }
    group_path
  };
  let mut res_list = Vec::new();
  let tag = if regression { "regression" } else { "comparsion" };
  for path in all_files("tech") {
    let file_path = path.display().to_string();
    let parse_res = {
      let group_name = format!("[{tag}-parse] {file_path} ");
      let group_path = group_name2path(&group_name);
      let mut parse_group = c.benchmark_group(&group_name);
      let res = projs
        .clone()
        .map(|proj| proj.parse_bench(&mut parse_group, &file_path, &group_path))
        .collect();
      parse_group.finish();
      (group_path, res)
    };
    let write_res = {
      let group_name = format!("[{tag}-write] {file_path} ");
      let group_path = group_name2path(&group_name);
      let mut write_group = c.benchmark_group(&group_name);
      let res = projs
        .clone()
        .map(|proj| proj.write_bench(&mut write_group, &file_path, &group_path))
        .collect();
      write_group.finish();
      (group_path, res)
    };
    res_list.push((file_path, [parse_res, write_res]));
  }
  res_list
}

pub fn info_table(projs: impl Clone + Iterator<Item = impl Proj>) -> String {
  format!(
    "<div class=\"info-table\"><table><thead><tr><th rowspan=\"2\" style=\"font-weight:bold;\">Project</th><th rowspan=\"2\" style=\"font-weight:bold;\">Lang</th><th rowspan=\"2\" style=\"font-weight:bold;\">Version</th><th colspan=\"3\" style=\"text-align:center;font-weight:bold;\">Type Support</th><th rowspan=\"2\" style=\"font-weight:bold;\">Boolean<br>Expression</th><th rowspan=\"2\" style=\"font-weight:bold;\">Comment</th></tr><tr><th>All</th><th>Partly</th><th>AST only</th></tr></thead><tbody>{}</tbody></table></div>", 
    projs.map(|proj| proj.info_html()).join(""),
  )
}
pub fn res_table(
  res_list: ResList,
  projs: impl Clone + Iterator<Item = impl Proj>,
  regression: bool,
) -> String {
  let mut parse_table = format!(
    "<table><thead><tr>{}<th>Test Case</th></tr></thead>",
    if regression {
      projs.map(|proj| format!("<th>{}</th>", proj.info().version)).join("")
    } else {
      projs.map(|proj| proj.html()).join("")
    }
  );
  let mut write_table = parse_table.clone();
  for (file_path, [(parse_path, parse_res), (write_path, write_res)]) in res_list {
    if COMPARE {
      parse_table += &format!(
        "<tr>{}<td>{file_path}</td></tr>",
        parse_res.iter().map(|res| res.html()).join("")
      );
      write_table += &format!(
        "<tr>{}<td>{file_path}</td></tr>",
        write_res.iter().map(|res| res.html()).join("")
      );
    } else {
      parse_table += &format!(
        "<tr>{}<td><a href=\"./{parse_path}/report/\">{file_path}</a></td></tr>",
        parse_res.iter().map(|res| res.html()).join("")
      );
      write_table += &format!(
        "<tr>{}<td><a href=\"./{write_path}/report/\">{file_path}</a></td></tr>",
        write_res.iter().map(|res| res.html()).join("")
      );
    }
  }
  parse_table += "</tbody></table>";
  write_table += "</tbody></table>";
  format!("<h3>Parse Performance Comparison</h3>{parse_table}<h3>Write Performance Comparison</h3>{write_table}")
}

pub fn run_bench(
  projs: impl Clone + Iterator<Item = impl Proj>,
  regression: bool,
) -> String {
  let mut criterion = Criterion::default()
    .sample_size(50)
    .with_output_color(true)
    .warm_up_time(Duration::from_millis(100))
    .configure_from_args();
  let res_list = bench_all(&mut criterion, projs.clone(), regression);
  criterion.final_summary();

  let mut info_table = if COMPARE { String::new() } else { info_table(projs.clone()) };
  let res_table = res_table(res_list, projs, regression);
  info_table += &res_table;
  info_table
}
