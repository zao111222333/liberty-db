use core::fmt::{self, Write};
use nom::{
  IResult, Parser as _,
  branch::alt,
  character::complete::char,
  combinator::{map, map_res, opt},
  multi::many1,
  sequence::{delimited, preceded},
};

use crate::{
  Ctx,
  ast::{
    CodeFormatter, Indentation, ParseScope,
    parser::{float_one, key, space, unquote},
  },
};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum FormulaExpr {
  Add(Box<FormulaExpr>, Box<FormulaExpr>),
  Sub(Box<FormulaExpr>, Box<FormulaExpr>),
  Mul(Box<FormulaExpr>, Box<FormulaExpr>),
  Div(Box<FormulaExpr>, Box<FormulaExpr>),
  Neg(Box<FormulaExpr>),
  Num(f64),
  Var(String),
}

#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Formula {
  pub expr: FormulaExpr,
  pub value: Option<f64>,
}

impl Default for FormulaExpr {
  #[inline]
  fn default() -> Self {
    Self::Num(0.0)
  }
}

impl<C: Ctx> crate::ast::ParsingBuilder<C> for Formula {
  type Builder = FormulaExpr;
  #[inline]
  #[expect(clippy::renamed_function_params)]
  fn build(expr: Self::Builder, scope: &mut crate::ast::BuilderScope<C>) -> Self {
    let value = expr.eval(&expr, |k: &str| scope.voltage_map.get(k).copied());
    Self { expr, value }
  }
}
impl<C: Ctx> crate::ast::SimpleAttri<C> for Formula {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope<'_>,
  ) -> crate::ast::SimpleParseRes<'a, FormulaExpr> {
    crate::ast::parser::simple_custom(
      i,
      &mut scope.loc.line_num,
      FormulaExpr::parse,
      unquote,
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_fmt(format_args!("{}", self.expr))
  }
}

impl FormulaExpr {
  #[inline]
  pub fn parse(i: &str) -> IResult<&str, Self> {
    map_res(tokens, |tokens| parse_formula(&tokens)).parse_complete(i)
  }
  #[inline]
  #[expect(clippy::float_arithmetic, clippy::option_if_let_else)]
  pub fn eval<F: Fn(&str) -> Option<f64> + Copy>(
    &self,
    top: &Self,
    query_fn: F, // map: &HashMap<String, f64, S>,
  ) -> Option<f64> {
    match self {
      Self::Add(e1, e2) => {
        let f1 = e1.eval(top, query_fn)?;
        e2.eval(top, query_fn).map(|f2| f1 + f2)
      }
      Self::Sub(e1, e2) => {
        let f1 = e1.eval(top, query_fn)?;
        e2.eval(top, query_fn).map(|f2| f1 - f2)
      }
      Self::Mul(e1, e2) => {
        let f1 = e1.eval(top, query_fn)?;
        e2.eval(top, query_fn).map(|f2| f1 * f2)
      }
      Self::Div(e1, e2) => {
        let f1 = e1.eval(top, query_fn)?;
        e2.eval(top, query_fn).map(|f2| f1 / f2)
      }
      Self::Neg(e) => e.eval(top, query_fn).map(|f| -f),
      Self::Num(f) => Some(*f),
      Self::Var(k) => {
        if let Some(f) = query_fn(k) {
          Some(f)
        } else {
          crate::error!("Eval formula [{top}]: Can NOT find voltage {k}");
          None
        }
      }
    }
  }
  /// precedence
  const fn prec(&self) -> u8 {
    match self {
      Self::Add(..) | Self::Sub(..) => 1,
      Self::Mul(..) | Self::Div(..) => 2,
      Self::Neg(_) => 3,
      Self::Num(_) | Self::Var(_) => 4,
    }
  }
  /// recursively format,
  /// `parent_op`: parent node's op type,
  /// `is_right`: is right sub-tree
  fn fmt_with(
    &self,
    f: &mut fmt::Formatter<'_>,
    parent_op: ParentOp,
    is_right: bool,
  ) -> fmt::Result {
    let my_prec = self.prec();
    let par_prec = parent_op.prec();

    // need paren:
    // 1) my precedence < parent's precedence
    // 2) parent is Sub/Div && self is right sub-tree && my precedence == parent's precedence
    let need_paren = my_prec < par_prec
      || (is_right
        && matches!(parent_op, ParentOp::Sub | ParentOp::Div)
        && my_prec == par_prec);

    if need_paren {
      write!(f, "(")?;
    }

    match self {
      Self::Num(n) => write!(f, "{n}")?,
      Self::Var(s) => write!(f, "{s}")?,
      Self::Neg(expr) => {
        write!(f, "-")?;
        expr.fmt_with(f, ParentOp::Neg, false)?;
      }
      Self::Add(lhs, rhs) => {
        lhs.fmt_with(f, ParentOp::Add, false)?;
        write!(f, " + ")?;
        rhs.fmt_with(f, ParentOp::Add, true)?;
      }
      Self::Sub(lhs, rhs) => {
        lhs.fmt_with(f, ParentOp::Sub, false)?;
        write!(f, " - ")?;
        rhs.fmt_with(f, ParentOp::Sub, true)?;
      }
      Self::Mul(lhs, rhs) => {
        lhs.fmt_with(f, ParentOp::Mul, false)?;
        write!(f, " * ")?;
        rhs.fmt_with(f, ParentOp::Mul, true)?;
      }
      Self::Div(lhs, rhs) => {
        lhs.fmt_with(f, ParentOp::Div, false)?;
        write!(f, " / ")?;
        rhs.fmt_with(f, ParentOp::Div, true)?;
      }
    }

    if need_paren {
      write!(f, ")")?;
    }
    Ok(())
  }
}

