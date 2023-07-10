mod demo;
mod parser_bench;

fn main() {
  demo::demo();
  parser_bench::test_all_lib_files();
}
