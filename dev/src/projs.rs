#[macro_export]
macro_rules! gen_projs {
  ( $( ($name:tt,$type:ty) ),* $(,)? ) => {
    {
      #[derive(Debug, strum_macros::EnumIter)]
      enum Projs {
        $( $name, )*
      }
      use $crate::ProjLibrary;
      impl $crate::Proj for Projs {
        fn info(&self) -> $crate::ProjInfo {
          match self {
            $( Self::$name => <$type>::INFO, )*
          }
        }
        fn parse_bench(
          &self, group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
          file_path: &str, group_path: &str,
        ) -> $crate::BenchResult {
          match self {
            $( Self::$name => <$type>::parse_bench(group, file_path, group_path), )*
          }
        }
        fn write_bench(
          &self, group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
          file_path: &str, group_path: &str,
        ) -> $crate::BenchResult {
          match self {
            $( Self::$name => <$type>::write_bench(group, file_path, group_path), )*
          }
        }
      }
      <Projs as strum::IntoEnumIterator>::iter()
    }
  };
}
