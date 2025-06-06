use core::fmt;

/// The `sdf_cond` attribute is defined in the state-dependent timing group to support SDF file
/// generation and condition matching during back-annotation.
///
/// ### Syntax
/// ``` text
/// sdf_cond : "SDF expression" ;
/// ```
/// SDF expression
///
/// A string that represents a Boolean description of the state dependency of the
/// delay. Use a Boolean description that conforms to the valid syntax defined in
/// the OVI SDF, which is different from the Boolean expression. For a complete
/// description of the valid syntax for these expressions, see the OVI specification
/// for SDF, V1.0.
///
/// ### Example
/// ``` text
/// sdf_cond : "b == 1’b1" ;
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=327.3&end=327.14
/// ">Reference</a>
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SdfExpression {
  inner: String,
}
impl fmt::Display for SdfExpression {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "\"{}\"", self.inner)
  }
}
impl core::str::FromStr for SdfExpression {
  type Err = core::convert::Infallible;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self { inner: String::from_str(s)? })
  }
}
crate::ast::impl_self_builder!(SdfExpression);
crate::ast::impl_simple!(SdfExpression);

impl SdfExpression {
  #[must_use]
  #[inline]
  pub const fn new(s: String) -> Self {
    Self { inner: s }
  }
}
