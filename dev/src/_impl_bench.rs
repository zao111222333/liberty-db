#![expect(unused_imports)]
use crate::{ProjInfo, ProjLibrary, TypedSupport};
use criterion::black_box;
use std::{
  ffi::{CString, c_char, c_int, c_void},
  io::Cursor,
  str::FromStr,
};
impl ProjLibrary for liberty_db_latest::Library<liberty_db_latest::DefaultCtx> {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-db",
    url: "https://crates.io/crates/liberty-db",
    lang: "rust",
    version: "latest",
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

impl ProjLibrary for liberty_db_0p10p0::Library<liberty_db_0p10p0::DefaultCtx> {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-db",
    url: "https://crates.io/crates/liberty-db/0.10.0",
    lang: "rust",
    version: "0.10.0",
    typed_support: TypedSupport::AllTyped,
    parsed_boolexpr: true,
    other: "published at 2025-03-11",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::parse_lib(s).map_err(|_| ())
  }
  fn write(&self) -> Result<(), ()> {
    _ = black_box(self.to_string());
    Ok(())
  }
}

impl ProjLibrary for liberty_db_0p9p6::Library<liberty_db_0p9p6::DefaultCtx> {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-db",
    url: "https://crates.io/crates/liberty-db/0.9.6",
    lang: "rust",
    version: "0.9.6",
    typed_support: TypedSupport::AllTyped,
    parsed_boolexpr: true,
    other: "published at 2025-03-11",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::parse_lib(s).map_err(|_| ())
  }
  fn write(&self) -> Result<(), ()> {
    _ = black_box(self.to_string());
    Ok(())
  }
}

impl ProjLibrary for liberty_db_0p8p3::Library<liberty_db_0p8p3::DefaultCtx> {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-db",
    url: "https://crates.io/crates/liberty-db/0.8.3",
    lang: "rust",
    version: "0.8.3",
    typed_support: TypedSupport::AllTyped,
    parsed_boolexpr: true,
    other: "published at 2024-12-01",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::parse_lib(s).map_err(|_| ())
  }
  fn write(&self) -> Result<(), ()> {
    _ = black_box(self.to_string());
    Ok(())
  }
}

impl ProjLibrary for liberty_db_0p7p4::Library {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-db",
    url: "https://crates.io/crates/liberty-db/0.7.4",
    lang: "rust",
    version: "0.7.4",
    typed_support: TypedSupport::AllTyped,
    parsed_boolexpr: true,
    other: "published at 2024-12-01",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::parse_lib(s).map_err(|_| ())
  }
  fn write(&self) -> Result<(), ()> {
    _ = black_box(self.to_string());
    Ok(())
  }
}

impl ProjLibrary for liberty_db_0p6p14::Library {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-db",
    url: "https://crates.io/crates/liberty-db/0.6.14",
    lang: "rust",
    version: "0.6.14",
    typed_support: TypedSupport::AllTyped,
    parsed_boolexpr: true,
    other: "published at 2024-11-07",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::parse_lib(s).map_err(|_| ())
  }
  fn write(&self) -> Result<(), ()> {
    _ = black_box(self.to_string());
    Ok(())
  }
}

impl ProjLibrary for liberty_db_0p5p9::Library {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-db",
    url: "https://crates.io/crates/liberty-db/0.5.9",
    lang: "rust",
    version: "0.5.9",
    typed_support: TypedSupport::AllTyped,
    parsed_boolexpr: true,
    other: "published at 2024-08-27",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::parse_lib(s).map_err(|_| ())
  }
  fn write(&self) -> Result<(), ()> {
    _ = black_box(self.to_string());
    Ok(())
  }
}

impl ProjLibrary for liberty_db_0p4p13::Library {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-db",
    url: "https://crates.io/crates/liberty-db/0.4.13",
    lang: "rust",
    version: "0.4.13",
    typed_support: TypedSupport::AllTyped,
    parsed_boolexpr: true,
    other: "published at 2024-08-13",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::parse(s).map_err(|_| ())
  }
  fn write(&self) -> Result<(), ()> {
    _ = black_box(self.to_string());
    Ok(())
  }
}

