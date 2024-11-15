//!
//! `liberty` data structre ast
//!

mod fmt;
pub mod parser;
use crate::{library::AttributeType, ArcStr, NotNan};
use core::{fmt::Write, num::ParseIntError, str::FromStr};
pub use fmt::{CodeFormatter, DefaultCodeFormatter, DefaultIndentation, Indentation};
use itertools::Itertools;
use nom::{error::Error, IResult};
use std::{collections::HashMap, hash::Hasher};

const DEFINED_COMMENT: &str = " /* user defined attribute */";
pub(crate) const HASHER: ahash::RandomState =
  ahash::RandomState::with_seeds(1234567, 0, 0, core::u64::MAX);

/// Wrapper for simple attribute
pub type SimpleWrapper = ArcStr;
/// Wrapper for complex attribute
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ComplexWrapper {
  Single(Vec<ArcStr>),
  Multi(Vec<Vec<ArcStr>>),
}
impl ComplexWrapper {
  fn collect(mut vec: Vec<(Vec<&str>, usize)>) -> Self {
    if vec.len() <= 1 {
      vec.pop().map_or(Self::Single(Vec::new()), |first| {
        Self::Single(first.0.into_iter().map_into::<ArcStr>().collect())
      })
    } else {
      Self::Multi(
        vec
          .into_iter()
          .map(|(v, _)| v.into_iter().map_into::<ArcStr>().collect())
          .collect(),
      )
    }
  }
}
/// Wrapper for group attribute
///
/// ``` text
/// group_name ( title ) {
///   attri_key1 xxx
///   attri_key2 xxx
/// }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GroupWrapper {
  /// title
  pub title: Vec<ArcStr>,
  /// `attri_map`
  pub attri_map: Attributes,
}
impl Ord for GroupWrapper {
  #[inline]
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.title.cmp(&other.title)
  }
}
impl PartialOrd for GroupWrapper {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}
/// type for Undefined `Attributes`
pub type Attributes = HashMap<ArcStr, AttriValues>;
/// `AttriValues` for `undefined_attribute/serialization`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum AttriValues {
  /// Defined `Simple`
  Simple(SimpleDefined),
  /// Undefined `Complex`
  Complex(Vec<ComplexWrapper>),
  /// Defined `Group`
  Group(Vec<GroupWrapper>),
}

