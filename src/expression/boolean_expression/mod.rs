#![allow(clippy::unnecessary_box_returns, clippy::used_underscore_items)]
//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
mod latch_ff;
pub mod logic;
// mod logic_impl;
mod parser;
use crate::{
  Ctx, DefaultCtx,
  ast::{CodeFormatter, Indentation, ParseScope, ParsingBuilder},
  cell::CellCtx as _,
};
pub use latch_ff::{ClearPresetState, FF, FFBank, Latch, LatchBank, LatchFF};
use parser::BoolExprErr;

pub use biodivine_lib_bdd::{
  Bdd, BddVariableSet, boolean_expression::BooleanExpression as Expr,
};
use core::{
  borrow::Borrow,
  cmp::Ordering,
  fmt::{self, Write},
  ops::{Deref, DerefMut},
  str::FromStr as _,
};
use std::{collections::HashSet, sync::LazyLock};

static UNKNOWN: LazyLock<Box<Expr>> =
  LazyLock::new(|| Box::new(Expr::Variable("_unknown_".to_owned())));

pub trait BooleanExpressionLike: Borrow<Expr> + Into<Expr> {
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
impl BooleanExpressionLike for BddBooleanExpression {}

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

crate::ast::impl_self_builder!(BooleanExpression);
impl<C: 'static + Ctx> crate::ast::SimpleAttri<C> for BooleanExpression {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope<'_>,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::parser::simple_custom(
      i,
      &mut scope.loc.line_num,
      Self::parse,
      Self::unquote,
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "\"{self}\"")
  }
}
impl<C: 'static + Ctx> crate::ast::SimpleAttri<C> for LogicBooleanExpression {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope<'_>,
  ) -> crate::ast::SimpleParseRes<'a, Self::Builder> {
    <Self::Builder as crate::ast::SimpleAttri<DefaultCtx>>::nom_parse(i, scope)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "\"{self}\"")
  }
}
impl<C: 'static + Ctx> crate::ast::SimpleAttri<C> for PowerGroundBooleanExpression {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope<'_>,
  ) -> crate::ast::SimpleParseRes<'a, Self::Builder> {
    <Self::Builder as crate::ast::SimpleAttri<DefaultCtx>>::nom_parse(i, scope)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "\"{self}\"")
  }
}

impl Deref for LogicBooleanExpression {
  type Target = BddBooleanExpression;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for LogicBooleanExpression {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Deref for PowerGroundBooleanExpression {
  type Target = BddBooleanExpression;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for PowerGroundBooleanExpression {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
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
pub struct BddBooleanExpression {
  /// `BooleanExpression` itself
  pub expr: Expr,
  /// Use [binary decision diagrams](https://en.wikipedia.org/wiki/Binary_decision_diagram) (BDDs)
  /// as `id`, to impl `hash` and `compare`
  pub bdd: Bdd,
}

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PowerGroundBooleanExpression(pub BddBooleanExpression);

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LogicBooleanExpression(pub BddBooleanExpression);

impl Borrow<Expr> for BddBooleanExpression {
  #[inline]
  fn borrow(&self) -> &Expr {
    &self.expr
  }
}
impl From<BddBooleanExpression> for Expr {
  #[inline]
  fn from(val: BddBooleanExpression) -> Self {
    val.expr
  }
}
impl PartialEq for BddBooleanExpression {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.bdd == other.bdd
  }
}

impl Eq for BddBooleanExpression {}
#[expect(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for BddBooleanExpression {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(Bdd::cmp_structural(&self.bdd, &other.bdd))
  }
}
impl Ord for BddBooleanExpression {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap_or(Ordering::Equal)
  }
}

impl core::hash::Hash for BddBooleanExpression {
  #[inline]
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.bdd.hash(state);
  }
}

