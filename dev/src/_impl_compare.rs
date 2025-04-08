use crate::{ProjInfo, ProjLibrary, TypedSupport};
use criterion::black_box;
impl ProjLibrary for liberty_db_incoming::Library<liberty_db_incoming::DefaultCtx> {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-db",
    url: "INCOMING_URL",
    lang: "rust",
    version: "incoming",
    typed_support: TypedSupport::AllTyped,
    parsed_boolexpr: true,
    other: "incoming version",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::parse_lib(s, None).map_err(|_| ())
  }
  fn write(&self) -> Result<(), ()> {
    _ = black_box(self.to_string());
    Ok(())
  }
}

impl ProjLibrary for liberty_db_latest::Library<liberty_db_latest::DefaultCtx> {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-db",
    url: "BASE_URL",
    lang: "rust",
    version: "base",
    typed_support: TypedSupport::AllTyped,
    parsed_boolexpr: true,
    other: "current version",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::parse_lib(s, None).map_err(|_| ())
  }
  fn write(&self) -> Result<(), ()> {
    _ = black_box(self.to_string());
    Ok(())
  }
}
