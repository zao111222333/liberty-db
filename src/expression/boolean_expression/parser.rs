#![allow(
  clippy::non_ascii_literal,
  clippy::indexing_slicing,
  clippy::arithmetic_side_effects,
  clippy::wildcard_enum_match_arm
)]
use crate::ast::parser::{char_in_key, key};
use biodivine_lib_bdd::boolean_expression::BooleanExpression as Expr;
use core::fmt;
use nom::{
  IResult, Parser as _,
  branch::alt,
  bytes::{
    complete::{tag, take_while1},
    escaped, is_not,
  },
  character::{complete::char, one_of},
  combinator::{map, map_res, opt},
  multi::many1,
  sequence::delimited,
};

#[expect(clippy::too_many_lines)]
#[inline]
pub(super) fn _fmt(expr: &Expr, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  match expr {
    Expr::Variable(s) => {
      if s.as_bytes()[0].is_ascii_digit() {
        write!(f, "\\\"{s}\\\"")
      } else {
        write!(f, "{s}")
      }
    }
    Expr::Const(b) => {
      if *b {
        write!(f, "1")
      } else {
        write!(f, "0")
      }
    }
    Expr::Not(e) => match **e {
      Expr::Variable(_) | Expr::Const(_) | Expr::Not(_) => {
        write!(f, "!")?;
        _fmt(e, f)
      }
      _ => {
        write!(f, "!(")?;
        _fmt(e, f)?;
        write!(f, ")")
      }
    },
    Expr::Or(e1, e2) => {
      _fmt(e1, f)?;
      write!(f, "+")?;
      _fmt(e2, f)
    }
    Expr::And(e1, e2) => {
      match **e1 {
        Expr::Or(_, _) => {
          write!(f, "(")?;
          _fmt(e1, f)?;
          write!(f, ")")?;
        }
        _ => _fmt(e1, f)?,
      }
      write!(f, "*")?;
      match **e2 {
        Expr::Or(_, _) => {
          write!(f, "(")?;
          _fmt(e2, f)?;
          write!(f, ")")
        }
        _ => _fmt(e2, f),
      }
    }
    Expr::Xor(e1, e2) => {
      match **e1 {
        Expr::Or(_, _) | Expr::And(_, _) => {
          write!(f, "(")?;
          _fmt(e1, f)?;
          write!(f, ")")?;
        }
        _ => _fmt(e1, f)?,
      }
      write!(f, "^")?;
      match **e2 {
        Expr::Or(_, _) | Expr::And(_, _) => {
          write!(f, "(")?;
          _fmt(e2, f)?;
          write!(f, ")")
        }
        _ => _fmt(e2, f),
      }
    }
    Expr::Cond(e1, e2, e3) => {
      write!(f, "(")?;
      match **e1 {
        Expr::Or(_, _) | Expr::And(_, _) => {
          write!(f, "(")?;
          _fmt(e1, f)?;
          write!(f, ")")?;
        }
        _ => _fmt(e1, f)?,
      }
      write!(f, "?")?;
      match **e2 {
        Expr::Or(_, _) | Expr::And(_, _) => {
          write!(f, "(")?;
          _fmt(e2, f)?;
          write!(f, ")")?;
        }
        _ => _fmt(e2, f)?,
      }
      write!(f, ":")?;
      match **e3 {
        Expr::Or(_, _) | Expr::And(_, _) => {
          write!(f, "(")?;
          _fmt(e3, f)?;
          write!(f, ")")?;
        }
        _ => _fmt(e3, f)?,
      }
      write!(f, ")")
    }
    Expr::Imp(_, _) | Expr::Iff(_, _) => unreachable!(),
  }
}

#[derive(Debug, Eq, PartialEq)]
enum SingleOp {
  Not,
  // !A = A'
  BackNot,
  One,
  Zero,
}
#[derive(Debug, Eq, PartialEq)]
enum BinaryOp {
  Or,
  And,
  Xor,
}
#[derive(Debug, Eq, PartialEq)]
enum Token {
  Space,
  SingleOp(SingleOp),
  BinaryOp(BinaryOp),
  Node(Expr),
  Tokens(Vec<Token>),
}

