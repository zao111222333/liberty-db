//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
pub mod logic;
use std::{
  collections::{HashSet, VecDeque},
  str::FromStr,
};

use biodivine_lib_bdd::{
  boolean_expression::BooleanExpression as Expr, Bdd, BddVariableSet,
  BddVariableSetBuilder,
};

mod parser;
use itertools::Itertools;
use parser::{BoolExprErr, Token};

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

#[derive(Debug, Clone)]
pub struct BooleanExpressionId {
  /// BooleanExpression itself
  pub expr: Expr,
  /// Use [binary decision diagrams](https://en.wikipedia.org/wiki/Binary_decision_diagram) (BDDs)
  /// as `id`, to impl `hash` and `compare`
  pub bdd: Bdd,
}

impl PartialEq for BooleanExpressionId {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.bdd == other.bdd
  }
}
impl Eq for BooleanExpressionId {}
impl std::hash::Hash for BooleanExpressionId {
  #[inline]
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.bdd.hash(state);
  }
}

impl TryFrom<BooleanExpression> for BooleanExpressionId {
  type Error = BoolExprErr;
  #[inline]
  fn try_from(value: BooleanExpression) -> Result<Self, Self::Error> {
    let mut builder = BddVariableSetBuilder::new();
    let mut node_set = HashSet::new();
    parser::get_nodes(&value.expr, &mut node_set);
    //  println!("{:?}", node_set);
    for s in node_set.into_iter().sorted() {
      let _ = builder.make_variable(s.as_str());
    }
    let variables = builder.build();
    match variables.safe_eval_expression(&value.expr) {
      Some(bdd) => Ok(Self { expr: value.expr, bdd }),
      None => Err(BoolExprErr::NoIdea(1)),
    }
  }
}

impl FromStr for BooleanExpressionId {
  type Err = BoolExprErr;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let expr = BooleanExpression::from_str(s)?;
    expr.try_into()
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
    //  println!("{:?}", tokens);
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

impl std::fmt::Display for BooleanExpressionId {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    parser::_fmt(&self.expr, f)
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
      let bool_expr = BooleanExpressionId::from_str(s);
      if should_success {
        if let Ok(e) = bool_expr {
          let fmt_s = format!("{e}");
          println!("parsed:   {}", fmt_s);
          let fmt_bool_expr = BooleanExpressionId::from_str(&fmt_s);
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
        assert_eq!(BooleanExpressionId::from_str(s1), BooleanExpressionId::from_str(s2));
      } else {
        println!("they are different");
        assert_ne!(BooleanExpressionId::from_str(s1), BooleanExpressionId::from_str(s2));
      }
    }
  }
}
