use pyo3::{
  exceptions::PyValueError,
  prelude::*,
  types::{PyString, PyTuple},
};
use pyo3_stub_gen::{PyStubType, TypeInfo};

use crate::{
  cell::PgType,
  units::{
    CapacitiveLoadUnit, CurrentUnit, LeakagePowerUnit, PullingResistanceUnit, TimeUnit,
    VoltageUnit,
  },
};

#[macro_export]
macro_rules! impl_py_enum {
  ($t:tt) => {
    impl<'py> FromPyObject<'py> for $t {
      #[inline]
      fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        match ob.extract::<alloc::borrow::Cow<'_, str>>()?.parse() {
          Ok(t) => Ok(t),
          Err(_) => Err(PyValueError::new_err("Matching variant not found")),
        }
      }
    }
    impl<'py> IntoPyObject<'py> for $t {
      type Target = PyString;
      type Output = Bound<'py, Self::Target>;
      type Error = PyErr;
      #[inline]
      fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(PyString::new(py, self.as_ref()))
      }
    }
    impl PyStubType for $t {
      #[inline]
      fn type_output() -> TypeInfo {
        use strum::IntoEnumIterator;
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

impl_py_enum!(TimeUnit);
impl_py_enum!(VoltageUnit);
impl_py_enum!(CurrentUnit);
impl_py_enum!(PullingResistanceUnit);
impl_py_enum!(LeakagePowerUnit);
impl_py_enum!(PgType);

impl<'py> FromPyObject<'py> for CapacitiveLoadUnit {
  #[inline]
  fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
    let (val, s_ff_pf) = ob.extract::<(f64, String)>()?;
    let ff_pf = match s_ff_pf.as_str() {
      "ff" => true,
      "pf" => false,
      _ => return Err(PyValueError::new_err("Matching variant not found")),
    };
    Ok(Self { ff_pf, val })
  }
}
impl<'py> IntoPyObject<'py> for CapacitiveLoadUnit {
  type Target = PyTuple;
  type Output = Bound<'py, Self::Target>;
  type Error = PyErr;
  #[inline]
  fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
    (self.val, if self.ff_pf { "ff" } else { "pf" }).into_pyobject(py)
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
