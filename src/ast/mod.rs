//!
//! `liberty` data structre ast
//!

mod fmt;
pub mod parser;
use crate::ArcStr;
use core::{
  fmt::Write,
  num::{ParseFloatError, ParseIntError},
  str::FromStr,
};
pub use fmt::{
  CodeFormatter, DefaultCodeFormatter, DefaultIndentation, Indentation,
  TestCodeFormatter, TestIndentation,
};
use nom::{error::Error, IResult};
use ordered_float::ParseNotNanError;
/// Wrapper for simple attribute
pub type SimpleWrapper = ArcStr;
/// Wrapper for complex attribute
pub type ComplexWrapper = Vec<Vec<ArcStr>>;
// pub type ComplexWrapper = (Vec<ArcStr>, ComplexShape);

// pub enum ComplexShape {
//   SingleLine,
//   Table { size1: usize, size2: usize },
//   Arbitrary(Vec<usize>),
// }
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
  /// `attr_list`
  pub attr_list: AttributeList,
}
/// type for Undefined `AttributeList`
pub type AttributeList = Vec<(ArcStr, AttriValue)>;
/// `AttriValue` for `undefined_attribute/serialization`
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum AttriValue {
  /// `Simple`
  Simple(SimpleWrapper),
  /// `Complex`
  Complex(ComplexWrapper),
  /// `Group`
  Group(GroupWrapper),
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
  #[allow(clippy::match_like_matches_macro)]
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

pub(crate) type SimpleParseErr<'a, T> =
  IResult<&'a str, Result<T, AttriValue>, Error<&'a str>>;

#[inline]
pub fn nom_parse_from_str<'a, T: SimpleAttri + FromStr>(
  i: &'a str,
  line_num: &mut usize,
) -> SimpleParseErr<'a, T> {
  let (input, s) = parser::simple(i, line_num)?;
  s.parse()
    .map_or(Ok((input, Err(AttriValue::Simple(ArcStr::from(s))))), |simple| {
      Ok((input, Ok(simple)))
    })
}

