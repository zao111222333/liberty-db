//!
//!  This crate implement `liberty` data structre in Rust.
//!

mod fmt;
pub mod impls;
pub mod parser;
pub use fmt::CodeFormatter;
use itertools::Itertools;
use nom::{error::Error, IResult};
use std::{
  cell::RefCell,
  collections::HashMap,
  fmt::{Debug, Display, Write},
  hash::Hash,
  ops::DerefMut,
  str::FromStr,
  sync::Arc,
};
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
#[derive(Debug, Clone, Default)]
pub struct GroupWrapper {
  /// title
  pub title: Vec<String>,
  /// attr_list
  pub attr_list: AttributeList,
}
/// type for UndefinedAttributes, same to `attri_list`
pub type AttributeList = Vec<(String, AttriValue)>;
/// AttriValue for undefined_attribute/serialization
#[derive(Debug, Clone)]
pub enum AttriValue {
  ///
  Simple(SimpleWrapper),
  ///
  Complex(ComplexWrapper),
  ///
  Group(GroupWrapper),
}

/// Error for LinkedGroup
#[derive(Debug)]
#[derive(thiserror::Error)]
// #[derive(PartialEq)]
pub enum LinkError {
  /// Not Find
  #[error("Can not find in hashset!")]
  NotFind,
  /// BorrowError
  #[error("{0}")]
  BorrowError(core::cell::BorrowError),
}

impl PartialEq for LinkError {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::BorrowError(_), Self::BorrowError(_)) => true,
      (LinkError::NotFind, LinkError::NotFind) => true,
      _ => false,
    }
  }
}

/// Reference: https://rustcc.cn/article?id=ac75148b-6eb0-4249-b36d-0a14875b736e
#[derive(Debug, Clone)]
pub struct LinkedGroup<LinkTo>
where
  LinkTo: HashedGroup + GroupAttri,
{
  id: Arc<<LinkTo as HashedGroup>::Id>,
  from: Arc<RefCell<GroupMap<LinkTo>>>,
}

impl<LinkTo: HashedGroup + GroupAttri> LinkedGroup<LinkTo> {
  pub fn new(
    id: Arc<<LinkTo as HashedGroup>::Id>,
    from: &Arc<RefCell<GroupMap<LinkTo>>>,
  ) -> Self {
    Self { id: id.clone(), from: from.clone() }
  }
  pub fn get_linked<F>(&self, f: F)
  where
    F: FnOnce(Result<&LinkTo, LinkError>),
  {
    match self.from.as_ref().try_borrow() {
      Ok(set) => match set.get(&self.id) {
        Some(linked) => f(Ok(linked)),
        None => f(Err(LinkError::NotFind)),
      },
      Err(err) => f(Err(LinkError::BorrowError(err))),
    }
  }
}

/// Simple Attribute in Liberty
pub trait SimpleAttri: Sized + Display + FromStr {
  /// Parser error
  /// basic parser
  #[inline]
  fn parse(s: &str) -> Result<Self, <Self as FromStr>::Err> {
    FromStr::from_str(s)
  }
  /// nom_parse, auto implement
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> IResult<&'a str, Result<Self, (<Self as FromStr>::Err, AttriValue)>, Error<&'a str>>
  {
    let (input, simple) = parser::simple(i, line_num)?;
    match Self::parse(simple) {
      Ok(s) => Ok((input, Ok(s))),
      Err(e) => Ok((input, Err((e, AttriValue::Simple(simple.to_string()))))),
    }
  }
  /// to_wrapper, auto implement
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    format!("{self}")
  }
  #[inline]
  fn fmt_liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> std::fmt::Result {
    <SimpleWrapper as Format>::liberty(&self.to_wrapper(), key, f)
  }
}

#[derive(thiserror::Error, Debug)]
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
  fn parse(v: Vec<&str>) -> Result<Self, Self::Error>;
  /// to_wrapper
  fn to_wrapper(&self) -> ComplexWrapper;
  /// nom_parse, auto implement
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> IResult<&'a str, Result<Self, Self::Error>, Error<&'a str>> {
    let (input, complex) = parser::complex(i, line_num)?;
    match Self::parse(complex) {
      Ok(s) => Ok((input, Ok(s))),
      Err(e) => Ok((input, Err(e))),
    }
  }
  #[inline]
  fn fmt_liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> std::fmt::Result {
    // if let Some(wrapper) = self.to_wrapper() {
    //   <ComplexWrapper as Format>::liberty(&wrapper, key, f)
    // } else {
    //   Ok(())
    // }
    <ComplexWrapper as Format>::liberty(&self.to_wrapper(), key, f)
  }
}

#[derive(Debug, Default)]
pub struct GroupMap<T: HashedGroup> {
  map: HashMap<Arc<<T as HashedGroup>::Id>, T>,
}

impl<T: HashedGroup> GroupMap<T> {
  #[inline]
  pub fn insert(&mut self, v: T) -> Option<T> {
    <Self as DerefMut>::deref_mut(self).insert(v.id(), v)
  }
}
use std::ops::Deref;
impl<T: HashedGroup> Deref for GroupMap<T> {
  type Target = HashMap<Arc<<T as HashedGroup>::Id>, T>;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.map
  }
}

impl<T: HashedGroup> DerefMut for GroupMap<T> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.map
  }
}

