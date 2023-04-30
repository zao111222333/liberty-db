//!
//!  This crate implement `liberty` data structre in Rust.
//!

pub mod wrapper;
pub mod parser;

use std::hash::Hash;
use nom::{
  error::Error,
  IResult,
};

pub type SimpleWrapper = String;
pub type ComplexWrapper = Vec<String>;
#[derive(Debug)]
pub struct GroupWrapper{
  title: ComplexWrapper, 
  attr_list: Vec<(String, AttriValue)>,
}
// pub type Comment<'a> = Vec<&'a str>;
pub type UndefinedAttributes=Vec<(String, AttriValue)>;
#[derive(Debug)]
pub enum AttriValue {
  Simple(SimpleWrapper),
  Complex(ComplexWrapper),
  Group(GroupWrapper),
}

// #[derive(Debug)]
// pub enum AttriType {
//   SimpleSingle,
//   SimpleMulti,
//   Complex1d,
//   Complex2d,
//   ComplexSpecial,
//   Group(GroupFn),
//   Undefine,
// }

pub trait SimpleAttri: Sized {
  type Error: std::error::Error;
  fn parse<'a>(s: &'a str)->Result<Self,Self::Error>;
}
pub trait ComplexAttri: Sized {
  type Error: std::error::Error;
  fn parse<'a>(v: &'a Vec<&'a str>)->Result<Self,Self::Error>;
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

pub trait GroupIdx<G:GroupAttri>: Sized+Hash{
  fn new<'a>(attri:&G, title: Vec<&'a str>)-> Result<Self,IdxError<'a>>;
}

pub trait GroupAttri: Sized+Default {
  type Idx: GroupIdx<Self>;
  type Set;
  fn push_self<'a>(self, title: Vec<&'a str>, set: &mut Self::Set)->Result<(),IdxError<'a>>;
  fn add_undefine_attri(&mut self, key: &str, attri: AttriValue);
}
pub trait GroupParser: GroupAttri {
  fn parse_push2set<'a>(i: &'a str, set: &mut Self::Set, line_num: &mut usize) -> IResult<&'a str, (), Error<&'a str>>;
}

impl SimpleAttri for f64 {
  type Error=std::num::ParseFloatError;
  fn parse<'a>(s: &'a str)->Result<Self,Self::Error> {
    s.parse()
  }
}

impl ComplexAttri for Vec<f64> {
  type Error=std::num::ParseFloatError;
  fn parse<'a>(v: &Vec<&'a str>)->Result<Self,Self::Error> {
    v.iter().map(|s| s.parse()).collect()
  }  
}