#[inline]
fn space(i: &str) -> IResult<&str, Token> {
  map(take_while1(move |c: char| matches!(c, '\t' | '\r' | ' ')), |_| Token::Space)
    .parse_complete(i)
}

fn open_b(i: &str) -> IResult<&str, Token> {
  delimited(
    char('('),
    map(many1(alt((space, single_op, open_b, binary_op, node))), |v| {
      Token::Tokens(space_and(v))
    }),
    char(')'),
  )
  .parse_complete(i)
}
/// filter all non-and space
/// A' !B
/// A !B
/// A B
/// A 1B
/// 1 1B
/// 0 1B
/// `t` is B
/// data[i] is A
fn space_and(v: Vec<Token>) -> Vec<Token> {
  let indices_filter: Vec<bool> = v
    .iter()
    .enumerate()
    .map(|(i, t)| match (v.get(i.saturating_sub(1)), t, v.get(i + 1)) {
      (
        Some(Token::Node(_) | Token::Tokens(_) | Token::SingleOp(SingleOp::BackNot)),
        Token::Space,
        Some(
          Token::Node(_)
          | Token::Tokens(_)
          | Token::SingleOp(SingleOp::Not | SingleOp::One | SingleOp::Zero),
        ),
      ) => false,
      (_, Token::Space, _) => true,
      _ => false,
    })
    .collect();
  v.into_iter()
    .enumerate()
    .filter_map(|(i, t)| if indices_filter[i] { None } else { Some(t) })
    .collect()
}
fn single_op(i: &str) -> IResult<&str, Token> {
  alt((
    map(char('!'), |_| Token::SingleOp(SingleOp::Not)),
    map(alt((char('\''), char('’'))), |_| Token::SingleOp(SingleOp::BackNot)),
    map(char('0'), |_| Token::SingleOp(SingleOp::Zero)),
    map(char('1'), |_| Token::SingleOp(SingleOp::One)),
  ))
  .parse_complete(i)
}
fn binary_op(i: &str) -> IResult<&str, Token> {
  alt((
    map(alt((char('+'), char('|'))), |_| Token::BinaryOp(BinaryOp::Or)),
    map(alt((char('&'), char('*'))), |_| Token::BinaryOp(BinaryOp::And)),
    map(char('^'), |_| Token::BinaryOp(BinaryOp::Xor)),
  ))
  .parse_complete(i)
}

fn node(i: &str) -> IResult<&str, Token> {
  alt((
    map(key, |k| Token::Node(Expr::Variable(k.to_owned()))),
    map(delimited(tag(r#"\""#), take_while1(char_in_key), tag(r#"\""#)), |s: &str| {
      Token::Node(Expr::Variable(s.to_owned()))
    }),
  ))
  .parse_complete(i)
}

fn token_vec(i: &str) -> IResult<&str, Vec<Token>> {
  map(many1(alt((space, open_b, single_op, binary_op, node))), space_and)
    .parse_complete(i)
}

/// `BoolExprErr`
#[derive(Clone, Copy, Debug, thiserror::Error, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BoolExprErr {
  /// `Nom`
  #[error("Lexing parser, Nom error")]
  Nom,
  /// `SingleOp`
  #[error("right of single op is not {{signle op / expr}}")]
  SingleOp,
  /// `BinaryOp`
  #[error("binary_op left / right is not expr")]
  BinaryOp,
  /// `Bracket`
  #[error("left-right Bracket mismatch")]
  Bracket,
  /// `BackNot`
  #[error("Can not move back-not")]
  BackNot,
  /// `NoIdea`
  #[error("something go wrong, code=1")]
  NoIdea1,
  /// `NoIdea`
  #[error("something go wrong, code=2")]
  NoIdea2,
  /// `FailToBuildBdd`
  #[error("Fail to build BDD")]
  FailToBuildBdd,
}

/// **(internal)** Utility method to find first occurrence of a specific token in the token tree.
#[inline]
fn index_of_first(data: &[Token], token: &Token) -> Option<usize> {
  data.iter().position(|t| t == token)
}