impl fmt::Display for Formula {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(&self.expr, f)
  }
}
impl fmt::Display for FormulaExpr {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // no parent op for top level, either is not sub-tree
    self.fmt_with(f, ParentOp::None, false)
  }
}

/// To denote the relation between current node and parent node,
/// and what's op type of parent node
#[derive(Copy, Clone)]
enum ParentOp {
  None,
  Add,
  Sub,
  Mul,
  Div,
  Neg,
}

impl ParentOp {
  /// precedence
  const fn prec(self) -> u8 {
    match self {
      Self::None => 0,
      Self::Add | Self::Sub => 1,
      Self::Mul | Self::Div => 2,
      Self::Neg => 3,
    }
  }
}

/// `ExprErr`
#[derive(Clone, Copy, Debug, thiserror::Error, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ExprErr {
  /// `SingleOp`
  #[error("rset tokens")]
  LeftToken,
  /// `Bracket`
  #[error("left-right Bracket mismatch")]
  Bracket,
  /// `NoIdea`
  #[error("something go wrong, code=1")]
  NoIdea1,
}

#[inline]
fn parse_formula(data: &[Token<'_>]) -> Result<FormulaExpr, ExprErr> {
  struct Parser<'a> {
    tokens: &'a [Token<'a>],
    pos: usize,
  }
  #[expect(clippy::arithmetic_side_effects, clippy::wildcard_enum_match_arm)]
  impl<'a> Parser<'a> {
    // entry: after parse, need no token left
    fn parse(&mut self) -> Result<FormulaExpr, ExprErr> {
      let expr = self.parse_add_sub()?;
      if self.pos != self.tokens.len() {
        return Err(ExprErr::LeftToken);
      }
      Ok(expr)
    }
    fn current(&self) -> Option<&Token<'a>> {
      self.tokens.get(self.pos)
    }

    // add/sub
    fn parse_add_sub(&mut self) -> Result<FormulaExpr, ExprErr> {
      let mut node = self.parse_mul_div()?;
      while let Some(tok) = self.current() {
        match tok {
          Token::Add => {
            self.pos += 1;
            let rhs = self.parse_mul_div()?;
            node = FormulaExpr::Add(Box::new(node), Box::new(rhs));
          }
          Token::Sub => {
            self.pos += 1;
            let rhs = self.parse_mul_div()?;
            node = FormulaExpr::Sub(Box::new(node), Box::new(rhs));
          }
          _ => break,
        }
      }
      Ok(node)
    }

    // mul/div
    fn parse_mul_div(&mut self) -> Result<FormulaExpr, ExprErr> {
      let mut node = self.parse_unary()?;
      while let Some(tok) = self.current() {
        match tok {
          Token::Mul => {
            self.pos += 1;
            let rhs = self.parse_unary()?;
            node = FormulaExpr::Mul(Box::new(node), Box::new(rhs));
          }
          Token::Div => {
            self.pos += 1;
            let rhs = self.parse_unary()?;
            node = FormulaExpr::Div(Box::new(node), Box::new(rhs));
          }
          _ => break,
        }
      }
      Ok(node)
    }

    // unary op, negative only for now
    fn parse_unary(&mut self) -> Result<FormulaExpr, ExprErr> {
      match self.current() {
        Some(Token::Sub) => {
          self.pos += 1;
          let expr = self.parse_unary()?;
          Ok(FormulaExpr::Neg(Box::new(expr)))
        }
        Some(Token::Add) => {
          self.pos += 1;
          let expr = self.parse_unary()?;
          Ok(expr)
        }
        _ => self.parse_primary(),
      }
    }

    // primary op: num/var/paren
    fn parse_primary(&mut self) -> Result<FormulaExpr, ExprErr> {
      let res = match self.current() {
        Some(Token::Num(n)) => Ok(FormulaExpr::Num(*n)),
        Some(Token::Var(name)) => Ok(FormulaExpr::Var((*name).to_owned())),
        // Recursively call `parse_formula` when meet paren
        Some(Token::Paren(inner)) => Ok(parse_formula(inner)?),
        _ => return Err(ExprErr::NoIdea1),
      };
      // move pos
      self.pos += 1;
      res
    }
  }
  let mut parser = Parser { tokens: data, pos: 0 };
  parser.parse()
}

