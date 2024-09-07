use criterion::{black_box, Criterion};
use std::{
  ffi::{c_char, c_void, CString, OsStr},
  fs::{metadata, read_to_string, File},
  io::{BufWriter, Cursor, Write},
  panic,
  path::{Path, PathBuf},
  time::Duration,
};

extern "C" {
  fn ot_parse_lib(s: *const c_char) -> *mut c_void;
  fn ot_write_lib(ptr: *mut c_void);
  fn ot_drop_lib(ptr: *mut c_void);
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

type ParseFn<L> = fn(&str) -> Result<L, ()>;
type WriteFn<L> = fn(&L) -> Result<(), ()>;
type DropFn<L> = fn(L);

#[allow(dead_code)]
struct ProjCtx<L> {
  name: &'static str,
  info: &'static str,
  parse_fn: ParseFn<L>,
  write_fn: WriteFn<L>,
  drop_fn: DropFn<L>,
}
impl<L> ProjCtx<L> {
  fn html(&self) -> String {
    format!("<th><a href=\"{}\">{}</a></th>", self.info, self.name)
  }
  fn fail(&self, file_path: &str, parse_or_write: bool) {
    println!(
      "\n{}{}",
      console::style(format!(
        "[{}] {file_path} / {}",
        if parse_or_write { "parse" } else { "write" },
        self.name
      ))
      .red(),
      console::style(" [FAIL] ").bold().red()
    );
  }
  fn id(&self) -> String {
    format!(" {}", self.name)
  }
  fn collect_result(&self, group_path: &str) -> BenchResult {
    let data = read_to_string(
      Path::new("target/criterion")
        .join(group_path)
        .join(self.id())
        .join("new/estimates.json"),
    )
    .unwrap();
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    if let serde_json::Value::Number(n) = &v["mean"]["point_estimate"] {
      if let Some(f) = n.as_f64() {
        return BenchResult::Ok {
          run_time: Duration::from_nanos(f as u64),
          path: Path::new(group_path)
            .join(self.id())
            .join("report")
            .display()
            .to_string(),
        };
      }
    }
    BenchResult::Fail
  }
  fn parse_bench(
    &self,
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    file_path: &str,
    group_path: &str,
  ) -> BenchResult {
    let library_string = read_to_string(file_path).unwrap();
    let s = library_string.as_str();
    if let Ok(Ok(try_l)) = panic::catch_unwind(|| (self.parse_fn)(black_box(s))) {
      (self.drop_fn)(try_l);
      group.bench_function(self.id(), |b| {
        b.iter(|| (self.parse_fn)(black_box(s)).map(|l| (self.drop_fn)(l)))
      });
      self.collect_result(group_path)
    } else {
      self.fail(file_path, true);
      BenchResult::Fail
    }
  }
  fn write_bench(
    &self,
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    file_path: &str,
    group_path: &str,
  ) -> BenchResult {
    let library_string = read_to_string(file_path).unwrap();
    let s = library_string.as_str();
    if let Ok(Ok(l)) = panic::catch_unwind(|| (self.parse_fn)(black_box(s))) {
      if (self.write_fn)(black_box(&l)).is_ok() {
        group.bench_function(self.id(), |b| b.iter(|| (self.write_fn)(black_box(&l))));
        (self.drop_fn)(l);
        return self.collect_result(group_path);
      }
      (self.drop_fn)(l);
    }
    self.fail(file_path, false);
    BenchResult::Fail
  }
}

const LIBERTY_DB: ProjCtx<liberty_db::Library> = ProjCtx {
  name: "liberty-db (this)",
  info: "https://crates.io/crates/liberty-db",
  parse_fn: |s| liberty_db::Library::parse_lib(s).map_err(|_| ()),
  write_fn: |l| {
    _ = black_box(l.to_string());
    Ok(())
  },
  drop_fn: drop,
};

const LIBERTY_IO: ProjCtx<liberty_io::Group> = ProjCtx {
  name: "liberty-io (rust)",
  info: "https://crates.io/crates/liberty-io",
  parse_fn: |s| {
    let mut cursor = Cursor::new(s.as_bytes());
    liberty_io::read_liberty_bytes(&mut cursor).map_err(|_| ())
  },
  write_fn: |_| Err(()),
  drop_fn: drop,
};

const LIBERTYPARSE: ProjCtx<libertyparse::Liberty> = ProjCtx {
  name: "libertyparse (rust)",
  info: "https://crates.io/crates/libertyparse",
  parse_fn: |s| libertyparse::Liberty::parse_str(s).map_err(|_| ()),
  write_fn: |_| Err(()),
  drop_fn: drop,
};

const OPEN_TIMER: ProjCtx<*mut c_void> = ProjCtx {
  name: "OpenTimer (cpp)",
  info: "https://github.com/OpenTimer/OpenTimer",
  parse_fn: |s| {
    let cstr = CString::new(s).unwrap();
    Ok(unsafe { ot_parse_lib(cstr.as_ptr()) })
  },
  write_fn: |l| {
    unsafe { ot_write_lib(*l) };
    Ok(())
  },
  drop_fn: |l| unsafe { ot_drop_lib(l) },
};

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
type ResList = Vec<(String, [(String, [BenchResult; 4]); 2])>;
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
      let mut res: [BenchResult; 4] = Default::default();
      let mut parse_group = c.benchmark_group(&group_name);
      res[0] = LIBERTY_DB.parse_bench(&mut parse_group, &file_path, &group_path);
      res[1] = LIBERTY_IO.parse_bench(&mut parse_group, &file_path, &group_path);
      res[2] = LIBERTYPARSE.parse_bench(&mut parse_group, &file_path, &group_path);
      res[3] = OPEN_TIMER.parse_bench(&mut parse_group, &file_path, &group_path);
      parse_group.finish();
      (group_path, res)
    };
    let write_res = {
      let group_name = format!("[write] {file_path} ");
      let group_path = group_name2path(&group_name);
      let mut res: [BenchResult; 4] = Default::default();
      let mut write_group = c.benchmark_group(&group_name);
      res[0] = LIBERTY_DB.write_bench(&mut write_group, &file_path, &group_path);
      res[1] = LIBERTY_IO.write_bench(&mut write_group, &file_path, &group_path);
      res[2] = LIBERTYPARSE.write_bench(&mut write_group, &file_path, &group_path);
      res[3] = OPEN_TIMER.write_bench(&mut write_group, &file_path, &group_path);
      write_group.finish();
      (group_path, res)
    };
    res_list.push((file_path, [parse_res, write_res]));
  }
  res_list
}