pub(crate) enum UndefinedAttriValue {
  Simple(SimpleWrapper),
  Complex(ComplexWrapper),
  Group(GroupWrapper),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SimpleDefined {
  /// Boolean `Simple`
  Boolean(Vec<Result<bool, ArcStr>>),
  /// string `Simple`
  String(Vec<ArcStr>),
  /// integer `Simple`
  Integer(Vec<Result<isize, ArcStr>>),
  /// float `Simple`
  Float(Vec<Result<NotNan<f64>, ArcStr>>),
}

#[inline]
pub(crate) fn attributs_fmt_liberty<T: Write, I: Indentation>(
  attributes: &Attributes,
  f: &mut CodeFormatter<'_, T, I>,
) -> core::fmt::Result {
  #[expect(clippy::all)]
  #[inline]
  fn fmt1<T: Write, I: Indentation, U: Format>(
    v: &Vec<U>,
    key: &ArcStr,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    v.iter().try_for_each(|u| Format::liberty(u, key, f))
  }
  #[expect(clippy::all)]
  #[inline]
  fn fmt2<T: Write, I: Indentation, U: SimpleAttri>(
    v: &Vec<Result<U, ArcStr>>,
    key: &ArcStr,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    v.iter().try_for_each(|res_u| {
      match res_u {
        Ok(u) => SimpleAttri::fmt_liberty(u, key, f),
        Err(u) => SimpleAttri::fmt_liberty(u, key, f),
      }
      .and(write!(f, "{DEFINED_COMMENT}"))
    })
  }
  attributes.iter().sorted().try_for_each(|(key, attri)| match attri {
    AttriValues::Simple(SimpleDefined::String(v)) => fmt1(v, key, f),
    AttriValues::Simple(SimpleDefined::Boolean(v)) => fmt2(v, key, f),
    AttriValues::Simple(SimpleDefined::Float(v)) => fmt2(v, key, f),
    AttriValues::Simple(SimpleDefined::Integer(v)) => fmt2(v, key, f),
    AttriValues::Complex(v) => fmt1(v, key, f),
    AttriValues::Group(v) => fmt1(v, key, f),
  })
}
#[inline]
pub(crate) fn attributs_set_undefined_simple(
  attri_map: &mut Attributes,
  key: &str,
  undefined: ArcStr,
) {
  if let Some(AttriValues::Simple(SimpleDefined::String(v))) = attri_map.get_mut(key) {
    v.push(undefined);
  } else {
    _ = attri_map.insert(
      ArcStr::from(key),
      AttriValues::Simple(SimpleDefined::String(vec![undefined])),
    );
  }
}
#[inline]
pub(crate) fn attributs_set_undefined_complex(
  attri_map: &mut Attributes,
  key: &str,
  undefined: ComplexWrapper,
) {
  if let Some(AttriValues::Complex(v)) = attri_map.get_mut(key) {
    v.push(undefined);
  } else {
    _ = attri_map.insert(ArcStr::from(key), AttriValues::Complex(vec![undefined]));
  }
}
#[expect(clippy::too_many_lines)]
#[inline]
pub(crate) fn attributs_set_undefined_attri(
  attri_map: &mut Attributes,
  key: &str,
  group_name: &str,
  scope: &ParseScope,
  undefined: UndefinedAttriValue,
) {
  match scope.define_map.get(&define_id(group_name, key)) {
    None => {
      log::warn!("Line={}; undefined {}", scope.line_num, key);
      if let Some(value) = attri_map.get_mut(key) {
        match (value, undefined) {
          (
            AttriValues::Simple(SimpleDefined::String(v)),
            UndefinedAttriValue::Simple(u),
          ) => {
            v.push(u);
          }
          (AttriValues::Complex(v), UndefinedAttriValue::Complex(u)) => {
            v.push(u);
          }
          (AttriValues::Group(v), UndefinedAttriValue::Group(u)) => {
            v.push(u);
          }
          (_, _) => {
            log::error!(
              "Line={}; Key={key}, the old undefined attribute do NOT meet new one",
              scope.line_num
            );
          }
        }
      } else {
        _ = attri_map.insert(
          ArcStr::from(key),
          match undefined {
            UndefinedAttriValue::Simple(u) => {
              AttriValues::Simple(SimpleDefined::String(vec![u]))
            }
            UndefinedAttriValue::Complex(u) => AttriValues::Complex(vec![u]),
            UndefinedAttriValue::Group(u) => AttriValues::Group(vec![u]),
          },
        );
      }
    }
    Some(DefinedType::Simple(simple_type)) => {
      if let UndefinedAttriValue::Simple(u) = undefined {
        if let Some(value) = attri_map.get_mut(key) {
          match simple_type {
            AttributeType::Boolean => {
              if let AttriValues::Simple(SimpleDefined::Boolean(v)) = value {
                v.push(u.parse().map_or(Err(u), Ok));
              } else {
                log::error!(
                  "Line={}; Key={key}, the old attribute do NOT meet new one",
                  scope.line_num
                );
              }
            }
            AttributeType::String => {
              if let AttriValues::Simple(SimpleDefined::String(v)) = value {
                v.push(u);
              } else {
                log::error!(
                  "Line={}; Key={key}, the old attribute do NOT meet new one",
                  scope.line_num
                );
              }
            }
            AttributeType::Integer => {
              if let AttriValues::Simple(SimpleDefined::Integer(v)) = value {
                v.push(u.parse().map_or(Err(u), Ok));
              } else {
                log::error!(
                  "Line={}; Key={key}, the old attribute do NOT meet new one",
                  scope.line_num
                );
              }
            }
            AttributeType::Float => {
              if let AttriValues::Simple(SimpleDefined::Float(v)) = value {
                v.push(u.parse().map_or(Err(u), Ok));
              } else {
                log::error!(
                  "Line={}; Key={key}, the old attribute do NOT meet new one",
                  scope.line_num
                );
              }
            }
          }
        } else {
          _ = attri_map.insert(
            ArcStr::from(key),
            match simple_type {
              AttributeType::Boolean => {
                AttriValues::Simple(SimpleDefined::Boolean(vec![u
                  .parse()
                  .map_or(Err(u), Ok)]))
              }
              AttributeType::String => {
                AttriValues::Simple(SimpleDefined::String(vec![u]))
              }
              AttributeType::Integer => {
                AttriValues::Simple(SimpleDefined::Integer(vec![u
                  .parse()
                  .map_or(Err(u), Ok)]))
              }
              AttributeType::Float => AttriValues::Simple(SimpleDefined::Float(vec![u
                .parse()
                .map_or(Err(u), Ok)])),
            },
          );
        }
      } else {
        log::error!("Line={}; Key={key}, `defined` got wrong type", scope.line_num);
      }
    }
    Some(DefinedType::Group) => {
      if let UndefinedAttriValue::Group(u) = undefined {
        if let Some(value) = attri_map.get_mut(key) {
          if let AttriValues::Group(v) = value {
            v.push(u);
          } else {
            log::error!(
              "Line={}; Key={key}, the old attribute do NOT meet new one",
              scope.line_num
            );
          }
        } else {
          _ = attri_map.insert(ArcStr::from(key), AttriValues::Group(vec![u]));
        }
      } else {
        log::error!("Line={}; Key={key}, `defined_group` got wrong type", scope.line_num);
      }
    }
  }
}

/// Error for `LinkedGroup`
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum LinkError {
  /// `Not Find`
  #[error("Can not find in hashset!")]
  NotFind,
  /// `BorrowError`
  #[error("{0}")]
  BorrowError(core::cell::BorrowError),
}

impl PartialEq for LinkError {
  #[expect(clippy::match_like_matches_macro)]
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::NotFind, Self::NotFind) | (Self::BorrowError(_), Self::BorrowError(_)) => {
        true
      }
      _ => false,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefinedType {
  Simple(AttributeType),
  Group,
}

#[derive(Debug, Default)]
pub(crate) struct ParseScope {
  pub line_num: usize,
  pub define_map: HashMap<u64, DefinedType>,
}

#[inline]
#[must_use]
pub(crate) fn define_id(group_name: &str, key: &str) -> u64 {
  use std::hash::{BuildHasher, Hash};
  let mut hasher = HASHER.build_hasher();
  group_name.hash(&mut hasher);
  key.hash(&mut hasher);
  hasher.finish()
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

#[derive(Debug, Clone)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DefinedAttribute {
  /// Boolean
  Boolean(Vec<bool>),
  /// string
  String(Vec<ArcStr>),
  /// integer
  Integer(Vec<isize>),
  /// float
  Float(Vec<NotNan<f64>>),
}

pub(crate) type SimpleParseRes<'a, T> =
  IResult<&'a str, Result<T, ArcStr>, Error<&'a str>>;
pub(crate) type ComplexParseRes<'a, T> =
  IResult<&'a str, Result<T, (ComplexParseError, ComplexWrapper)>, Error<&'a str>>;
#[inline]
pub(crate) fn nom_parse_from_str<'a, T: SimpleAttri + FromStr>(
  i: &'a str,
  scope: &mut ParseScope,
) -> SimpleParseRes<'a, T> {
  let (input, s) = parser::simple(i, &mut scope.line_num)?;
  s.parse()
    .map_or(Ok((input, Err(ArcStr::from(s)))), |simple| Ok((input, Ok(simple))))
}

