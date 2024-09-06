use criterion::{black_box, Criterion};
use std::{
  ffi::{c_char, CString, OsStr},
  fmt::Write,
  fs::{metadata, read_to_string},
  io::Cursor,
  panic,
  path::PathBuf,
  time::Duration,
};

// 声明外部 C 函数
extern "C" {
  fn ot_parse_lib(path: *const c_char);
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
struct ProjCtx<L> {
  name: &'static str,
  info: &'static str,
  parse_fn: ParseFn<L>,
  write_fn: WriteFn<L>,
}
impl<L> ProjCtx<L> {
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
  fn parse_bench(
    &self,
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    file_path: &str,
  ) {
    let library_string = read_to_string(file_path).unwrap();
    let s = library_string.as_str();
    if let Ok(Ok(_)) = panic::catch_unwind(|| (self.parse_fn)(black_box(s))) {
      group.bench_function(format!(" {}", self.name).as_str(), |b| {
        b.iter(|| (self.parse_fn)(black_box(s)))
      });
    } else {
      self.fail(file_path, true);
    }
  }
  fn write_bench(
    &self,
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    file_path: &str,
  ) {
    let library_string = read_to_string(file_path).unwrap();
    let s = library_string.as_str();
    if let Ok(Ok(l)) = panic::catch_unwind(|| (self.parse_fn)(black_box(s))) {
      group.bench_function(format!(" {}", self.name).as_str(), |b| {
        b.iter(|| (self.write_fn)(black_box(&l)))
      });
    } else {
      self.fail(file_path, false);
    }
  }
}

const LIBERTY_DB: ProjCtx<liberty_db::Library> = ProjCtx {
  name: "liberty-db",
  info: "https://crates.io/crates/liberty-db",
  parse_fn: |s| liberty_db::Library::parse_lib(s).map_err(|_| ()),
  write_fn: |l: &liberty_db::Library| {
    let mut buf = String::new();
    match write!(buf, "{l}") {
      Ok(_) => Ok(()),
      Err(_) => Err(()),
    }
  },
};

const LIBERTY_IO: ProjCtx<liberty_io::Group> = ProjCtx {
  name: "liberty-io",
  info: "https://crates.io/crates/liberty-io",
  parse_fn: |s| {
    let mut cursor = Cursor::new(s.as_bytes());
    liberty_io::read_liberty_bytes(&mut cursor).map_err(|_| ())
  },
  write_fn: |_| Err(()),
};

const LIBERTYPARSE: ProjCtx<libertyparse::Liberty> = ProjCtx {
  name: "libertyparse",
  info: "https://crates.io/crates/libertyparse",
  parse_fn: |s| libertyparse::Liberty::parse_str(s).map_err(|_| ()),
  write_fn: |_| Err(()),
};

fn bench_all(c: &mut Criterion) {
  for path in all_files() {
    let file_path = path.display().to_string();
    let file_path_c_str = CString::new(file_path.as_str()).unwrap();
    {
      let mut parse_group = c.benchmark_group(&format!("[parse] {file_path} "));
      LIBERTY_DB.parse_bench(&mut parse_group, &file_path);
      LIBERTY_IO.parse_bench(&mut parse_group, &file_path);
      LIBERTYPARSE.parse_bench(&mut parse_group, &file_path);
      // 调用 C++ 函数
      if let Ok(_) = panic::catch_unwind(|| unsafe {
        ot_parse_lib(file_path_c_str.as_ptr());
      }) {
        parse_group.bench_function(format!(" {}", "OpenTimer").as_str(), |b| {
          b.iter(|| unsafe {
            ot_parse_lib(file_path_c_str.as_ptr());
          })
        });
      } else {
        // self.fail(file_path, true);
      }
      parse_group.finish();
    }
    {
      let mut write_group = c.benchmark_group(&format!("[write] {file_path} "));
      LIBERTY_DB.write_bench(&mut write_group, &file_path);
      LIBERTY_IO.write_bench(&mut write_group, &file_path);
      LIBERTYPARSE.write_bench(&mut write_group, &file_path);
      write_group.finish();
    }
  }
}

fn main() {
  let mut criterion = Criterion::default()
    .sample_size(100)
    .with_output_color(true)
    .warm_up_time(Duration::from_millis(1000))
    .configure_from_args();
  bench_all(&mut criterion);
  criterion.final_summary();
}
