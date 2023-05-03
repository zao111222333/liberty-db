//!
//!  This crate implement `liberty` data structre in Rust.
//!

pub mod parser;
pub mod wrapper;

use std::{hash::Hash, fmt::Display};
use nom::{
  error::Error,
  IResult, combinator::map,
};

pub type SimpleWrapper = String;
pub type ComplexWrapper = Vec<Vec<String>>;
#[derive(Debug,Clone)]
pub struct GroupWrapper{
  title: Vec<String>,
  attr_list: Vec<(String, AttriValue)>,
}
// pub type Comment<'a> = Vec<&'a str>;
pub type UndefinedAttributes=Vec<(String, AttriValue)>;
#[derive(Debug,Clone)]
pub enum AttriValue {
  Simple(SimpleWrapper),
  Complex(ComplexWrapper),
  Group(GroupWrapper),
}

pub trait SimpleAttri: Sized + Display{
  type Error: std::error::Error;
  fn parse(s: &str)->Result<Self, Self::Error>;
  #[inline]
  fn nom_parse<'a>(i: &'a str, line_num: &mut usize) -> IResult<&'a str, Result<Self,(Self::Error,AttriValue)>, Error<&'a str>>{
    let (input,simple) = parser::simple(i,line_num)?;
    match Self::parse(simple){
      Ok(s) => Ok((input,Ok(s))),
      Err(e) => Ok((
        input,
        Err((e,AttriValue::Simple(simple.to_string())))
      )),
    }
  }
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper{
    format!("{self}")
  }
}
pub trait ComplexAttri: Sized {
  type Error: std::error::Error;
  fn parse(v: &Vec<Vec<&str>>)->Result<Self,Self::Error>;
  fn to_wrapper(&self) -> ComplexWrapper {todo!()}
  #[inline]
  fn nom_parse<'a>(i: &'a str, line_num: &mut usize) -> IResult<&'a str, Result<Self,(Self::Error,AttriValue)>, Error<&'a str>>{
    let (input,complex) = parser::complex(i,line_num)?;
    match Self::parse(&complex){
      Ok(s) => Ok((input,Ok(s))),
      Err(e) => Ok((
        input,
        Err((e, AttriValue::Complex(
          complex.into_iter()
          .map(|vec_string| vec_string.into_iter().map(|s| s.to_string()).collect())
          .collect()
        )))
      )),
    }
  }
}

pub trait HashedGroup: Sized {
  type Idx: Sized+Hash;
  fn idx<'a>(s: &Self, title: Vec<&'a str>)->Result<Self::Idx,IdxError<'a>>;
}

pub trait GroupAttri: Sized {
  /// `HashedGroup`: `HashMap<<Self as HashedGroup>::Idx,Self>`
  /// 
  /// `Otherwise`: `Vec<Self>`
  type Set;
  fn add_undefine_attri(&mut self, key: &str, attri: AttriValue);
  fn nom_parse<'a>(i: &'a str, line_num: &mut usize) -> IResult<&'a str, (Vec<&'a str>,Self), Error<&'a str>>;
  fn to_wrapper(&self) -> GroupWrapper {todo!()}
}

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum IdxError<'a> {
  /// TitleLenMismatch(want,got,title)
  #[error("title length mismatch (want={0},got={1}), title={2:?}")]
  TitleLenMismatch(usize,usize,Vec<&'a str>),
  #[error("replace same idx")]
  RepeatIdx,
  #[error("{0}")]
  Other(String),
}

impl SimpleAttri for f64 {
  type Error=std::num::ParseFloatError;
  fn parse(s: &str)->Result<Self,Self::Error> {
    s.parse()
  }
}

impl ComplexAttri for Vec<f64> {
  type Error=std::num::ParseFloatError;

  fn parse<'a>(v: &'a Vec<Vec<&'a str>>)->Result<Self,Self::Error> {
    v.iter().flatten()
      .map(|s| s.parse())
      .collect()
  }
}