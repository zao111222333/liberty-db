use crate::common::table::{
  CompactTableCtx, DefaultCompactTableCtx, DefaultTableCtx, TableCtx,
};

pub trait Ctx: serde::Serialize + serde::de::DeserializeOwned + Default + Clone {
  type Library: core::fmt::Debug
    + Clone
    + Default
    + serde::Serialize
    + serde::de::DeserializeOwned;
  type Cell: crate::cell::CellCtx
    + core::fmt::Debug
    + Clone
    + Default
    + serde::Serialize
    + serde::de::DeserializeOwned;
  type FFLatch: core::fmt::Debug
    + Clone
    + Default
    + serde::Serialize
    + serde::de::DeserializeOwned;
  type Pin: core::fmt::Debug
    + Clone
    + Default
    + serde::Serialize
    + serde::de::DeserializeOwned;
  type Timing: core::fmt::Debug
    + Clone
    + Default
    + serde::Serialize
    + serde::de::DeserializeOwned;
  type InternalPower: core::fmt::Debug
    + Clone
    + Default
    + serde::Serialize
    + serde::de::DeserializeOwned;
  type Table: TableCtx<Self>
    + core::fmt::Debug
    + Clone
    + Default
    + serde::Serialize
    + serde::de::DeserializeOwned;
  type CompactTable: CompactTableCtx<Self>
    + core::fmt::Debug
    + Clone
    + Default
    + serde::Serialize
    + serde::de::DeserializeOwned;
  /// TODO: Specify more types of Ctx
  type Other: core::fmt::Debug
    + Clone
    + Default
    + serde::Serialize
    + serde::de::DeserializeOwned;
}

/// Default context config, you can specify
/// other context for cell & library according your requirements.
#[derive(Debug, Default, Clone, Copy)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DefaultCtx;
impl Ctx for DefaultCtx {
  type Library = ();
  type Cell = crate::cell::DefaultCellCtx;
  type FFLatch = ();
  type Pin = ();
  type Timing = ();
  type InternalPower = ();
  type Table = DefaultTableCtx<Self>;
  type CompactTable = DefaultCompactTableCtx<Self>;
  type Other = ();
}
