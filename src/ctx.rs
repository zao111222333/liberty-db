pub trait Ctx: serde::Serialize + serde::de::DeserializeOwned + Default {
  type Cell: crate::cell::CellCtx
    + core::fmt::Debug
    + Clone
    + Default
    + serde::Serialize
    + serde::de::DeserializeOwned;
  type Library: core::fmt::Debug
    + Clone
    + Default
    + serde::Serialize
    + serde::de::DeserializeOwned;
  /// TODO: Specify more types of Ctx
  type Dummy: core::fmt::Debug
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
  type Cell = crate::cell::DefaultCellCtx;
  type Library = ();
  type Dummy = ();
}
