//!
//! implement basic types
//!
use crate::{
  ast::{
    self, is_word, join_fmt_no_quote, CodeFormatter, ComplexAttri, ComplexParseRes,
    IdError, Indentation, NameAttri, ParseScope, SimpleAttri,
  },
  ArcStr, NotNan,
};
use core::fmt::{self, Write};
use itertools::Itertools;

use super::items::{Formula, NameList, WordSet};

impl SimpleAttri for NotNan<f64> {
  #[inline]
  #[expect(clippy::undocumented_unsafe_blocks)]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(i, &mut scope.line_num, ast::parser::parse_float)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_float(self.into_inner())
  }
}

impl SimpleAttri for bool {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(i, &mut scope.line_num, ast::parser::parse_bool)
  }
}
impl SimpleAttri for usize {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(i, &mut scope.line_num, ast::parser::parse_usize)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_int(*self)
  }
}

impl SimpleAttri for isize {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::nom_parse_from_str(i, scope)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_int(*self)
  }
}

impl NameAttri for Option<ArcStr> {
  #[inline]
  fn parse(mut v: Vec<&str>) -> Result<Self, IdError> {
    let l = v.len();
    if l > 1 {
      Err(IdError::length_dismatch(1, l, v))
    } else {
      Ok(v.pop().map(ArcStr::from))
    }
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    self.as_ref().map_or(Ok(()), |s| {
      if is_word(s) {
        write!(f, "{s}")
      } else {
        write!(f, "\"{s}\"")
      }
    })
  }
}

impl NameAttri for ArcStr {
  #[inline]
  fn parse(mut v: Vec<&str>) -> Result<Self, IdError> {
    let l = v.len();
    if l != 1 {
      return Err(IdError::length_dismatch(1, l, v));
    }
    v.pop()
      .map_or(Err(IdError::Other("Unkown pop error".into())), |s| Ok(s.into()))
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    if is_word(self) {
      write!(f, "{self}")
    } else {
      write!(f, "\"{self}\"")
    }
  }
}

impl NameAttri for NameList {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, IdError> {
    let l = v.len();
    match l {
      0 => Err(IdError::length_dismatch(1, 0, v)),
      #[expect(clippy::indexing_slicing)]
      1 => Ok(Self::Name(v[0].into())),
      _ => Ok(Self::List(WordSet { inner: v.into_iter().map(ArcStr::from).collect() })),
    }
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "{self}")
  }
}

impl fmt::Display for NameList {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Name(s) => {
        if is_word(s) {
          write!(f, "{s}")
        } else {
          write!(f, "\"{s}\"")
        }
      }
      Self::List(set) => join_fmt_no_quote(
        set.inner.iter().sorted(),
        f,
        |s, ff| if is_word(s) { write!(ff, "{s}") } else { write!(ff, "\"{s}\"") },
        ", ",
      ),
    }
  }
}

impl NameAttri for Vec<ArcStr> {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, IdError> {
    Ok(v.into_iter().map(ArcStr::from).collect())
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    join_fmt_no_quote(
      self.iter(),
      f,
      |s, ff| if is_word(s) { write!(ff, "{s}") } else { write!(ff, "\"{s}\"") },
      ", ",
    )
  }
}
impl<const N: usize> NameAttri for [ArcStr; N] {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, IdError> {
    let l = v.len();
    if l == N {
      match TryInto::<[ArcStr; N]>::try_into(
        v.into_iter().map(ArcStr::from).collect::<Vec<ArcStr>>(),
      ) {
        Ok(name) => Ok(name),
        Err(e) => Err(IdError::Other(format!("try_into error: {e:?}"))),
      }
    } else {
      Err(IdError::length_dismatch(N, l, v))
    }
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    join_fmt_no_quote(
      self.iter(),
      f,
      |s, ff| if is_word(s) { write!(ff, "{s}") } else { write!(ff, "\"{s}\"") },
      ", ",
    )
  }
}

impl SimpleAttri for ArcStr {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::nom_parse_from_str(i, scope)
  }
  #[inline]
  fn is_set(&self) -> bool {
    !self.is_empty()
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    if is_word(self) {
      write!(f, "{self}")
    } else {
      write!(f, "\"{self}\"")
    }
  }
}

// impl<const N: usize> ComplexAttri for [ArcStr; N] {
//   #[inline]
//   fn parse<'a, I: Iterator<Item = &'a &'a str>>(
//     iter: I,
//     _scope: &mut ParseScope,
//   ) -> Result<Self, ComplexParseError> {
//     let v = iter.map(|&s| ArcStr::from(s)).collect::<Vec<ArcStr>>();
//     if v.len() == N {
//       TryInto::<[ArcStr; N]>::try_into(v).map_or(Err(ComplexParseError::Other), Ok)
//     } else {
//       Err(ComplexParseError::LengthDismatch)
//     }
//   }
//   #[inline]
//   fn is_set(&self) -> bool {
//     N == 0
//   }
//   #[inline]
//   fn fmt_self<T: Write, I: Indentation>(
//     &self,
//     f: &mut CodeFormatter<'_, T, I>,
//   ) -> fmt::Result {
//     ast::join_fmt(self.iter(), f, |s, ff| write!(ff, "{s}"), ", ")
//   }
// }