/// Simple Attribute in Liberty
pub(crate) trait SimpleAttri: Sized + core::fmt::Display {
  /// `nom_parse`, auto implement
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> SimpleParseRes<'a, Self>;
  #[inline]
  fn is_set(&self) -> bool {
    true
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    write!(f, "{self}")
  }
  /// `fmt_liberty`
  #[inline]
  fn fmt_liberty<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    if self.is_set() {
      write!(f, "\n{}{key} : ", f.indentation())?;
      self.fmt_self(f)?;
      write!(f, ";")
    } else {
      Ok(())
    }
  }
}

/// `ComplexParseError`
#[derive(thiserror::Error, Debug)]
pub enum ComplexParseError {
  /// `ParseFloatError`
  #[error("{0}")]
  Float(#[from] fast_float2::Error),
  // Float(#[from] ParseNotNanError<ParseFloatError>),
  /// `ParseIntError`
  #[error("{0}")]
  Int(#[from] ParseIntError),
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

#[inline]
pub fn join_fmt<
  'a,
  T: Sized + 'a,
  I: Iterator<Item = T>,
  W: Write,
  F: FnMut(T, &mut W) -> core::fmt::Result,
>(
  iter: I,
  f: &mut W,
  func: F,
  sep: &str,
) -> core::fmt::Result {
  write!(f, "\"")?;
  join_fmt_no_quote(iter, f, func, sep)?;
  write!(f, "\"")
}

#[inline]
pub fn join_fmt_no_quote<
  'a,
  T: Sized + 'a,
  I: Iterator<Item = T>,
  W: Write,
  F: FnMut(T, &mut W) -> core::fmt::Result,
>(
  mut iter: I,
  f: &mut W,
  mut func: F,
  sep: &str,
) -> core::fmt::Result {
  if let Some(first) = iter.next() {
    func(first, f)?;
    while let Some(t) = iter.next() {
      write!(f, "{sep}")?;
      func(t, f)?;
    }
  }
  Ok(())
}

/// Complex Attribute in Liberty
pub(crate) trait ComplexAttri: Sized {
  /// basic `parser`
  fn parse<'a, I: Iterator<Item = &'a Vec<&'a str>>>(
    iter: I,
    scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError>;
  /// `nom_parse`, auto implement
  #[expect(clippy::arithmetic_side_effects)]
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ComplexParseRes<'a, Self> {
    let (input, vec) = parser::complex(i, &mut scope.line_num)?;
    let mut line_num = 0;
    let iter = vec.iter().map(|(v, n)| {
      line_num += n;
      v
    });
    let res = Self::parse(iter, scope);
    scope.line_num += line_num;
    match res {
      Ok(s) => Ok((input, Ok(s))),
      Err(e) => Ok((input, Err((e, ComplexWrapper::collect(vec))))),
    }
  }
  #[inline]
  fn is_set(&self) -> bool {
    true
  }
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result;
  /// `fmt_liberty`
  #[inline]
  fn fmt_liberty<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    if self.is_set() {
      let indent1 = f.indentation();
      write!(f, "\n{indent1}{key} (")?;
      f.indent(1);
      self.fmt_self(f)?;
      f.dedent(1);
      write!(f, ");")
    } else {
      Ok(())
    }
  }
}

/// `GroupComments`
pub type GroupComments<T> = <T as Group>::Comments;

/// `AttriComment`
pub type AttriComment = Vec<ArcStr>;
/// Group Functions
pub(crate) trait GroupFn {
  /// `post_parse_process` call back
  #[inline]
  fn post_parse_process(&mut self, _scope: &mut ParseScope) {}
}
/// Export Group APIs
#[expect(private_bounds)]
pub trait Group: Sized + GroupAttri {
  /// group Comments
  type Comments;
  /// `test_wrapper`
  #[inline]
  fn display(&self) -> GroupDisplay<'_, Self> {
    GroupDisplay { inner: self }
  }
}
/// `GroupAttri`, internal Group APIs
pub(crate) trait GroupAttri: Sized {
  /// `nom_parse`, will be implemented by macros
  fn nom_parse<'a>(
    i: &'a str,
    group_name: &str,
    scope: &mut ParseScope,
  ) -> IResult<&'a str, Result<Self, IdError>, Error<&'a str>>;
  /// `fmt_liberty`
  fn fmt_liberty<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result;
}

