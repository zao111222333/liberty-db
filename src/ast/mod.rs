//!
//!  This crate implement `liberty` data structre in Rust.
//!

pub mod impls;
pub mod parser;
mod fmt;
pub use fmt::CodeFormatter;
use itertools::Itertools;
use std::{hash::Hash, fmt::{Display, Write}};
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
  #[error("{0}")]
  Int(std::num::ParseIntError),
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
/// `#[derive(liberty_macros::Group,liberty_macros::NameIdx)]`
pub trait GroupAttri: Sized + std::fmt::Debug{
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
      let mut output = String::new();
      let mut f = CodeFormatter::new(&mut output , "| ");
      if let Err(e) = Format::liberty(&wrapper, &mut f){
          panic!("");
      }
      println!("{}",output);
      return (group,wrapper,n);
    },
    Ok((_,Err(e))) => panic!("{:#?}",e),
    Err(e) => panic!("{:#?}",e),
  }
}
pub trait Format {
  fn liberty<T: Write>(
    &self, 
    f: &mut CodeFormatter<'_, T>,
  ) -> std::fmt::Result;
}
impl Format for SimpleWrapper {
  fn liberty<T: Write>(&self, f: &mut CodeFormatter<'_, T>) -> std::fmt::Result {
    if is_word(self){
      write!(f, " : {};",self)
    }else{
      write!(f, " : \"{}\";",self)
    }
  }
}

fn is_word(s: &String)->bool{
  s.chars().all(parser::char_in_word)
}
impl Format for ComplexWrapper {
  fn liberty<T: Write>(&self, f: &mut CodeFormatter<'_, T>) -> std::fmt::Result {
    if self.is_empty() {return write!(f, " ();\n")};
    if self[0].iter().all(is_word){
      write!(f," ({}", self[0].join(","))?;  
    }else{
      write!(f," (\"{}\"", self[0].join(","))?;
    }
    f.indent(1);
    for v in self.iter().skip(1){
      if v.iter().all(is_word){
        write!(f, ", \\\n{}",v.join(","))?;
      }else{
        write!(f, ", \\\n\"{}\"",v.join(","))?;
      }
    }
    f.dedent(1);
    write!(f, ");")
  }
}

impl Format for GroupWrapper {
  fn liberty<T: Write>(&self, f: &mut CodeFormatter<'_, T>) -> std::fmt::Result {
    f.indent(1);
    write!(f," ({}) {{\n", self.title.iter().map(
      |s|
      if is_word(s){
        s.clone()
      }else{
        "\"".to_owned()+s+"\""
      }
    ).join(","))?;
    let mut iter = self.attr_list.iter().peekable();
    loop {
      if let Some((key,attr)) = iter.next(){
        write!(f,"{}", key)?;
        match attr{
          AttriValue::Simple(a) => Format::liberty(a, f)?,
          AttriValue::Complex(a) => Format::liberty(a, f)?,
          AttriValue::Group(a) => Format::liberty(a, f)?,
        }
        if iter.peek().is_none(){
          f.dedent(1);
          write!(f,"\n")?;
          break;
        }
        write!(f,"\n")?;
      }
    }
    write!(f, "}}")
  }
}