use crate::{
  ast::{CodeFormatter, Indentation, ParseScope, SimpleAttri},
  ArcStr,
};
use core::fmt::{self, Write};

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
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SdfExpression {
  inner: ArcStr,
}
impl fmt::Display for SdfExpression {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(&self.inner, f)
  }
}
impl core::str::FromStr for SdfExpression {
  type Err = core::convert::Infallible;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self { inner: ArcStr::from_str(s)? })
  }
}
impl SimpleAttri for SdfExpression {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_fmt(format_args!("\"{self}\""))
  }
}
impl SdfExpression {
  #[must_use]
  #[inline]
  pub fn new(s: ArcStr) -> Self {
    Self { inner: s }
  }
}