/// Error for parser Group Index
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum IdError {
  /// TitleLenMismatch(want,got,title)
  #[error("title length dismatch (want={want},got={got}), title={title:?}")]
  LengthDismatch { want: usize, got: usize, title: Vec<ArcStr> },
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

impl IdError {
  #[inline]
  pub(crate) fn length_dismatch(want: usize, got: usize, v: Vec<&str>) -> Self {
    Self::LengthDismatch {
      want,
      got,
      title: v.into_iter().map(ArcStr::from).collect(),
    }
  }
}

/// If more than one `#[liberty(name)]`,
/// need to impl `NamedGroup` manually
pub(crate) trait NamedGroup: GroupAttri {
  /// parse name from `v: &[&str]` and then set self
  fn parse_set_name(&mut self, v: Vec<&str>) -> Result<(), IdError>;
  /// `fmt_liberty`
  fn fmt_name<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result;
}

/// `NameAttri`
pub trait NameAttri: Sized {
  /// basic parser
  fn parse(v: Vec<&str>) -> Result<Self, IdError>;
  /// name `to_vec`
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result;
}

/// Error for parser
#[derive(Debug, thiserror::Error)]
pub enum ParserError {
  /// TitleLenMismatch(want,got,title)
  #[error("Line#{0}, {1}")]
  IdError(usize, IdError),
  /// replace same id
  #[error("Line#{0}, {1}")]
  NomError(usize, String),
  /// something else
  #[error("Line#{0}, {1}")]
  Other(usize, String),
}

impl ParserError {
  #[inline]
  pub(crate) fn nom(line: usize, e: nom::Err<Error<&str>>) -> Self {
    Self::NomError(
      line,
      match e {
        nom::Err::Incomplete(_) => e.to_string(),
        nom::Err::Failure(_e) | nom::Err::Error(_e) => format!(
          "type [{}] at [{}]",
          _e.code.description(),
          _e.input.lines().next().unwrap_or("")
        ),
      },
    )
  }
}

/// `GroupDisplay`
#[derive(Debug)]
pub struct GroupDisplay<'a, G> {
  pub inner: &'a G,
}

impl<'a, G: GroupAttri> core::fmt::Display for GroupDisplay<'a, G> {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ff = DefaultCodeFormatter::new(f);
    self.inner.fmt_liberty(core::any::type_name::<G>(), &mut ff)
  }
}

