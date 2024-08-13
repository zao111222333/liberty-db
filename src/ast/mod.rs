//!
//! `liberty` data structre ast
//!

mod fmt;
pub mod parser;
use crate::ArcStr;
use core::{
  fmt::{Debug, Display, Write},
  num::{ParseFloatError, ParseIntError},
  str::FromStr,
};
pub use fmt::CodeFormatter;
use itertools::Itertools;
use nom::{error::Error, IResult};
use ordered_float::ParseNotNanError;
/// Wrapper for simple attribute
pub type SimpleWrapper = ArcStr;
/// Wrapper for complex attribute
pub type ComplexWrapper = Vec<Vec<ArcStr>>;
/// Wrapper for group attribute
///
/// ``` text
/// group_name ( title ) {
///   attri_key1 xxx
///   attri_key2 xxx
/// }
/// ```
#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GroupWrapper {
  /// title
  pub title: Vec<ArcStr>,
  /// attr_list
  pub attr_list: AttributeList,
}
/// type for UndefinedAttributes, same to `attri_list`
pub type AttributeList = Vec<(ArcStr, AttriValue)>;
/// AttriValue for undefined_attribute/serialization
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
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

// /// Reference: https://rustcc.cn/article?id=ac75148b-6eb0-4249-b36d-0a14875b736e
// #[derive(Debug, Clone)]
// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct LinkedGroup<LinkTo>
// where
//   LinkTo: HashedGroup + GroupAttri,
// {
//   id: Arc<<LinkTo as HashedGroup>::Id>,
//   from: Arc<RefCell<GroupMap<LinkTo>>>,
// }

// impl<LinkTo: HashedGroup + GroupAttri> LinkedGroup<LinkTo> {
//   pub fn new(
//     id: Arc<<LinkTo as HashedGroup>::Id>,
//     from: &Arc<RefCell<GroupMap<LinkTo>>>,
//   ) -> Self {
//     Self { id: id.clone(), from: from.clone() }
//   }
//   pub fn get_linked<F>(&self, f: F)
//   where
//     F: FnOnce(Result<&LinkTo, LinkError>),
//   {
//     match self.from.as_ref().try_borrow() {
//       Ok(set) => match set.get(&self.id) {
//         Some(linked) => f(Ok(linked)),
//         None => f(Err(LinkError::NotFind)),
//       },
//       Err(err) => f(Err(LinkError::BorrowError(err))),
//     }
//   }
// }

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
      Err(e) => Ok((input, Err((e, AttriValue::Simple(ArcStr::from(simple)))))),
    }
  }
  // TODO: efficent?
  /// to_wrapper, auto implement
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    format!("{self}").into()
  }
  /// fmt_liberty
  #[inline]
  fn fmt_liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> core::fmt::Result {
    <SimpleWrapper as Format>::liberty(&self.to_wrapper(), key, f)
  }
}

/// ComplexParseError
#[derive(thiserror::Error, Debug)]
pub enum ComplexParseError {
  /// ParseFloatError
  #[error("{0}")]
  Float(ParseNotNanError<ParseFloatError>),
  /// ParseIntError
  #[error("{0}")]
  Int(ParseIntError),
  /// title length mismatch
  #[error("title length mismatch")]
  LengthDismatch,
  /// other error
  #[error("other")]
  Other,
  /// unsurpport word
  #[error("unsurpport word")]
  UnsupportedWord,
}

/// NameAttri
pub trait NameAttri: Sized + Clone {
  /// basic parser
  fn parse(v: Vec<ArcStr>) -> Result<Self, IdError>;
  /// name to_vec
  fn to_vec(self) -> Vec<ArcStr>;
}

/// Complex Attribute in Liberty
pub trait ComplexAttri: Sized {
  /// basic parser
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError>;
  /// to_wrapper
  fn to_wrapper(&self) -> ComplexWrapper;
  /// nom_parse, auto implement
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> IResult<&'a str, Result<Self, (ComplexParseError, AttriValue)>, Error<&'a str>> {
    let (input, complex) = parser::complex(i, line_num)?;
    match Self::parse(&complex) {
      Ok(s) => Ok((input, Ok(s))),
      Err(e) => Ok((
        input,
        Err((
          e,
          AttriValue::Complex(vec![complex.into_iter().map(ArcStr::from).collect()]),
        )),
      )),
    }
  }
  /// fmt_liberty
  #[inline]
  fn fmt_liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> core::fmt::Result {
    <ComplexWrapper as Format>::liberty(&self.to_wrapper(), key, f)
  }
}

/// Group Id
// pub type GroupId<T> = Arc<<T as HashedGroup>::Id>;
pub type GroupComments<T> = <T as GroupAttri>::Comments;

/// AttriComment
pub type AttriComment = Vec<ArcStr>;
/// Group Functions
pub trait GroupFn {
  /// post_process call back
  fn post_process(&mut self) {}
}
/// GroupAttri
pub trait GroupAttri: Sized {
  /// group Name
  type Name;
  /// group Comments
  type Comments;
  /// return name
  fn name(&self) -> Self::Name;
  /// get name
  fn set_name(&mut self, name: Self::Name);
  /// nom_parse, will be implemented by macros
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> IResult<&'a str, Result<Self, IdError>, Error<&'a str>>;
  /// fmt_liberty
  fn fmt_liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> core::fmt::Result;
}

/// Error for parser Group Index
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum IdError {
  /// TitleLenMismatch(want,got,title)
  #[error("title length dismatch (want={0},got={1}), title={2:?}")]
  LengthDismatch(usize, usize, Vec<ArcStr>),
  /// replace same id
  #[error("replace same id")]
  RepeatIdx,
  /// replace same attribute
  #[error("replace same attribute")]
  RepeatAttri,
  /// Int Error
  #[error("{0}")]
  Int(ParseIntError),
  /// something else
  #[error("{0}")]
  Other(String),
}