/// **(internal)** Parse a `ExprToken` tree into a `BooleanExpression` (or error if invalid).
#[inline]
fn parse_formula(data: &[Token]) -> Result<Box<Expr>, BoolExprErr> {
  or(data)
}
/// **(internal)** Recursive parsing step 4: extract `|` operators.
#[inline]
fn or(data: &[Token]) -> Result<Box<Expr>, BoolExprErr> {
  Ok(if let Some(or_token) = index_of_first(data, &Token::BinaryOp(BinaryOp::Or)) {
    Box::new(Expr::Or(and(&data[..or_token])?, or(&data[(or_token + 1)..])?))
  } else {
    and(data)?
  })
}

/// **(internal)** Recursive parsing step 5: extract `&` operators.
#[inline]
fn and(data: &[Token]) -> Result<Box<Expr>, BoolExprErr> {
  Ok(
    if let Some(and_token) = data
      .iter()
      .position(|t| matches!(*t, Token::BinaryOp(BinaryOp::And) | Token::Space))
    {
      Box::new(Expr::And(xor(&data[..and_token])?, and(&data[(and_token + 1)..])?))
    } else {
      xor(data)?
    },
  )
}

/// **(internal)** Recursive parsing step 6: extract `^` operators.
#[inline]
fn xor(data: &[Token]) -> Result<Box<Expr>, BoolExprErr> {
  Ok(if let Some(xor_token) = index_of_first(data, &Token::BinaryOp(BinaryOp::Xor)) {
    Box::new(Expr::Xor(terminal(&data[..xor_token])?, xor(&data[(xor_token + 1)..])?))
  } else {
    terminal(data)?
  })
}

/// **(internal)** Recursive parsing step 7: extract terminals and negations.
#[inline]
fn terminal(data: &[Token]) -> Result<Box<Expr>, BoolExprErr> {
  match (data.first(), data.last()) {
    (None, _) => Err(BoolExprErr::Bracket),
    (Some(Token::SingleOp(SingleOp::Not)), _) => {
      Ok(Box::new(Expr::Not(terminal(&data[1..])?)))
    }
    (Some(Token::SingleOp(SingleOp::Zero)), _) => {
      _ = terminal(&data[1..]);
      Ok(Box::new(Expr::Const(false)))
    }
    (Some(Token::SingleOp(SingleOp::One)), _) => {
      _ = terminal(&data[1..]);
      Ok(Box::new(Expr::Const(true)))
    }
    (_, Some(Token::SingleOp(SingleOp::BackNot))) => {
      Ok(Box::new(Expr::Not(terminal(&data[..(data.len() - 1)])?)))
    }
    (Some(Token::Node(n)), _) => {
      if data.len() == 1 {
        Ok(Box::new(n.clone()))
      } else {
        Err(BoolExprErr::NoIdea2)
      }
    }
    (Some(Token::Tokens(v)), _) => {
      if data.len() == 1 {
        Ok(parse_formula(v)?)
      } else {
        Err(BoolExprErr::NoIdea2)
      }
    }
    _ => Err(BoolExprErr::NoIdea1),
  }
}

impl core::str::FromStr for super::BooleanExpression {
  type Err = BoolExprErr;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match token_vec(s) {
      Ok((_s, tokens)) => {
        if _s.is_empty() {
          Ok(Self { expr: *(parse_formula(&tokens)?) })
        } else {
          Err(BoolExprErr::Nom)
        }
      }
      Err(_) => Err(BoolExprErr::Nom),
    }
  }
}

impl super::BooleanExpression {
  #[inline]
  pub fn parse(i: &str) -> IResult<&str, Self> {
    map_res(token_vec, |tokens| match parse_formula(&tokens) {
      Ok(expr) => Ok(Self { expr: *expr }),
      Err(e) => Err(e),
    })
    .parse_complete(i)
  }
  #[inline]
  pub(super) fn unquote(i: &str) -> IResult<&str, &str> {
    delimited(
      char('"'),
      escaped(opt(alt((tag(r#"\""#), is_not(r#"\""#)))), '\\', one_of(r#"\"rnt"#)),
      char('"'),
    )
    .parse_complete(i)
  }
}