impl ComplexAttri for super::items::IdVector {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ComplexParseRes<'a, Self> {
    ast::parser::complex2(
      i,
      &mut scope.line_num,
      ast::parser::parse_usize,
      ast::parser::float_vec,
      |id, vec| Self { id, vec },
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_int(self.id)?;
    write!(f, ", \\\n{}", f.indentation())?;
    ast::join_fmt(
      self.vec.iter(),
      f,
      |float, ff| ff.write_float(float.into_inner()),
      ", ",
    )
  }
}

impl ComplexAttri for Vec<NotNan<f64>> {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ComplexParseRes<'a, Self> {
    ast::parser::complex1(i, &mut scope.line_num, ast::parser::float_vec)
  }
  #[inline]
  fn is_set(&self) -> bool {
    !self.is_empty()
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    ast::join_fmt(self.iter(), f, |float, ff| ff.write_float(float.into_inner()), ", ")
  }
}

impl ComplexAttri for ArcStr {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ComplexParseRes<'a, Self> {
    ast::parser::complex1(i, &mut scope.line_num, ast::parser::parse_arcstr)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    if is_word(self) {
      write!(f, "{self}")
    } else {
      write!(f, "\"{self}\"")
    }
  }
}
impl ComplexAttri for NotNan<f64> {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ComplexParseRes<'a, Self> {
    ast::parser::complex1(i, &mut scope.line_num, ast::parser::parse_float)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_float(self.into_inner())
  }
}
impl ComplexAttri for [NotNan<f64>; 2] {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ComplexParseRes<'a, Self> {
    ast::parser::complex2(
      i,
      &mut scope.line_num,
      ast::parser::parse_float,
      ast::parser::parse_float,
      |v1, v2| [v1, v2],
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_float(self[0].into_inner())?;
    f.write_str(", ")?;
    f.write_float(self[1].into_inner())
  }
}
impl ComplexAttri for [ArcStr; 2] {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ComplexParseRes<'a, Self> {
    ast::parser::complex2(
      i,
      &mut scope.line_num,
      ast::parser::parse_arcstr,
      ast::parser::parse_arcstr,
      |v1, v2| [v1, v2],
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_str(&self[0])?;
    f.write_str(", ")?;
    f.write_str(&self[1])
  }
}
impl ComplexAttri for Vec<ArcStr> {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ComplexParseRes<'a, Self> {
    match ast::parser::complex1(i, &mut scope.line_num, ast::parser::complex_words) {
      Ok((_i, Ok(vec))) => Ok((
        _i,
        Ok(
          vec
            .into_iter()
            .map(|(n, s)| {
              scope.line_num += n;
              ArcStr::from(s)
            })
            .collect(),
        ),
      )),
      Ok((_, Err(_))) => unreachable!(),
      Err(e) => Err(e),
    }
  }
  #[inline]
  fn is_set(&self) -> bool {
    !self.is_empty()
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    join_fmt_no_quote(
      self.iter(),
      f,
      |s, ff| if is_word(s) { write!(ff, "{s}") } else { write!(ff, "\"{s}\"") },
      ", ",
    )
  }
}
impl ComplexAttri for (NotNan<f64>, NotNan<f64>, ArcStr) {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ComplexParseRes<'a, Self> {
    ast::parser::complex3(
      i,
      &mut scope.line_num,
      ast::parser::parse_float,
      ast::parser::parse_float,
      ast::parser::parse_arcstr,
      |v1, v2, v3| (v1, v2, v3),
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_float(self.0.into_inner())?;
    f.write_str(", ")?;
    f.write_float(self.1.into_inner())?;
    f.write_str(", ")?;
    f.write_str(&self.2)
  }
}
impl ComplexAttri for (usize, NotNan<f64>) {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ComplexParseRes<'a, Self> {
    ast::parser::complex2(
      i,
      &mut scope.line_num,
      ast::parser::parse_usize,
      ast::parser::parse_float,
      |v1, v2| (v1, v2),
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_int(self.0)?;
    f.write_str(", ")?;
    f.write_float(self.1.into_inner())
  }
}

impl ComplexAttri for (NotNan<f64>, NotNan<f64>) {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ComplexParseRes<'a, Self> {
    ast::parser::complex2(
      i,
      &mut scope.line_num,
      ast::parser::parse_float,
      ast::parser::parse_float,
      |v1, v2| (v1, v2),
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_float(self.0.into_inner())?;
    f.write_str(", ")?;
    f.write_float(self.1.into_inner())
  }
}

impl fmt::Display for Formula {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(f)
  }
}
impl SimpleAttri for Formula {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    #[inline]
    fn f(i: &str) -> nom::IResult<&str, Formula, nom::error::Error<&str>> {
      nom::combinator::map(ast::parser::formula, |s| Formula(ArcStr::from(s)))(i)
    }
    ast::parser::simple_custom(i, &mut scope.line_num, f)
  }
}