/// Simple Attribute in Liberty
pub trait SimpleAttri: Sized + core::fmt::Display {
  /// `nom_parse`, auto implement
  fn nom_parse<'a>(i: &'a str, line_num: &mut usize) -> SimpleParseErr<'a, Self>;
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
  Float(#[from] ParseNotNanError<ParseFloatError>),
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
pub trait ComplexAttri: Sized {
  /// basic `parser`
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError>;
  /// `nom_parse`, auto implement
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
pub type GroupComments<T> = <T as GroupAttri>::Comments;

/// `AttriComment`
pub type AttriComment = Vec<ArcStr>;
/// Group Functions
pub trait GroupFn {
  /// `post_process` call back
  #[inline]
  fn post_process(&mut self) {}
}
/// `GroupAttri`
pub trait GroupAttri: Sized {
  /// group Comments
  type Comments;
  /// `test_wrapper`
  #[inline]
  fn test_wrapper(self) -> TestWrapper<Self> {
    TestWrapper { inner: self, line_count: 0 }
  }
  /// `nom_parse`, will be implemented by macros
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
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
pub trait NamedGroup: GroupAttri {
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

fn display_nom_error(e: &nom::Err<Error<&str>>) -> ArcStr {
  match e {
    nom::Err::Incomplete(_) => e.to_string(),
    nom::Err::Failure(_e) | nom::Err::Error(_e) => format!(
      "type[{}] at[{}]",
      _e.code.description(),
      _e.input.lines().next().unwrap_or("")
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

/// `TestWrapper`
#[derive(Debug)]
pub struct TestWrapper<G> {
  pub inner: G,
  pub line_count: usize,
}

impl<G: GroupAttri> FromStr for TestWrapper<G> {
  type Err = String;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut line_count = 1;
    match G::nom_parse(s, &mut line_count) {
      Ok((_, Ok(inner))) => Ok(Self { inner, line_count }),
      Ok((_, Err(e))) => Err(format!("{e:#?}")),
      Err(e) => Err(format!("{e:#?}")),
    }
  }
}

impl<G: GroupAttri> core::fmt::Display for TestWrapper<G> {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ff = TestCodeFormatter::new(f);
    self.inner.fmt_liberty(core::any::type_name::<G>(), &mut ff)
  }
}

#[cfg(test)]
#[inline]
pub fn test_parse<G: GroupAttri + core::fmt::Debug>(input: &str) -> G {
  let wrapper = input.parse::<TestWrapper<G>>().expect("Group parse failed");
  println!("{:?}", wrapper.inner);
  println!("{wrapper}");
  wrapper.inner
}

#[cfg(test)]
#[inline]
pub fn test_parse_fmt<G: GroupAttri + core::fmt::Debug>(
  input: &str,
  fmt_want: &str,
) -> G {
  let wrapper = input.parse::<TestWrapper<G>>().expect("Group parse failed");
  let fmt_str = wrapper.to_string();
  println!("{fmt_str}");
  crate::util::text_diff(fmt_want, fmt_str.as_str());
  wrapper.inner
}

/// For basic formatter
pub trait Format {
  /// `.lib` format
  fn liberty<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result;
  /// `.db` format
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
impl Format for AttriComment {
  /// See [`core::str::replace`](core::str::replace)
  #[allow(clippy::arithmetic_side_effects, clippy::undocumented_unsafe_blocks)]
  #[inline]
  fn liberty<T: Write, I: Indentation>(
    &self,
    _: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    if self.is_empty() {
      Ok(())
    } else {
      let mut first_line = true;
      let mut first_char = || {
        if first_line {
          first_line = false;
          '/'
        } else {
          '*'
        }
      };
      let indent = f.indentation();
      self.iter().try_for_each(|line| {
        let mut last_end = 0;
        line.match_indices('\n').try_for_each(|(start, _)| {
          // see [alloc::str::replace]
          let s = unsafe { line.get_unchecked(last_end..start) };
          last_end = start + 1;
          f.write_fmt(format_args!("\n{indent}{}* {s}", first_char()))
        })?;
        // see [alloc::str::replace]
        let s = unsafe { line.get_unchecked(last_end..line.len()) };
        f.write_fmt(format_args!("\n{indent}{}* {s}", first_char()))
      })?;
      f.write_str(" */")
    }
  }
}

#[allow(clippy::arithmetic_side_effects, clippy::undocumented_unsafe_blocks)]
#[inline]
pub(crate) fn fmt_first_line_comment<T: Write, I: Indentation>(
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
    let mut first_line = true;
    let mut first_chars = || {
      if first_line {
        first_line = false;
        "/"
      } else {
        "\n*"
      }
    };
    comment.iter().try_for_each(|line| {
      let mut last_end = 0;
      line.match_indices('\n').try_for_each(|(start, _)| {
        // see [alloc::str::replace]
        let s = unsafe { line.get_unchecked(last_end..start) };
        last_end = start + 1;
        f.write_fmt(format_args!("{}* {s}", first_chars()))
      })?;
      // see [alloc::str::replace]
      let s = unsafe { line.get_unchecked(last_end..line.len()) };
      f.write_fmt(format_args!("{}* {s}", first_chars()))
    })?;
    f.write_str(" */")
  }
}

impl Format for SimpleWrapper {
  #[inline]
  fn liberty<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    if self.is_empty() {
      Ok(())
    } else if is_word(self) {
      write!(f, "\n{}{key} : {self};", f.indentation())
    } else {
      write!(f, "\n{}{key} : \"{self}\";", f.indentation())
    }
  }
}

impl Format for ComplexWrapper {
  #[allow(clippy::indexing_slicing)]
  #[inline]
  fn liberty<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    if self.is_empty() || (self.len() == 1 && self[0].is_empty()) {
      return Ok(());
    };
    let indent1 = f.indentation();
    if self[0].iter().all(is_word) {
      write!(f, "\n{indent1}{key} ({}", self[0].join(", "))?;
    } else {
      write!(f, "\n{indent1}{key} (\"{}\"", self[0].join(", "))?;
    }
    f.indent(1);
    let indent2 = f.indentation();
    for v in self.iter().skip(1) {
      if v.iter().all(is_word) {
        write!(f, ", \\\n{indent2}{}", v.join(", "))?;
      } else {
        write!(f, ", \\\n{indent2}\"{}\"", v.join(", "))?;
      }
    }
    f.dedent(1);
    write!(f, ");")
  }
}

#[inline]
pub(crate) fn liberty_attr_list<T: Write, I: Indentation>(
  attr_list: &AttributeList,
  f: &mut CodeFormatter<'_, T, I>,
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
    write!(f, ") {{")?;
    f.indent(1);
    liberty_attr_list(&self.attr_list, f)?;
    f.dedent(1);
    write!(f, "\n{indent}}}")
  }
}
