//!
//!  This crate implement `liberty` data structre in Rust.
//!

pub mod parser;
/// TODO: Remove it
pub mod wrapper;

use std::{hash::Hash, fmt::Display};
use nom::{error::Error,IResult};

/// Wrapper for simple attribute
pub type SimpleWrapper = String;
/// Wrapper for complex attribute
pub type ComplexWrapper = Vec<Vec<String>>;
/// Wrapper for group attribute
/// 
/// ``` text
/// group_name ( title ) {
///   attri_key1 xxx
///   attri_key2 xxx
/// }
/// ```
#[derive(Debug,Clone,Default)]
pub struct GroupWrapper{
  /// title
  pub title: Vec<String>,
  /// attr_list
  pub attr_list: Vec<(String, AttriValue)>,
}
/// type for UndefinedAttributes, same to `attri_list`
pub type UndefinedAttributes=Vec<(String, AttriValue)>;
/// AttriValue for undefined_attribute/serialization
#[derive(Debug,Clone)]
pub enum AttriValue {
  /// 
  Simple(SimpleWrapper),
  /// 
  Complex(ComplexWrapper),
  /// 
  Group(GroupWrapper),
}

/// Simple Attribute in Liberty
pub trait SimpleAttri: Sized + Display{
  /// Parser error
  type Error: std::error::Error;
  /// basic parser
  fn parse(s: &str)->Result<Self, Self::Error>;
  /// nom_parse, auto implement
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
  /// to_wrapper, auto implement
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper{
    format!("{self}")
  }
}
/// Complex Attribute in Liberty
pub trait ComplexAttri: Sized {
  /// Parser error
  type Error: std::error::Error;
  /// basic parser
  fn parse(v: &Vec<Vec<&str>>)->Result<Self,Self::Error>;
  /// to_wrapper
  fn to_wrapper(&self) -> ComplexWrapper;
  /// when `is_empty`, it will not wrapper
  fn is_empty(&self) -> bool;
  /// nom_parse, auto implement
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

/// Index for [HashedGroup]
pub trait GroupIdx: Sized+Hash {
  /// generate title for wrapper
  fn title(&self) -> Vec<String>;
}

/// Group Attribute with hased property in Liberty, e.g. [Cell](crate::cell::Cell)
pub trait HashedGroup: Sized {
  /// its Index
  type Idx: GroupIdx;
  /// combine `self` and `title`, generate index
  fn idx<'a>(&self, title: Vec<&'a str>) -> Result<Self::Idx,IdxError<'a>>;
}

/// Group Attribute in Liberty
/// 
/// Use `#[derive(liberty_macros::Group)]` or 
/// 
/// `#[derive(liberty_macros::GroupHashed,liberty_macros::NameIdx)]`
pub trait GroupAttri: Sized {
  /// `HashedGroup`: `HashMap<<Self as HashedGroup>::Idx,Self>`
  /// 
  /// `Otherwise`: `Vec<Self>`
  type Set;
  /// 
  fn add_undefine_attri(&mut self, key: &str, attri: AttriValue);
  /// nom_parse, will be implemented by macros
  fn nom_parse<'a>(i: &'a str, line_num: &mut usize) -> IResult<&'a str, (Vec<&'a str>,Self), Error<&'a str>>;
  /// 
  fn to_wrapper(&self, title: Vec<String>) -> GroupWrapper;
}

/// Error for parser Group Index
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum IdxError<'a> {
  /// TitleLenMismatch(want,got,title)
  #[error("title length mismatch (want={0},got={1}), title={2:?}")]
  TitleLenMismatch(usize,usize,Vec<&'a str>),
  /// replace same idx
  #[error("replace same idx")]
  RepeatIdx,
  /// something else
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
  fn is_empty(&self) -> bool {
    self.is_empty()
  }
  fn parse<'a>(v: &'a Vec<Vec<&'a str>>)->Result<Self,Self::Error> {
    v.iter().flatten()
      .map(|s| s.parse())
      .collect()
  }

  fn to_wrapper(&self) -> ComplexWrapper {
    vec![self.iter().map(|f|format!("{:.10E}",f)).collect()]
  }
}