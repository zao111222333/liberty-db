//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
pub mod logic;
pub mod parser;
use biodivine_lib_bdd::{boolean_expression::BooleanExpression as Expr, Bdd};

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
  /// Use [binary decision diagrams](https://en.wikipedia.org/wiki/Binary_decision_diagram) (BDDs)
  /// as `id`, to impl `hash` and `compare`
  pub bdd: Bdd,
  /// BooleanExpression itself
  pub expr: Expr,
}

impl PartialEq for BooleanExpression {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.bdd == other.bdd
  }
}
impl Eq for BooleanExpression {}
impl std::hash::Hash for BooleanExpression {
  #[inline]
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.bdd.hash(state);
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
      let bool_expr = BooleanExpression::from_str(s);
      if should_success {
        if let Ok(e) = bool_expr {
          let fmt_s = format!("{e}");
          println!("parsed:   {}", fmt_s);
          let fmt_bool_expr = BooleanExpression::from_str(&fmt_s);
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
        assert_eq!(BooleanExpression::from_str(s1), BooleanExpression::from_str(s2));
      } else {
        println!("they are different");
        assert_ne!(BooleanExpression::from_str(s1), BooleanExpression::from_str(s2));
      }
    }
  }
}