impl LogicBooleanExpression {
  #[must_use]
  #[inline]
  pub fn new(expr: Expr, logic_variables: &BddVariableSet) -> Self {
    Self(BddBooleanExpression {
      bdd: logic_variables.safe_eval_expression(&expr).unwrap_or_else(|| {
        crate::error!("Failed to build BDD for [{expr}]");
        logic_variables.mk_false()
      }),
      expr,
    })
  }
  #[must_use]
  #[inline]
  pub fn bdd_new(bdd: Bdd, logic_variables: &BddVariableSet) -> Self {
    Self(BddBooleanExpression {
      expr: bdd.to_boolean_expression(logic_variables),
      bdd,
    })
  }
}

impl<C: 'static + Ctx> ParsingBuilder<C> for LogicBooleanExpression {
  type Builder = BooleanExpression;
  #[inline]
  fn build(builder: Self::Builder, scope: &mut crate::ast::BuilderScope<C>) -> Self {
    Self::new(builder.expr, &scope.cell_extra_ctx.logic_variables)
  }
}

impl PowerGroundBooleanExpression {
  #[must_use]
  #[inline]
  pub fn new(expr: Expr, pg_variables: &BddVariableSet) -> Self {
    Self(BddBooleanExpression {
      bdd: pg_variables.safe_eval_expression(&expr).unwrap_or_else(|| {
        crate::error!("Failed to build BDD for [{expr}]");
        pg_variables.mk_false()
      }),
      expr,
    })
  }
  #[must_use]
  #[inline]
  pub fn bdd_new(bdd: Bdd, pg_variables: &BddVariableSet) -> Self {
    Self(BddBooleanExpression { expr: bdd.to_boolean_expression(pg_variables), bdd })
  }
}
impl<C: 'static + Ctx> ParsingBuilder<C> for PowerGroundBooleanExpression {
  type Builder = BooleanExpression;
  #[inline]
  fn build(builder: Self::Builder, scope: &mut crate::ast::BuilderScope<C>) -> Self {
    Self::new(builder.expr, &scope.cell_extra_ctx.pg_variables)
  }
}

pub struct BooleanExpressionDisplay<'a>(pub &'a Expr);
impl fmt::Display for BooleanExpressionDisplay<'_> {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    parser::_fmt(self.0, f)
  }
}
impl fmt::Debug for BooleanExpressionDisplay<'_> {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("BoolExpr").field(&format!("{self}")).finish()
  }
}

impl fmt::Display for BooleanExpression {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", BooleanExpressionDisplay(&self.expr))
  }
}

impl fmt::Display for LogicBooleanExpression {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", BooleanExpressionDisplay(&self.expr))
  }
}

impl fmt::Display for PowerGroundBooleanExpression {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", BooleanExpressionDisplay(&self.expr))
  }
}

impl<C: 'static + Ctx> crate::Cell<C> {
  #[inline]
  pub fn build_logic_boolexpr(
    &self,
    expr: Expr,
  ) -> Result<LogicBooleanExpression, BoolExprErr> {
    self
      .extra_ctx
      .logic_variables()
      .safe_eval_expression(&expr)
      .map_or(Err(BoolExprErr::FailToBuildBdd), |bdd| {
        Ok(LogicBooleanExpression(BddBooleanExpression { expr, bdd }))
      })
  }
  #[inline]
  pub fn parse_logic_boolexpr(
    &self,
    s: &str,
  ) -> Result<LogicBooleanExpression, BoolExprErr> {
    let expr = BooleanExpression::from_str(s)?.expr;
    self.build_logic_boolexpr(expr)
  }
  #[inline]
  pub fn build_pg_boolexpr(
    &self,
    expr: Expr,
  ) -> Result<PowerGroundBooleanExpression, BoolExprErr> {
    self
      .extra_ctx
      .pg_variables()
      .safe_eval_expression(&expr)
      .map_or(Err(BoolExprErr::FailToBuildBdd), |bdd| {
        Ok(PowerGroundBooleanExpression(BddBooleanExpression { expr, bdd }))
      })
  }
  #[inline]
  pub fn parse_pg_boolexpr(
    &self,
    s: &str,
  ) -> Result<PowerGroundBooleanExpression, BoolExprErr> {
    let expr = BooleanExpression::from_str(s)?.expr;
    self.build_pg_boolexpr(expr)
  }
}