/// Group Id
pub type GroupId<T: HashedGroup> = Arc<<T as HashedGroup>::Id>;

/// Group Attribute with hased property in Liberty, e.g. [Cell](crate::cell::Cell)
pub trait HashedGroup {
  /// its Index
  type Id: Sized + Hash + Eq + Debug + Clone;
  // type GroupId = Arc<<Self as HashedGroup>::Id>;
  /// generate title for wrapper
  fn title(&self) -> Vec<String>;
  /// generate id from self
  fn id(&self) -> GroupId<Self>;
  // fn idx_box(&self) -> Box<Self::Id>;
  // fn idx_clone(&self) -> Self::Id;
  /// combine `self` and `title`, generate index
  fn gen_id(&self, title: Vec<String>) -> Result<Self::Id, IdError>;
}

/// Group Attribute in Liberty
///
/// Use `#[derive(liberty_macros::Group)]` or
///
/// `#[derive(liberty_macros::Group,liberty_macros::NameIdx)]`
pub trait GroupAttri: Sized + std::fmt::Debug {
  ///
  fn undefined_list(&mut self) -> &mut AttributeList;
  // fn add_undefine_attri(&mut self, key: &str, attri: AttriValue);
  /// nom_parse, will be implemented by macros
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> IResult<&'a str, Result<Self, IdError>, Error<&'a str>>;
  ///
  // fn to_wrapper(&self) -> GroupWrapper;
  fn fmt_liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> std::fmt::Result;
}

/// Error for parser Group Index
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum IdError {
  /// TitleLenMismatch(want,got,title)
  #[error("title length dismatch (want={0},got={1}), title={2:?}")]
  LengthDismatch(usize, usize, Vec<String>),
  /// replace same id
  #[error("replace same id")]
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
  IdError(usize, IdError),
  /// replace same id
  #[error("replace same id")]
  NomError(usize, nom::Err<Error<&'a str>>),
  // NomError(usize,Error<&'a str>),
  /// something else
  #[error("{0}")]
  Other(usize, String),
}

pub(crate) fn test_parse_group<G: GroupAttri>(s: &str) -> (G, usize) {
  let mut n = 1;
  match G::nom_parse(s, &mut n) {
    Ok((_, Ok(group))) => {
      println!("{:?}", group);
      // println!("{:?}",group.to_wrapper());
      println!("{n}");
      // let wrapper = group.to_wrapper();
      let mut output = String::new();
      let mut f = CodeFormatter::new(&mut output, "| ");
      if let Err(e) = GroupAttri::fmt_liberty(&group, std::any::type_name::<G>(), &mut f)
      {
        panic!("");
      }
      println!("{}", output);
      return (group, n);
    }
    Ok((_, Err(e))) => panic!("{:#?}", e),
    Err(e) => panic!("{:#?}", e),
  }
}
/// For basic formatter
pub trait Format {
  /// `.lib` format
  fn liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> std::fmt::Result;
  /// `.db` format
  fn db<T: Write>(&self, key: &str, f: &mut CodeFormatter<'_, T>) -> std::fmt::Result {
    todo!()
  }
  /// `.json` format
  fn json<T: Write>(&self, key: &str, f: &mut CodeFormatter<'_, T>) -> std::fmt::Result {
    todo!()
  }
}
pub(crate) fn is_word(s: &String) -> bool {
  s.chars().all(parser::char_in_word)
}
impl Format for SimpleWrapper {
  #[inline]
  fn liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> std::fmt::Result {
    if is_word(self) {
      write!(f, "\n{key} : {};", self)
    } else {
      write!(f, "\n{key} : \"{}\";", self)
    }
  }
}
impl Format for ComplexWrapper {
  fn liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> std::fmt::Result {
    if self.is_empty() {
      return write!(f, "\n{key} ();");
    };
    if self[0].iter().all(is_word) {
      write!(f, "\n{key} ({}", self[0].join(","))?;
    } else {
      write!(f, "\n{key} (\"{}\"", self[0].join(","))?;
    }
    f.indent(1);
    for v in self.iter().skip(1) {
      if v.iter().all(is_word) {
        write!(f, ", \\\n{}", v.join(","))?;
      } else {
        write!(f, ", \\\n\"{}\"", v.join(","))?;
      }
    }
    f.dedent(1);
    write!(f, ");")
  }
}

pub(crate) fn liberty_attr_list<T: Write>(
  attr_list: &AttributeList,
  f: &mut CodeFormatter<'_, T>,
) -> std::fmt::Result {
  for (key, attr) in attr_list.iter() {
    match attr {
      AttriValue::Simple(a) => Format::liberty(a, key, f)?,
      AttriValue::Complex(a) => Format::liberty(a, key, f)?,
      AttriValue::Group(a) => Format::liberty(a, key, f)?,
    }
  }
  Ok(())
}

impl Format for GroupWrapper {
  fn liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> std::fmt::Result {
    write!(
      f,
      "\n{key} ({}) {{",
      self
        .title
        .iter()
        .map(|s| if is_word(s) { s.clone() } else { "\"".to_owned() + s + "\"" })
        .join(",")
    )?;
    f.indent(1);
    liberty_attr_list(&self.attr_list, f);
    f.dedent(1);
    write!(f, "\n}}")
  }
}
