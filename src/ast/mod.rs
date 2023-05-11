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
  pub attr_list: AttributeList,
}
/// type for UndefinedAttributes, same to `attri_list`
pub type AttributeList=Vec<(String, AttriValue)>;
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
  #[inline]
  fn liberty<T: Write>(&self, key: &str, f: &mut CodeFormatter<'_, T>) -> std::fmt::Result {
    <SimpleWrapper as Format>::liberty(&self.to_wrapper(), key, f)
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
  fn parse(v: Vec<&str>)->Result<Self,Self::Error>;
  /// to_wrapper
  fn to_wrapper(&self) -> Option<ComplexWrapper>;
  /// nom_parse, auto implement
  #[inline]
  fn nom_parse<'a>(i: &'a str, line_num: &mut usize) -> IResult<&'a str, Result<Self,Self::Error>, Error<&'a str>>{
    let (input,complex) = parser::complex(i,line_num)?;
    match Self::parse(complex){
      Ok(s) => Ok((input,Ok(s))),
      Err(e) => Ok((
        input,
        Err(e)
      )),
    }
  }
  #[inline]
  fn liberty<T: Write>(&self, key: &str, f: &mut CodeFormatter<'_, T>) -> std::fmt::Result {
    if let Some(wrapper) = self.to_wrapper(){
      <ComplexWrapper as Format>::liberty(&wrapper, key, f)
      // if wrapper.is_empty() {return write!(f, "\n{key} ();")};
      // if wrapper[0].iter().all(is_word){
      //   write!(f,"\n{key} ({}", wrapper[0].join(","))?;  
      // }else{
      //   write!(f,"\n{key} (\"{}\"", wrapper[0].join(","))?;
      // }
      // f.indent(1);
      // for v in wrapper.iter().skip(1){
      //   if v.iter().all(is_word){
      //     write!(f, ", \\\n{}",v.join(","))?;
      //   }else{
      //     write!(f, ", \\\n\"{}\"",v.join(","))?;
      //   }
      // }
      // f.dedent(1);
      // write!(f, ");")
    }else {
      Ok(())
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
  // fn to_wrapper(&self) -> GroupWrapper;
  fn liberty<T: Write>(&self, key: &str, f: &mut CodeFormatter<'_, T>) -> std::fmt::Result;
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

pub(crate) fn test_parse_group<G:GroupAttri> (s: &str) -> (G,usize){
  let mut n= 1;
  match G::nom_parse(s,&mut n){
    Ok((_,Ok(group))) =>{
      println!("{:?}",group);
      // println!("{:?}",group.to_wrapper());
      println!("{n}");
      // let wrapper = group.to_wrapper();
      let mut output = String::new();
      let mut f = CodeFormatter::new(&mut output , "| ");
      if let Err(e) = GroupAttri::liberty(&group,"some_key",&mut f){
          panic!("");
      }
      println!("{}",output);
      return (group,n);
    },
    Ok((_,Err(e))) => panic!("{:#?}",e),
    Err(e) => panic!("{:#?}",e),
  }
}
pub trait Format {
  fn liberty<T: Write>(
    &self, 
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> std::fmt::Result;
}
pub(crate) fn is_word(s: &String)->bool{
  s.chars().all(parser::char_in_word)
}
impl Format for SimpleWrapper {
  #[inline]
  fn liberty<T: Write>(&self, key: &str, f: &mut CodeFormatter<'_, T>) -> std::fmt::Result {
    if is_word(self){
      write!(f, "\n{key} : {};",self)
    }else{
      write!(f, "\n{key} : \"{}\";",self)
    }
  }
}
impl Format for ComplexWrapper {
  fn liberty<T: Write>(&self, key: &str, f: &mut CodeFormatter<'_, T>) -> std::fmt::Result {
    if self.is_empty() {return write!(f, "\n{key} ();")};
    if self[0].iter().all(is_word){
      write!(f,"\n{key} ({}", self[0].join(","))?;  
    }else{
      write!(f,"\n{key} (\"{}\"", self[0].join(","))?;
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

pub(crate) fn liberty_attr_list<T: Write>(attr_list: &AttributeList, f: &mut CodeFormatter<'_, T>) -> std::fmt::Result {
  for (key,attr) in attr_list.iter(){
    match attr{
      AttriValue::Simple(a) => Format::liberty(a, key, f)?,
      AttriValue::Complex(a) => Format::liberty(a, key, f)?,
      AttriValue::Group(a) => Format::liberty(a, key, f)?,
    }
  }
  Ok(())
}

impl Format for GroupWrapper {
  fn liberty<T: Write>(&self, key:&str, f: &mut CodeFormatter<'_, T>) -> std::fmt::Result {
    write!(f,"\n{key} ({}) {{", self.title.iter().map(
      |s|
      if is_word(s){
        s.clone()
      }else{
        "\"".to_owned()+s+"\""
      }
    ).join(","))?;
    f.indent(1);
    liberty_attr_list(&self.attr_list, f);
    f.dedent(1);
    write!(f, "\n}}")
  }
}