#[inline]
fn _get_nodes<'a>(
  expr: &'a Expr,
  node_set: &mut HashSet<&'a str, crate::ast::RandomState>,
) {
  match expr {
    Expr::Const(_) => (),
    Expr::Variable(node) => {
      _ = node_set.insert(node);
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
      *node += "~";
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
  use crate::DefaultCtx;
  use core::{f64::consts::E, str::FromStr as _};
  use itertools::Itertools as _;
  impl From<BooleanExpression> for BddBooleanExpression {
    #[inline]
    fn from(value: BooleanExpression) -> Self {
      let node_set = value.expr.support_set();
      let mut node_set: Vec<&str> = node_set.iter().map(String::as_str).collect();
      node_set.sort_unstable();
      let variables = BddVariableSet::new(&node_set);
      Self {
        bdd: variables.eval_expression(&value.expr),
        expr: value.expr,
      }
    }
  }
  impl std::str::FromStr for BddBooleanExpression {
    type Err = BoolExprErr;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
      let expr = BooleanExpression::from_str(s)?;
      Ok(expr.into())
    }
  }

  #[test]
  fn parse_fmt_self_check() {
    for (should_success, s) in [
      (true, "A"),
      (true, "A^B+C"),
      (true, "(A+B)*(C+D)"),
      (true, r#"\"1A\" + \"1B\""#),
      (true, r#"A_1 + \"1B_2\""#),
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
      let bool_expr = BddBooleanExpression::from_str(s);
      if should_success {
        if let Ok(e) = bool_expr {
          let fmt_s = format!("{}", BooleanExpression { expr: e.clone().expr });
          println!("parsed:   {fmt_s}");
          let fmt_bool_expr = BddBooleanExpression::from_str(&fmt_s);
          if let Ok(fmt_e) = fmt_bool_expr {
            println!("reparsed: {}", BooleanExpression { expr: fmt_e.clone().expr });
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
      // skip this case, should guarantee same variables
      // (false, "A+B+C", "A+B+D"),
      (true, "1A", "1"),
      (true, "1A+B", "1+B"),
    ] {
      println!("----");
      println!("s1: {s1}");
      println!("s2: {s2}");
      if same {
        println!("they should same");
        assert_eq!(
          BddBooleanExpression::from_str(s1),
          BddBooleanExpression::from_str(s2)
        );
      } else {
        println!("they are different");
        assert_ne!(
          BddBooleanExpression::from_str(s1),
          BddBooleanExpression::from_str(s2)
        );
      }
    }
  }
  /// `cond ? then_exp : else_exp` is equal to `(cond & then_exp) | (!cond & else_exp)`
  #[test]
  fn if_else() {
    let a = Box::new(Expr::Variable("A".to_owned()));
    let b = Box::new(Expr::Variable("B".to_owned()));
    let c = Box::new(Expr::Variable("C".to_owned()));
    let cond: BddBooleanExpression =
      BooleanExpression { expr: Expr::Cond(a.clone(), b.clone(), c.clone()) }.into();
    let or_and: BddBooleanExpression = BooleanExpression {
      expr: Expr::Or(
        Box::new(Expr::And(a.clone(), b)),
        Box::new(Expr::And(Box::new(Expr::Not(a)), c)),
      ),
    }
    .into();
    assert_eq!(cond, or_and);
  }
  #[test]
  fn lid_bdd() {
    let variables = BddVariableSet::new(&["A", "B", "C", "D"]);
    let x1 = variables.eval_expression_string("(A|B)&(C|D)");
    let x2 = variables.eval_expression_string("B&D | B&C | A&D | A&C");
    assert_eq!(x1, x2);
    println!("{variables}");
    for valuation in x1.sat_valuations() {
      println!("{valuation}");
      assert!(x1.eval_in(&valuation));
    }
  }
  #[test]
  fn lid_bdd2() {
    let variables = BddVariableSet::new(&["A", "B", "C", "D"]);
    let x1 = variables.eval_expression_string("(A|B)&(C|D)");
    let variables = BddVariableSet::new(&["A", "B", "C", "D", "E"]);
    let x2 = variables.eval_expression_string("(A|B)&(C|D)");
    assert_ne!(x1, x2);
  }
}
