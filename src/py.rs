use pyo3::{prelude::*, types::PyTuple};
use pyo3_stub_gen::{PyStubType, TypeInfo};

use crate::{
  cell::PgType,
  expression::logic,
  units::{
    CapacitiveLoadUnit, CurrentUnit, LeakagePowerUnit, PullingResistanceUnit, TimeUnit,
    VoltageUnit,
  },
};
#[macro_export]
macro_rules! impl_py_enum {
  ($t:path) => {
    impl<'py> FromPyObject<'_, 'py> for $t {
      type Error = pyo3::PyErr;
      #[inline]
      fn extract(ob: pyo3::Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        match ob.extract::<alloc::borrow::Cow<'_, str>>()?.parse() {
          Ok(t) => Ok(t),
          Err(_) => {
            Err(pyo3::exceptions::PyValueError::new_err("Matching variant not found"))
          }
        }
      }
    }
    impl<'py> IntoPyObject<'py> for $t {
      type Target = pyo3::types::PyString;
      type Output = Bound<'py, Self::Target>;
      type Error = PyErr;
      #[inline]
      fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &'static str = self.into();
        Ok(pyo3::types::PyString::new(py, s))
      }
    }
    impl PyStubType for $t {
      #[inline]
      fn type_output() -> TypeInfo {
        use strum::IntoEnumIterator as _;
        TypeInfo {
          name: format!(
            "typing.Literal{:?}",
            Self::iter().into_iter().map(|t| t.to_string()).collect::<Vec<_>>()
          ),
          import: ["typing".into()].into_iter().collect(),
        }
      }
    }
  };
}

impl_py_enum!(logic::Edge);
impl_py_enum!(TimeUnit);
impl_py_enum!(VoltageUnit);
impl_py_enum!(CurrentUnit);
impl_py_enum!(PullingResistanceUnit);
impl_py_enum!(LeakagePowerUnit);
impl_py_enum!(PgType);

impl<'py> FromPyObject<'_, 'py> for CapacitiveLoadUnit {
  type Error = PyErr;

  #[inline]
  fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
    let (val, s_ff_pf) = ob.extract::<(f64, String)>()?;
    match s_ff_pf.as_str() {
      "ff" => Ok(Self::FF(val)),
      "pf" => Ok(Self::PF(val)),
      _ => Err(pyo3::exceptions::PyValueError::new_err("Matching variant not found")),
    }
  }
}
impl<'py> IntoPyObject<'py> for CapacitiveLoadUnit {
  type Target = PyTuple;
  type Output = Bound<'py, Self::Target>;
  type Error = PyErr;
  #[inline]
  fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
    match self {
      Self::FF(val) => (val, "ff"),
      Self::PF(val) => (val, "pf"),
    }
    .into_pyobject(py)
  }
}
impl PyStubType for CapacitiveLoadUnit {
  #[inline]
  fn type_output() -> TypeInfo {
    TypeInfo::with_module(
      "tuple[builtins.float, typing.Literal[\"ff\", \"pf\"]]",
      "typing".into(),
    )
  }
}