/// If more than one `#[liberty(name)]`,
/// need to impl `NamedGroup` manually
pub trait NamedGroup: GroupAttri {
  /// parse name from Vec<ArcStr>
  fn parse(v: Vec<ArcStr>) -> Result<Self::Name, IdError>;
  /// name to Vec<ArcStr>
  fn name2vec(name: Self::Name) -> Vec<ArcStr>;
  /// fmt_liberty
  #[inline]
  fn fmt_liberty<T: Write>(&self, f: &mut CodeFormatter<'_, T>) -> core::fmt::Result {
    write!(
      f,
      "{}",
      Self::name2vec(self.name())
        .into_iter()
        .map(|s| if is_word(&s) { s } else { format!("\"{s}\"").into() })
        .join(", ")
    )
  }
}

fn display_nom_error(e: &nom::Err<Error<&str>>) -> ArcStr {
  match e {
    nom::Err::Incomplete(_) => e.to_string(),
    nom::Err::Failure(e) | nom::Err::Error(e) => format!(
      "type[{}] at[{}]",
      e.code.description(),
      e.input.lines().next().unwrap_or("")
    ),
  }
  .into()
}
/// Error for parser
#[derive(Debug, thiserror::Error)]
pub enum ParserError<'a> {
  /// TitleLenMismatch(want,got,title)
  #[error("Line#{0}, {1}")]
  IdError(usize, IdError),
  /// replace same id
  #[error("Line#{0}, {}", display_nom_error(.1))]
  NomError(usize, nom::Err<Error<&'a str>>),
  /// something else
  #[error("Line#{0}, {1}")]
  Other(usize, String),
}

#[allow(unused)]
pub(crate) fn test_parse_group<G: GroupAttri + Debug>(s: &str) -> (G, usize) {
  let mut n = 1;
  match G::nom_parse(s, &mut n) {
    Ok((_, Ok(group))) => {
      println!("{group:#?}");
      println!("{n}");
      let mut output = String::new();
      let mut f = CodeFormatter::new(&mut output);
      if let Err(e) = GroupAttri::fmt_liberty(&group, core::any::type_name::<G>(), &mut f)
      {
        panic!("{e}");
      }
      println!("{output}");
      (group, n)
    }
    Ok((_, Err(e))) => panic!("{e:#?}"),
    Err(e) => panic!("{e:#?}"),
  }
}
/// For basic formatter
pub trait Format {
  /// `.lib` format
  fn liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> core::fmt::Result;
  /// `.db` format
  fn db<T: Write>(&self, key: &str, f: &mut CodeFormatter<'_, T>) -> core::fmt::Result {
    _ = key;
    _ = f;
    todo!()
  }
  /// `.json` format
  fn json<T: Write>(&self, key: &str, f: &mut CodeFormatter<'_, T>) -> core::fmt::Result {
    _ = key;
    _ = f;
    todo!()
  }
}
pub(crate) fn is_word(s: &ArcStr) -> bool {
  !s.is_empty() && s.chars().all(parser::char_in_word)
}
impl Format for AttriComment {
  #[inline]
  fn liberty<T: Write>(
    &self,
    _: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> core::fmt::Result {
    match self.len() {
      0 => Ok(()),
      _ => write!(f, "\n/* {} */", self.join("\n").replace('\n', "\n* ")),
    }
  }
}

impl Format for SimpleWrapper {
  #[inline]
  fn liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> core::fmt::Result {
    if self.is_empty() {
      Ok(())
    } else if is_word(self) {
      write!(f, "\n{key} : {self};")
    } else {
      write!(f, "\n{key} : \"{self}\";")
    }
  }
}

impl Format for ComplexWrapper {
  #[allow(clippy::indexing_slicing)]
  #[inline]
  fn liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> core::fmt::Result {
    if self.is_empty() || (self.len() == 1 && self[0].is_empty()) {
      return Ok(());
    };
    if self[0].iter().all(is_word) {
      write!(f, "\n{key} ({}", self[0].join(", "))?;
    } else {
      write!(f, "\n{key} (\"{}\"", self[0].join(", "))?;
    }
    f.indent(1);
    for v in self.iter().skip(1) {
      if v.iter().all(is_word) {
        write!(f, ", \\\n{}", v.join(", "))?;
      } else {
        write!(f, ", \\\n\"{}\"", v.join(", "))?;
      }
    }
    f.dedent(1);
    write!(f, ");")
  }
}

#[inline]
pub(crate) fn liberty_attr_list<T: Write>(
  attr_list: &AttributeList,
  f: &mut CodeFormatter<'_, T>,
) -> core::fmt::Result {
  for (key, attr) in attr_list {
    match attr {
      AttriValue::Simple(a) => Format::liberty(a, key, f)?,
      AttriValue::Complex(a) => Format::liberty(a, key, f)?,
      AttriValue::Group(a) => Format::liberty(a, key, f)?,
    }
  }
  Ok(())
}

impl Format for GroupWrapper {
  #[inline]
  fn liberty<T: Write>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T>,
  ) -> core::fmt::Result {
    write!(
      f,
      "\n{key} ({}) {{",
      self
        .title
        .iter()
        .map(|s| if is_word(s) { s.clone() } else { format!("\"{s}\"").into() })
        .join(",")
    )?;
    f.indent(1);
    liberty_attr_list(&self.attr_list, f)?;
    f.dedent(1);
    write!(f, "\n}}")
  }
}