impl ProjLibrary for liberty_db_0p3p1::library::Library {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-db",
    url: "https://crates.io/crates/liberty-db/0.3.1",
    lang: "rust",
    version: "0.3.1",
    typed_support: TypedSupport::PartialTyped,
    parsed_boolexpr: false,
    other: "published at 2023-08-03",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::parse(s).map_err(|_| ())
  }
  fn write(&self) -> Result<(), ()> {
    let mut s = String::new();
    _ = self.fmt(&mut s);
    _ = black_box(s);
    Ok(())
  }
}

impl ProjLibrary for liberty_io::Group {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty-io",
    url: "https://crates.io/crates/liberty-io",
    lang: "rust",
    version: "0.0.4",
    typed_support: TypedSupport::AstOnly,
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
    version: "0.3.0",
    typed_support: TypedSupport::PartialTyped,
    parsed_boolexpr: true,
    other: "",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::parse_str(s).map_err(|_| ())
  }
}

#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Copy)]
#[cfg(target_os = "linux")]
pub struct OpenTimerLibrary(*mut c_void);
#[cfg(target_os = "linux")]
extern "C" {
  fn ot_parse_lib(s: *const c_char) -> *mut c_void;
  fn ot_write_lib(ptr: *mut c_void, debug: c_int);
  fn ot_drop_lib(ptr: *mut c_void);
}
#[cfg(target_os = "linux")]
impl ProjLibrary for OpenTimerLibrary {
  const INFO: ProjInfo = ProjInfo {
    name: "OpenTimer",
    url: "https://github.com/OpenTimer/OpenTimer/tree/a57d03b39886c1e2f113c1a893f5b3fad9199a52",
    version: "2",
    lang: "C++17",
    typed_support: TypedSupport::PartialTyped,
    parsed_boolexpr: true,
    other: "STA tool's liberty component",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    let cstr = CString::new(s).unwrap();
    Ok(OpenTimerLibrary(unsafe { ot_parse_lib(cstr.as_ptr()) }))
  }
  fn write(&self) -> Result<(), ()> {
    unsafe { ot_write_lib(self.0, 0) };
    Ok(())
  }
  #[allow(clippy::not_unsafe_ptr_arg_deref)]
  fn drop(self) {
    unsafe { ot_drop_lib(self.0) }
  }
}

#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Copy)]
#[cfg(target_os = "linux")]
pub struct Si2drLibertyLibrary(*mut c_void);
#[cfg(target_os = "linux")]
extern "C" {
  fn si2dr_liberty_parse_lib(s: *const c_char) -> *mut c_void;
  fn si2dr_liberty_write_lib(ptr: *mut c_void, debug: c_int);
  fn si2dr_liberty_drop_lib(ptr: *mut c_void);
}
#[cfg(target_os = "linux")]
impl ProjLibrary for Si2drLibertyLibrary {
  const INFO: ProjInfo = ProjInfo {
    name: "si2dr_liberty",
    url: "https://github.com/csguth/LibertyParser",
    version: "1.0",
    lang: "C",
    typed_support: TypedSupport::AllTyped,
    parsed_boolexpr: true,
    other: "Synopsys's version at 2005, many attributes are not supported",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    let cstr = CString::new(s).unwrap();
    Ok(Si2drLibertyLibrary(unsafe { si2dr_liberty_parse_lib(cstr.as_ptr()) }))
  }
  fn write(&self) -> Result<(), ()> {
    unsafe { si2dr_liberty_write_lib(self.0, 0) };
    Ok(())
  }
  #[allow(clippy::not_unsafe_ptr_arg_deref)]
  fn drop(self) {
    unsafe { si2dr_liberty_drop_lib(self.0) }
  }
}

impl ProjLibrary for liberty2json::Liberty {
  const INFO: ProjInfo = ProjInfo {
    name: "liberty2json",
    url: "https://github.com/erihsu/liberty2json/tree/7d0a4f233f143fce9c2844208f4d48033622d93f",
    lang: "rust",
    version: "0.1.0",
    typed_support: TypedSupport::AstOnly,
    parsed_boolexpr: false,
    other: "",
  };
  fn parse(s: &str) -> Result<Self, ()> {
    Self::from_str(s).map_err(|_| ())
  }
}