#[derive(Debug)]
enum Token<'s> {
  Add,
  Sub,
  Mul,
  Div,
  Num(f64),
  Var(&'s str),
  Paren(Vec<Token<'s>>),
}

fn tokens(i: &str) -> IResult<&str, Vec<Token<'_>>> {
  many1(preceded(
    opt(space),
    alt((
      map(char('+'), |_| Token::Add),
      map(char('-'), |_| Token::Sub),
      map(char('*'), |_| Token::Mul),
      map(char('/'), |_| Token::Div),
      map(float_one, Token::Num),
      map(key, Token::Var),
      open_paren,
    )),
  ))
  .parse_complete(i)
}

fn open_paren(i: &str) -> IResult<&str, Token<'_>> {
  delimited(char('('), map(tokens, Token::Paren), char(')')).parse_complete(i)
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn tokenize() {
    tokens("-0.5 ;");
    tokens("VDD + 0.5 ;");
    let (_, t) = tokens("0.3*(-A1+0.1) + 0.5 ;").unwrap();
    let expr = parse_formula(&t).unwrap();
    println!("{expr}");
    let (_, t) = tokens("-+A ;").unwrap();
    parse_formula(&t);
    let (_, t) = tokens("A+ ;").unwrap();
    parse_formula(&t);
  }
  #[test]
  fn parse_fmt_self_check() {
    for (should_success, s) in [
      (true, "-0.5 ;"),
      (true, "VDD + 0.5 ;"),
      (true, "(A+B)*(C+D);"),
      (true, "0.3*(-A1+0.1) + 0.5 ;"),
      (true, "-+A ;"),
      (false, "A+ ;"),
      (false, "- ;"),
      (false, "(A"),
    ] {
      println!("----");
      println!("origin:   {s}");
      let parse_res = FormulaExpr::parse(s);
      if should_success {
        if let Ok((_, expr)) = parse_res {
          println!("parsed:   {expr}");
          let fmt_s = expr.to_string();
          let reparse_res = FormulaExpr::parse(&fmt_s);
          if let Ok((_, reparse_expr)) = reparse_res {
            println!("reparsed: {reparse_expr}");
            assert_eq!(expr, reparse_expr);
          } else {
            println!("{expr:?}");
            println!("{reparse_res:?}");
            panic!("not equal");
          }
        } else {
          println!("{parse_res:?}");
          panic!("It should success");
        }
      } else if let Err(e) = parse_res {
        println!("{e}");
      } else {
        panic!("It should go wrong");
      }
    }
  }
}
