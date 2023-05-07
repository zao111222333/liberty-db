//!
//!  This crate implement `liberty` data structre in Rust.
//!

pub mod impls;
pub mod parser;

use std::{hash::Hash, fmt::Display};
use nom::{error::Error, IResult};

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


#[derive(thiserror::Error,Debug)]
pub enum ComplexParseError {
  #[error("{0}")]
  Float(std::num::ParseFloatError),
  #[error("title length mismatch")]
  LengthDismatch,
  #[error("other")]
  Other,
  #[error("unsurpport word")]
  UnsupportedWord,
}

/// Complex Attribute in Liberty
pub trait ComplexAttri: Sized {
  /// Parser error
  type Error: std::error::Error;
  /// basic parser
  fn parse(v: &Vec<Vec<&str>>)->Result<Self,Self::Error>;
  /// to_wrapper
  fn to_wrapper(&self) -> Option<ComplexWrapper>;
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

/// Group Attribute with hased property in Liberty, e.g. [Cell](crate::cell::Cell)
pub trait HashedGroup: Sized {
  /// its Index
  type Idx: Sized+Hash;
  /// generate title for wrapper
  fn title(&self) -> Vec<String>;
  /// generate idx from self
  fn idx(&self) -> &Self::Idx;
  fn idx_clone(&self) -> Self::Idx;
  /// combine `self` and `title`, generate index
  fn gen_idx(&self, title: Vec<String>) -> Result<Self::Idx,IdxError>;
}

/// Group Attribute in Liberty
/// 
/// Use `#[derive(liberty_macros::Group)]` or 
/// 
/// `#[derive(liberty_macros::GroupHashed,liberty_macros::NameIdx)]`
pub trait GroupAttri: Sized + std::fmt::Debug{
  /// `HashedGroup`: `HashMap<<Self as HashedGroup>::Idx,Self>`
  /// 
  /// `Otherwise`: `Vec<Self>`
  type Set;
  /// 
  fn add_undefine_attri(&mut self, key: &str, attri: AttriValue);
  /// nom_parse, will be implemented by macros
  fn nom_parse<'a>(i: &'a str, line_num: &mut usize) -> IResult<&'a str, Result<Self,IdxError>, Error<&'a str>>;
  /// 
  fn to_wrapper(&self) -> GroupWrapper;
}

/// Error for parser Group Index
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum IdxError {
  /// TitleLenMismatch(want,got,title)
  #[error("title length dismatch (want={0},got={1}), title={2:?}")]
  LengthDismatch(usize,usize,Vec<String>),
  /// replace same idx
  #[error("replace same idx")]
  RepeatIdx,
  /// something else
  #[error("{0}")]
  Other(String),
}

/// Error for parser
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum ParserError<'a> {
  /// TitleLenMismatch(want,got,title)
  #[error("Line:{0}, error:{1}")]
  IdxError(usize,IdxError),
  /// replace same idx
  #[error("replace same idx")]
  NomError(usize,nom::Err<Error<&'a str>>),
  // NomError(usize,Error<&'a str>),
  /// something else
  #[error("{0}")]
  Other(usize,String),
}

pub(crate) fn test_parse_group<G:GroupAttri> (s: &str) -> (G,GroupWrapper,usize){
  let mut n= 1;
  match G::nom_parse(s,&mut n){
    Ok((_,Ok(group))) =>{
      println!("{:?}",group);
      println!("{:?}",group.to_wrapper());
      println!("{n}");
      let wrapper = group.to_wrapper();
      return (group,wrapper,n);
    },
    Ok((_,Err(e))) => panic!("{:#?}",e),
    Err(e) => panic!("{:#?}",e),
}
}