use crate::table::{
  CompactTableCtx, DefaultCompactTableCtx, DefaultPolyTableCtx, DefaultPropagationTable,
  DefaultTableCtx, PolyTableCtx, PropagationTableCtx, TableCtx,
};

pub trait Ctx:
  'static
  + serde::Serialize
  + serde::de::DeserializeOwned
  + Default
  + Clone
  + core::fmt::Debug
{
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
  type PropagationTable: PropagationTableCtx<Self>
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
  type PolyTable: PolyTableCtx<Self>
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
  type PropagationTable = DefaultPropagationTable<Self>;
  type CompactTable = DefaultCompactTableCtx<Self>;
  type PolyTable = DefaultPolyTableCtx<Self>;
  type Other = ();
}
