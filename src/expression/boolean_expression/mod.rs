//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
mod latch_ff;
pub use latch_ff::{FFBank, Latch, LatchBank, LatchFF, FF};
pub mod logic;
use std::{
  borrow::Borrow,
  collections::{HashSet, VecDeque},
  str::FromStr,
};

use biodivine_lib_bdd::{
  boolean_expression::BooleanExpression as Expr, Bdd, BddVariableSetBuilder,
};

mod parser;
use itertools::Itertools;
use parser::{BoolExprErr, Token};

lazy_static! {
  static ref UNKNOWN: Box<Expr> = Box::new(Expr::Variable("_unknown_".to_owned()));
}

pub trait BooleanExpressionLike: Borrow<Expr> + Into<Expr> + From<Expr> {
  #[inline]
  fn get_nodes(&self) -> HashSet<String> {
    let mut node_set = HashSet::new();
    _get_nodes(&self.borrow(), &mut node_set);
    return node_set;
  }
  /// `A & B` -> `A* & B*`
  #[inline]
  fn previous(&self) -> Expr {
    let mut expr: Expr = self.borrow().clone();
    _previous(&mut expr);
    return expr;
  }
}
/// `(A)? B : C` <-> `(A & B) | (!A & C)`
#[inline]
fn condition(cond: Expr, then_value: Expr, else_value: Expr) -> Expr {
  let box_cond = Box::new(cond);
  let box_then = Box::new(then_value);
  let box_else = Box::new(else_value);
  let expr = Expr::Or(
    Box::new(Expr::And(box_cond.clone(), box_then)),
    Box::new(Expr::And(Box::new(Expr::Not(box_cond)), box_else)),
  );
  expr
}
#[inline]
fn condition_box(cond: Box<Expr>, then_value: Box<Expr>, else_value: Box<Expr>) -> Expr {
  // let box_cond = Box::new(cond);
  // let box_then = Box::new(then_value);
  // let box_else = Box::new(else_value);
  let expr = Expr::Or(
    Box::new(Expr::And(cond.clone(), then_value)),
    Box::new(Expr::And(Box::new(Expr::Not(cond)), else_value)),
  );
  expr
}

impl BooleanExpressionLike for Expr {}
impl BooleanExpressionLike for BooleanExpression {}
impl BooleanExpressionLike for IdBooleanExpression {}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =132.36
/// &end
/// =132.38
/// ">Reference</a>
///
/// | Operator | Description                 |
/// | -------- | --------------------------- |
/// | '        | invert previous expression  |
/// | ’        | invert previous expression(?)|
/// | !        | invert following expression |
/// | ^        | logical XOR                 |
/// | \*       | logical AND                 |
/// | &        | logical AND                 |
/// | space    | logical AND                 |
/// | \+       | logical OR                  |
/// | 1        | logical OR                  |
/// | 1        | signal tied to logic 1      |
/// | 0        | signal tied to logic 0      |
///
/// A pin name beginning with a number must be enclosed in double quotation marks preceded by a backslash (\), as in the following example
/// ```
/// function : " \"1A\" + \"1B\" " ;
/// ```
///
#[derive(Debug, Clone)]
pub struct BooleanExpression {
  /// BooleanExpression itself
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
impl Into<Expr> for BooleanExpression {
  #[inline]
  fn into(self) -> Expr {
    self.expr
  }
}
impl crate::ast::SimpleAttri for BooleanExpression {}
impl crate::ast::SimpleAttri for IdBooleanExpression {}

#[derive(Debug, Clone)]
pub struct IdBooleanExpression {
  /// BooleanExpression itself
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
impl Into<Expr> for IdBooleanExpression {
  #[inline]
  fn into(self) -> Expr {
    self.expr
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
    self.bdd == other.bdd
  }
}
impl Eq for IdBooleanExpression {}
impl std::hash::Hash for IdBooleanExpression {
  #[inline]
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.bdd.hash(state);
  }
}

impl From<BooleanExpression> for IdBooleanExpression {
  // type Error = BoolExprErr;
  #[inline]
  fn from(value: BooleanExpression) -> Self {
    let mut builder = BddVariableSetBuilder::new();
    let node_set = &value.get_nodes();
    for s in node_set.into_iter().sorted() {
      let _ = builder.make_variable(s.as_str());
    }
    let variables = builder.build();
    let bdd = variables.eval_expression(&value.expr);
    Self { expr: value.expr, bdd }
    // match variables.eval_expression(&value.expr) {
    //   Some(bdd) => Ok(Self { expr: value.expr, bdd }),
    //   None => Err(BoolExprErr::NoIdea(1)),
    // }
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

impl FromStr for BooleanExpression {
  type Err = BoolExprErr;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut tokens: VecDeque<Token> = match parser::token_vec(s) {
      Ok((_, vec)) => vec.into_iter().collect(),
      Err(_) => return Err(BoolExprErr::Nom),
    };
    let expr = parser::process_tokens(&mut tokens)?;
    Ok(Self { expr })
  }
}

impl std::fmt::Display for BooleanExpression {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    parser::_fmt(&self.expr, f)
  }
}

impl std::fmt::Display for IdBooleanExpression {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    parser::_fmt(&self.expr, f)
  }
}

#[inline]
fn _get_nodes(expr: &Expr, node_set: &mut HashSet<String>) {
  match expr {
    Expr::Const(_) => (),
    Expr::Imp(_, _) => todo!(),
    Expr::Iff(_, _) => todo!(),
    Expr::Variable(node) => {
      let _ = node_set.insert(node.to_string());
    }
    Expr::Not(e) => _get_nodes(e, node_set),
    Expr::And(e1, e2) => {
      _get_nodes(e1, node_set);
      _get_nodes(e2, node_set);
    }
    Expr::Or(e1, e2) => {
      _get_nodes(e1, node_set);
      _get_nodes(e2, node_set);
    }
    Expr::Xor(e1, e2) => {
      _get_nodes(e1, node_set);
      _get_nodes(e2, node_set);
    }
  }
}

#[inline]
fn _previous(expr: &mut Expr) {
  match expr {
    Expr::Const(_) => (),
    Expr::Imp(_, _) => todo!(),
    Expr::Iff(_, _) => todo!(),
    Expr::Variable(node) => {
      *node += "*";
    }
    Expr::Not(e) => _previous(e),
    Expr::And(e1, e2) => {
      _previous(e1);
      _previous(e2);
    }
    Expr::Or(e1, e2) => {
      _previous(e1);
      _previous(e2);
    }
    Expr::Xor(e1, e2) => {
      _previous(e1);
      _previous(e2);
    }
  }
}

#[allow(dead_code, unused_imports)]
mod test {
  use super::*;
  use itertools::Itertools;
  use std::{f64::consts::E, str::FromStr};
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
      println!("origin:   {}", s);
      let bool_expr = IdBooleanExpression::from_str(s);
      if should_success {
        if let Ok(e) = bool_expr {
          let fmt_s = format!("{e}");
          println!("parsed:   {}", fmt_s);
          let fmt_bool_expr = IdBooleanExpression::from_str(&fmt_s);
          if let Ok(fmt_e) = fmt_bool_expr {
            println!("reparsed: {}", fmt_e);
            assert_eq!(e, fmt_e);
          } else {
            println!("{:?}", e);
            println!("{:?}", fmt_bool_expr);
            assert!(false, " not equal");
          }
        } else {
          println!("{:?}", bool_expr);
          assert!(false, " It should success");
        }
      } else {
        if let Err(e) = bool_expr {
          println!("{}", e);
        } else {
          assert!(false, " It should go wrong");
        }
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
}
