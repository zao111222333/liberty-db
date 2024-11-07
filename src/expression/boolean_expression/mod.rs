#![allow(clippy::unnecessary_box_returns)]
//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
mod latch_ff;
pub mod logic;
mod parser;
use crate::{
  ast::{CodeFormatter, Indentation, ParseScope},
  ArcStr,
};
pub use latch_ff::{FFBank, Latch, LatchBank, LatchFF, FF};
use parser::{as_sdf_str, BoolExprErr};

pub use biodivine_lib_bdd::{
  boolean_expression::BooleanExpression as Expr, Bdd, BddVariableSet,
  BddVariableSetBuilder,
};
use core::{
  borrow::Borrow,
  cmp::Ordering,
  fmt::{self, Write},
  str::FromStr,
};
use itertools::Itertools;
use std::collections::HashSet;

use super::SdfExpression;
lazy_static::lazy_static! {
  static ref UNKNOWN: Box<Expr> = Box::new(Expr::Variable("_unknown_".to_owned()));
}

pub trait BooleanExpressionLike: Borrow<Expr> + Into<Expr> + From<Expr> {
  #[inline]
  fn get_nodes(&self) -> HashSet<ArcStr> {
    let mut node_set = HashSet::new();
    _get_nodes(self.borrow(), &mut node_set);
    node_set
  }
  /// `A & B` -> `A* & B*`
  #[inline]
  fn previous(&self) -> Expr {
    let mut expr: Expr = self.borrow().clone();
    _previous(&mut expr);
    expr
  }
}

impl BooleanExpressionLike for Expr {}
impl BooleanExpressionLike for BooleanExpression {}
impl BooleanExpressionLike for IdBooleanExpression {}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=132.36+132.41&end=132.38+133.13
/// ">Reference</a>
///
/// | Operator | Description                           |
/// | -------- | ------------------------------------- |
/// | '        | invert previous expression            |
/// | ’        | invert previous expression(?)         |
/// | !        | invert following expression           |
/// | ^        | logical XOR                           |
/// | \*       | logical AND                           |
/// | &        | logical AND                           |
/// | space    | logical AND (when no other separator) |
/// | \+       | logical OR                            |
/// | \|       | logical OR                            |
/// | 1        | signal tied to logic 1                |
/// | 0        | signal tied to logic 0                |
///
/// A pin name beginning with a number must be enclosed in double quotation marks preceded by a backslash (\), as in the following example
/// ``` liberty
/// function : " \"1A\" + \"1B\" " ;
/// ```
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BooleanExpression {
  /// `BooleanExpression` itself
  pub expr: Expr,
}

impl Borrow<Expr> for BooleanExpression {
  #[inline]
  fn borrow(&self) -> &Expr {
    &self.expr
  }
}
impl From<Expr> for BooleanExpression {
  #[inline]
  fn from(expr: Expr) -> Self {
    Self { expr }
  }
}
impl From<BooleanExpression> for Expr {
  #[inline]
  fn from(val: BooleanExpression) -> Self {
    val.expr
  }
}
impl crate::ast::SimpleAttri for BooleanExpression {
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
impl crate::ast::SimpleAttri for IdBooleanExpression {
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

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=132.36+132.41&end=132.38+133.13
/// ">Reference</a>
///
/// | Operator | Description                           |
/// | -------- | ------------------------------------- |
/// | '        | invert previous expression            |
/// | ’        | invert previous expression(?)         |
/// | !        | invert following expression           |
/// | ^        | logical XOR                           |
/// | \*       | logical AND                           |
/// | &        | logical AND                           |
/// | space    | logical AND (when no other separator) |
/// | \+       | logical OR                            |
/// | \|       | logical OR                            |
/// | 1        | signal tied to logic 1                |
/// | 0        | signal tied to logic 0                |
///
/// A pin name beginning with a number must be enclosed in double quotation marks preceded by a backslash (\), as in the following example
/// ``` liberty
/// function : " \"1A\" + \"1B\" " ;
/// ```
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct IdBooleanExpression {
  /// `sorted_nodes`
  pub sorted_nodes: Vec<ArcStr>,
  /// `BooleanExpression` itself
  pub expr: Expr,
  /// Use [binary decision diagrams](https://en.wikipedia.org/wiki/Binary_decision_diagram) (BDDs)
  /// as `id`, to impl `hash` and `compare`
  pub bdd: Bdd,
}
impl Borrow<Expr> for IdBooleanExpression {
  #[inline]
  fn borrow(&self) -> &Expr {
    &self.expr
  }
}
impl From<IdBooleanExpression> for Expr {
  #[inline]
  fn from(val: IdBooleanExpression) -> Self {
    val.expr
  }
}
impl From<Expr> for IdBooleanExpression {
  #[inline]
  fn from(expr: Expr) -> Self {
    BooleanExpression::from(expr).into()
  }
}
impl PartialEq for IdBooleanExpression {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.sorted_nodes == other.sorted_nodes && self.bdd == other.bdd
  }
}

impl IdBooleanExpression {
  /// convert `BooleanExpression` to sdf
  #[must_use]
  #[inline]
  pub fn sdf(&self) -> SdfExpression {
    let variables = BddVariableSet::new(
      self
        .sorted_nodes
        .iter()
        .map(ArcStr::as_str)
        .collect::<Vec<_>>()
        .as_slice(),
    );
    let s = self
      .bdd
      .sat_valuations()
      .map(|valuation| {
        let expr = Bdd::from(valuation).to_boolean_expression(&variables);
        as_sdf_str(&expr)
      })
      .join(") || ( ");
    SdfExpression::new(format!("( {s} )").into())
  }
}

impl Eq for IdBooleanExpression {}
#[expect(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for IdBooleanExpression {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self.sorted_nodes.partial_cmp(&other.sorted_nodes) {
      Some(Ordering::Equal) | None => Some(Bdd::cmp_structural(&self.bdd, &other.bdd)),
      ord => ord,
    }
  }
}
impl Ord for IdBooleanExpression {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap_or(Ordering::Equal)
  }
}