#[cfg(test)]
#[inline]
pub(crate) fn test_parse<G: GroupAttri + Group>(input: &str) -> G {
  let mut scope = ParseScope::default();
  let g = match G::nom_parse(input, "", &mut scope) {
    Ok((_, Ok(g))) => g,
    Ok((_, Err(e))) => panic!("{e:#?}"),
    Err(e) => panic!("{e:#?}"),
  };
  println!("{}", g.display());
  g
}

#[cfg(test)]
#[inline]
pub(crate) fn test_parse_fmt<G: GroupAttri + Group>(input: &str, fmt_want: &str) -> G {
  let mut scope = ParseScope::default();
  let g = match G::nom_parse(input, "", &mut scope) {
    Ok((_, Ok(g))) => g,
    Ok((_, Err(e))) => panic!("{e:#?}"),
    Err(e) => panic!("{e:#?}"),
  };
  let fmt_str = g.display().to_string();
  println!("{fmt_str}");
  dev_utils::text_diff(fmt_want, fmt_str.as_str());
  g
}

/// For basic formatter
pub(crate) trait Format {
  /// `.lib` format
  fn liberty<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result;
  /// `.db` format
  #[expect(dead_code)]
  #[inline]
  fn db<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    _ = key;
    _ = f;
    todo!()
  }
  /// `.json` format
  #[expect(dead_code)]
  #[inline]
  fn json<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    _ = key;
    _ = f;
    todo!()
  }
}
#[inline]
pub(crate) fn is_word(s: &ArcStr) -> bool {
  !s.is_empty() && s.chars().all(parser::char_in_word)
}
/// For basic formatter
impl Format for AttriComment {
  #[inline]
  fn liberty<T: Write, I: Indentation>(
    &self,
    _: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    if self.is_empty() {
      Ok(())
    } else {
      let indent = f.indentation();
      write!(f, "\n{indent}/* ")?;
      join_fmt_no_quote(
        self.iter().flat_map(|lines| lines.split('\n')),
        f,
        |line, ff| write!(ff, "{line}"),
        format!("\n{indent}** ").as_str(),
      )?;
      write!(f, " */")
    }
  }
}

#[inline]
pub(crate) fn fmt_library_beginning<T: Write, I: Indentation>(
  comment: &AttriComment,
  f: &mut CodeFormatter<'_, T, I>,
) -> core::fmt::Result {
  if comment.is_empty() {
    write!(
      f,
      "/* Generated by {} {} */",
      env!("CARGO_PKG_NAME"),
      env!("CARGO_PKG_VERSION"),
    )
  } else {
    write!(f, "/* ")?;
    join_fmt_no_quote(
      comment.iter().flat_map(|lines| lines.split('\n')),
      f,
      |line, ff| write!(ff, "{line}"),
      "\n** ",
    )?;
    write!(f, " */")
  }
}
impl Format for SimpleWrapper {
  #[inline]
  fn liberty<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    SimpleAttri::fmt_liberty(self, key, f).and(write!(f, "{DEFINED_COMMENT}"))
  }
}

impl Format for ComplexWrapper {
  #[inline]
  fn liberty<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    match self {
      Self::Single(signle) => {
        ComplexAttri::fmt_liberty(signle, key, f).and(write!(f, "{DEFINED_COMMENT}"))
      }
      Self::Multi(multi) => {
        let indent = f.indentation();
        write!(f, "\n{indent}{key} (")?;
        let mut iter = multi.iter();
        if let Some(v) = iter.next() {
          join_fmt(v.iter(), f, |s, ff| write!(ff, "{s}"), ", ")?;
        }
        while let Some(v) = iter.next() {
          write!(f, ", \\\n{indent}")?;
          join_fmt(v.iter(), f, |s, ff| write!(ff, "{s}"), ", ")?;
        }
        write!(f, "); {DEFINED_COMMENT}")
      }
    }
  }
}

impl Format for GroupWrapper {
  #[inline]
  fn liberty<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    let indent = f.indentation();
    write!(f, "\n{indent}{key} (")?;
    join_fmt_no_quote(
      self.title.iter(),
      f,
      |s, ff| if is_word(s) { write!(ff, "{s}") } else { write!(ff, "\"{s}\"") },
      ", ",
    )?;
    write!(f, ") {{ {DEFINED_COMMENT}")?;
    f.indent(1);
    attributs_fmt_liberty(&self.attri_map, f)?;
    f.dedent(1);
    write!(f, "\n{indent}}}")
  }
}
