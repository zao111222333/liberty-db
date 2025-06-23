use crate::ast::join_fmt_no_quote;
use biodivine_lib_bdd::{Bdd, BddVariableSet};
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
pub struct SdfExpression(String);
impl fmt::Display for SdfExpression {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "\"{}\"", self.0)
  }
}
impl core::str::FromStr for SdfExpression {
  type Err = core::convert::Infallible;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self(String::from_str(s)?))
  }
}
crate::ast::impl_self_builder!(SdfExpression);
crate::ast::impl_simple!(SdfExpression);

impl SdfExpression {
  #[expect(clippy::indexing_slicing, clippy::shadow_reuse)]
  #[must_use]
  #[inline]
  pub fn new(bdd: &Bdd, variables: &BddVariableSet) -> Self {
    use core::fmt::Write as _;
    let mut s = "( ".to_owned();
    _ = join_fmt_no_quote(
      bdd.sat_clauses(),
      &mut s,
      |valuation, f| {
        join_fmt_no_quote(
          valuation.to_values().into_iter(),
          f,
          |(var, level), f| {
            let name = variables.name_of(var);
            if name.as_bytes()[0].is_ascii_digit() {
              write!(f, "\\\"{name}\\\"")?;
            } else {
              write!(f, "{name}")?;
            }
            write!(f, " == 1'b{}", i8::from(level))
          },
          |f| write!(f, " && "),
        )
      },
      |f| write!(f, ") || ( "),
    );
    s.push_str(" )");
    Self(s)
  }
}
#[cfg(test)]
mod test {
  use super::*;
  use crate::expression::BddBooleanExpression;
  use std::str::FromStr as _;
  #[test]
  fn sdf() {
    let variables = BddVariableSet::new(&["A", "B", "C", "D"]);
    assert_eq!(
      SdfExpression(
        "( A == 1'b0 && B == 1'b1 && C == 1'b1) || ( A == 1'b1 && C == 1'b1 )".into()
      ),
      SdfExpression::new(
        &BddBooleanExpression::from_str("(A+B)*C").unwrap().bdd,
        &variables
      )
    );
  }
}
