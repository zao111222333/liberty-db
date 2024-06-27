use crate::ArcStr;

use crate::ast::SimpleAttri;

/// The expression must conform to `OVI SDF 2.1 timing-check condition syntax`.
///
/// #### Example
/// ``` liberty
/// sdf_cond_end : "SIG_0 == 1’b1" ;
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =210.10
/// &end
/// =210.19
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =203.45
/// &end
/// =203.45
/// ">Reference-Instance</a>
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SdfExpression {
  inner: ArcStr,
}
impl std::fmt::Display for SdfExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(&self.inner, f)
  }
}
impl std::str::FromStr for SdfExpression {
  type Err = core::convert::Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self { inner: ArcStr::from_str(s)? })
  }
}
impl SimpleAttri for SdfExpression {}
impl SdfExpression {
  pub fn new(s: ArcStr) -> Self {
    Self { inner: s }
  }
}