impl core::hash::Hash for IdBooleanExpression {
  #[inline]
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.sorted_nodes.hash(state);
    self.bdd.hash(state);
  }
}

impl From<BooleanExpression> for IdBooleanExpression {
  #[inline]
  fn from(value: BooleanExpression) -> Self {
    let mut builder = BddVariableSetBuilder::new();
    let node_set = value.get_nodes();
    let sorted_nodes = node_set
      .into_iter()
      .sorted()
      .inspect(|s| {
        _ = builder.make_variable(s.as_str());
      })
      .collect();
    let variables = builder.build();
    let bdd = variables.eval_expression(&value.expr);
    Self { sorted_nodes, expr: value.expr, bdd }
  }
}

impl FromStr for IdBooleanExpression {
  type Err = BoolExprErr;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let expr = BooleanExpression::from_str(s)?;
    Ok(expr.into())
  }
}

impl fmt::Display for BooleanExpression {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    parser::_fmt(&self.expr, f)
  }
}

impl fmt::Display for IdBooleanExpression {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    parser::_fmt(&self.expr, f)
  }
}

#[inline]
fn _get_nodes(expr: &Expr, node_set: &mut HashSet<ArcStr>) {
  match expr {
    Expr::Const(_) => (),
    Expr::Variable(node) => {
      _ = node_set.insert(ArcStr::from(node));
    }
    Expr::Not(e) => _get_nodes(e, node_set),
    Expr::And(e1, e2) | Expr::Or(e1, e2) | Expr::Xor(e1, e2) => {
      _get_nodes(e1, node_set);
      _get_nodes(e2, node_set);
    }
    Expr::Cond(e1, e2, e3) => {
      _get_nodes(e1, node_set);
      _get_nodes(e2, node_set);
      _get_nodes(e3, node_set);
    }
    Expr::Imp(_, _) | Expr::Iff(_, _) => unreachable!(),
  }
}