fn make_table(res_list: ResList) -> String {
  let mut parse_table = format!(
    "<table><thead><tr>{}{}{}{}<th>Test Case</th></tr></thead><tbody>",
    LIBERTY_DB.html(),
    LIBERTY_IO.html(),
    LIBERTYPARSE.html(),
    OPEN_TIMER.html(),
  );
  let mut write_table = parse_table.clone();
  for (file_path, [(parse_path, parse_res), (write_path, write_res)]) in res_list {
    parse_table += &format!(
      "<tr>{}{}{}{}<td><a href=\"./{parse_path}/report/\">{file_path}</a></td></tr>",
      parse_res[0].html(),
      parse_res[1].html(),
      parse_res[2].html(),
      parse_res[3].html(),
    );
    write_table += &format!(
      "<tr>{}{}{}{}<td><a href=\"./{write_path}/report\">{file_path}</a></td></tr>",
      write_res[0].html(),
      write_res[1].html(),
      write_res[2].html(),
      write_res[3].html(),
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
  let ccs_text = "body {font: 14px Helvetica Neue;text-rendering: optimizelegibility;}.body {width: 960px;margin: auto;}th {font-weight: 200}th,td {padding-right: 3px;padding-bottom: 3px;}a:link {color: #1F78B4;text-decoration: none;}th.ci-bound {opacity: 0.6}td.ci-bound {opacity: 0.5}.stats {width: 80%;margin: auto;display: flex;}.additional_stats {flex: 0 0 60%}.additional_plots {flex: 1}h2 {font-size: 36px;font-weight: 300;}h3 {font-size: 24px;font-weight: 300;}#footer {height: 40px;background: #888;color: white;font-size: larger;font-weight: 300;}#footer a {color: white;text-decoration: underline;}#footer p {text-align: center}";
  let table = make_table(res_list);
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
{table}
</div><div id="footer"><p>This report was generated by <a href="https://github.com/bheisler/criterion.rs">Criterion.rs</a>, a statistics-driven benchmarking library in Rust.</p></div></body></html>"#
  );
  println!(
    "{}",
    table.replace("href=\"./", "href=\"https://zao111222333.github.io/liberty-db/bench/"),
  );
}