#[inline]
fn _previous(expr: &mut Expr) {
  match expr {
    Expr::Const(_) => (),
    Expr::Variable(node) => {
      *node += "*";
    }
    Expr::Not(e) => _previous(e),
    Expr::And(e1, e2) | Expr::Or(e1, e2) | Expr::Xor(e1, e2) => {
      _previous(e1);
      _previous(e2);
    }
    Expr::Imp(_, _) | Expr::Iff(_, _) | Expr::Cond(_, _, _) => unreachable!(),
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use core::{f64::consts::E, str::FromStr};
  use itertools::Itertools;
  #[test]
  fn parse_fmt_self_check() {
    for (should_success, s) in [
      (true, "A"),
      (true, "A^B+C"),
      (true, "(A+B)*(C+D)"),
      (true, r#"\"1A\" + \"1B\""#),
      (true, "(A+B)*(C)"),
      (true, "!(A+((C+A^!!!B))')"),
      (true, "!(A&B)"),
      (true, "!(1&B)"),
      (true, "A+B+C+D"),
      (true, "B0’ + C"),
      (true, "A+B+C+D"),
      (true, "A+(B+C)^D"),
      (true, "!(1A&B)"),
      (true, "!(A B)"),
      (true, "!(A+B')"),
      (true, "!(A+B')|C"),
      (true, "(A)'''"),
      (true, "!!!(((A)))''"),
      (true, "!!(!((A))')'"),
      (false, ""),
      (false, "!"),
      (false, "A)"),
      (true, "1A"),
      (false, "2A"),
      (false, "(A"),
    ] {
      println!("----");
      println!("origin:   {s}");
      let bool_expr = IdBooleanExpression::from_str(s);
      if should_success {
        if let Ok(e) = bool_expr {
          let fmt_s = format!("{e}");
          println!("parsed:   {fmt_s}");
          let fmt_bool_expr = IdBooleanExpression::from_str(&fmt_s);
          if let Ok(fmt_e) = fmt_bool_expr {
            println!("reparsed: {fmt_e}");
            assert_eq!(e, fmt_e);
          } else {
            println!("{e:?}");
            println!("{fmt_bool_expr:?}");
            panic!("not equal");
          }
        } else {
          println!("{bool_expr:?}");
          panic!("It should success");
        }
      } else if let Err(e) = bool_expr {
        println!("{e}");
      } else {
        panic!("It should go wrong");
      }
    }
  }
  #[test]
  fn parse_hash() {
    for (same, s1, s2) in [
      (true, "!!(!((A))')'", "!A"),
      (true, "A*C+B*C", "(A+B)*C"),
      (true, "B*D+B*C+A*D+A*C", "(A+B)*(C+D)"),
      (false, "A+B^C", "A^B+C"),
      (true, "(A+B)+C", "A+(B+C)"),
      (false, "A+B+C", "A+B+D"),
      (true, "1A", "1"),
      (true, "1A+B", "1+B"),
    ] {
      println!("----");
      println!("s1: {s1}");
      println!("s2: {s2}");
      if same {
        println!("they should same");
        assert_eq!(IdBooleanExpression::from_str(s1), IdBooleanExpression::from_str(s2));
      } else {
        println!("they are different");
        assert_ne!(IdBooleanExpression::from_str(s1), IdBooleanExpression::from_str(s2));
      }
    }
  }
  /// `cond ? then_exp : else_exp` is equal to `(cond & then_exp) | (!cond & else_exp)`
  #[test]
  fn if_else() {
    let a = Box::new(Expr::Variable("A".to_owned()));
    let b = Box::new(Expr::Variable("B".to_owned()));
    let c = Box::new(Expr::Variable("C".to_owned()));
    let cond: IdBooleanExpression =
      BooleanExpression { expr: Expr::Cond(a.clone(), b.clone(), c.clone()) }.into();
    let or_and: IdBooleanExpression = BooleanExpression {
      expr: Expr::Or(
        Box::new(Expr::And(a.clone(), b)),
        Box::new(Expr::And(Box::new(Expr::Not(a)), c)),
      ),
    }
    .into();
    assert_eq!(cond, or_and);
  }
  #[test]
  fn sdf() {
    assert_eq!(
      SdfExpression::new("( A == 1'b0 && B == 1'b1 && C == 1'b1) || ( A == 1'b1 && B == 1'b0 && C == 1'b1) || ( A == 1'b1 && B == 1'b1 && C == 1'b1 )".into()), 
      IdBooleanExpression::from_str("(A+B)*C").unwrap().sdf(),
    );
  }
  #[test]
  fn lid_bdd() {
    let mut builder = BddVariableSetBuilder::new();
    let [a, b, c, d] = builder.make(&["A", "B", "C", "D"]);
    let variables = builder.build();
    let x1 = variables.eval_expression_string("(A|B)&(C|D)");
    let x2 = variables.eval_expression_string("B&D | B&C | A&D | A&C");
    assert_eq!(x1, x2);
    println!("{variables}");
    for valuation in x1.sat_valuations() {
      println!("{valuation}");
      assert!(x1.eval_in(&valuation));
    }
  }
}
