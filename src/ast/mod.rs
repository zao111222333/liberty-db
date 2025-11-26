#![expect(clippy::map_err_ignore, clippy::arithmetic_side_effects)]
//!
//! `liberty` data structre ast
//!

mod fmt;
pub mod parser;
#[cfg(feature = "lut_template")]
use crate::table::{CompactLutTemplate, TableTemple};
use crate::{
  Ctx, DefaultCtx,
  common::{f64_into_hash_ord_fn, parse_f64},
  expression::Formula,
  library::AttributeType,
  pin::BusTypeCtx,
  table::{PolyTemplate, PropagationLutTemplate},
};
use alloc::borrow::Cow;
#[cfg(feature = "lut_template")]
use alloc::sync::Arc;
use core::{cmp::Ordering, fmt::Write, marker::PhantomData, str::FromStr};
pub use fmt::{CodeFormatter, DefaultCodeFormatter, DefaultIndentation, Indentation};
use indexmap::IndexSet;
use itertools::{Itertools as _, izip};
use nom::{IResult, error::Error};
use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};
const DEFINED_COMMENT: &str = " /* user defined attribute */";

pub(crate) type RandomState = foldhash::quality::RandomState;

#[deprecated(since = "0.17.0", note = "use `LibertySet<T>`")]
pub type GroupSet<T> = LibertySet<T>;
pub type LibertySet<T> = IndexSet<T, RandomState>;
/// Just `Vec<T>`. (`LibertyVec` is a keyword for macro)
pub type LibertyVec<T> = Vec<T>;

#[expect(clippy::field_scoped_visibility_modifiers)]
#[derive(Default)]
pub(crate) struct BuilderScope<C: 'static + Ctx> {
  pub(crate) cell_extra_ctx: crate::cell::DefaultCellCtx,
  #[cfg(not(feature = "lut_template"))]
  ___p: PhantomData<C>,
  #[cfg(feature = "lut_template")]
  pub(crate) lu_table_template: HashMap<String, Arc<TableTemple<C>>, RandomState>,
  #[cfg(feature = "lut_template")]
  pub(crate) power_lut_template: HashMap<String, Arc<TableTemple<C>>, RandomState>,
  #[cfg(feature = "lut_template")]
  pub(crate) current_template: HashMap<String, Arc<TableTemple<C>>, RandomState>,
  #[cfg(feature = "lut_template")]
  pub(crate) compact_lut_template:
    HashMap<String, Arc<CompactLutTemplate<C>>, RandomState>,
  #[cfg(feature = "lut_template")]
  pub(crate) poly_template: HashMap<String, Arc<PolyTemplate<C>>, RandomState>,
  #[cfg(feature = "lut_template")]
  pub(crate) propagation_lut_template:
    HashMap<String, Arc<PropagationLutTemplate<C>>, RandomState>,
  pub(crate) voltage_map: HashMap<String, f64, RandomState>,
  pub(crate) bus_type: HashMap<String, BusTypeCtx, RandomState>,
}

pub(crate) trait ParsingBuilder<C: 'static + Ctx>: Sized {
  type Builder;
  fn build(builder: Self::Builder, scope: &mut BuilderScope<C>) -> Self;
  #[expect(clippy::type_complexity)]
  fn build_full(
    mut builder: Self::Builder,
    scope: &mut BuilderScope<C>,
    extra_before_build: Option<fn(&mut Self::Builder, &mut BuilderScope<C>)>,
    extra_after_build: Option<fn(&mut Self, &mut BuilderScope<C>)>,
  ) -> Self {
    if let Some(before_build) = extra_before_build {
      before_build(&mut builder, scope);
    }
    let mut t = Self::build(builder, scope);
    if let Some(after_build) = extra_after_build {
      after_build(&mut t, scope);
    }
    t
  }
}

pub(crate) trait ParsingSet<C: 'static + Ctx, T: 'static + ParsingBuilder<C>>:
  Sized + Default
{
  type BuilderSet;
  fn push_set(builder: &mut Self::BuilderSet, item: T::Builder, scope: &ParseScope<'_>);
  #[expect(clippy::type_complexity)]
  fn build_set(
    builder: Self::BuilderSet,
    scope: &mut BuilderScope<C>,
    before_build: Option<fn(&mut T::Builder, &mut BuilderScope<C>)>,
    after_build: Option<fn(&mut T, &mut BuilderScope<C>)>,
  ) -> Self;
  fn iter_set<K: 'static + Copy + core::fmt::Display>(
    &self,
    key: K,
  ) -> impl '_ + Iterator<Item = (K, &T)>;
}

impl<C: 'static + Ctx, T: 'static + Sized + Default + ParsingBuilder<C>> ParsingSet<C, T>
  for T
{
  type BuilderSet = T::Builder;
  fn push_set(builder: &mut Self::BuilderSet, item: T::Builder, _: &ParseScope<'_>) {
    *builder = item;
  }
  fn build_set(
    builder: Self::BuilderSet,
    scope: &mut BuilderScope<C>,
    before_build: Option<fn(&mut T::Builder, &mut BuilderScope<C>)>,
    after_build: Option<fn(&mut T, &mut BuilderScope<C>)>,
  ) -> Self {
    T::build_full(builder, scope, before_build, after_build)
  }
  fn iter_set<K: 'static + Copy + core::fmt::Display>(
    &self,
    key: K,
  ) -> impl '_ + Iterator<Item = (K, &T)> {
    core::iter::once((key, self))
  }
}

impl<C: 'static + Ctx, T: 'static + Sized + ParsingBuilder<C>> ParsingSet<C, T>
  for Option<T>
{
  type BuilderSet = Option<T::Builder>;
  fn push_set(builder: &mut Self::BuilderSet, item: T::Builder, scope: &ParseScope<'_>) {
    if builder.is_some() {
      let e = IdError::RepeatAttri;
      crate::error!("{} error={}", scope.loc, e);
    }
    *builder = Some(item);
  }
  fn build_set(
    builder: Self::BuilderSet,
    scope: &mut BuilderScope<C>,
    before_build: Option<fn(&mut T::Builder, &mut BuilderScope<C>)>,
    after_build: Option<fn(&mut T, &mut BuilderScope<C>)>,
  ) -> Self {
    builder.map(|b| T::build_full(b, scope, before_build, after_build))
  }
  fn iter_set<K: 'static + Copy + core::fmt::Display>(
    &self,
    key: K,
  ) -> impl '_ + Iterator<Item = (K, &T)> {
    self.iter().map(move |t| (key, t))
  }
}

impl<C: 'static + Ctx, T: 'static + Sized + ParsingBuilder<C>> ParsingSet<C, T>
  for Vec<T>
{
  type BuilderSet = Vec<T::Builder>;
  fn push_set(builder: &mut Self::BuilderSet, item: T::Builder, _: &ParseScope<'_>) {
    builder.push(item);
  }
  fn build_set(
    builder: Self::BuilderSet,
    scope: &mut BuilderScope<C>,
    before_build: Option<fn(&mut T::Builder, &mut BuilderScope<C>)>,
    after_build: Option<fn(&mut T, &mut BuilderScope<C>)>,
  ) -> Self {
    builder
      .into_iter()
      .map(|b| T::build_full(b, scope, before_build, after_build))
      .collect()
  }
  fn iter_set<K: 'static + Copy + core::fmt::Display>(
    &self,
    key: K,
  ) -> impl '_ + Iterator<Item = (K, &T)> {
    self.iter().map(move |t| (key, t))
  }
}

impl<
  C: 'static + Ctx,
  T: 'static + Sized + ParsingBuilder<C> + core::hash::Hash + Eq + Ord,
> ParsingSet<C, T> for LibertySet<T>
{
  type BuilderSet = Vec<T::Builder>;
  fn push_set(builder: &mut Self::BuilderSet, item: T::Builder, _: &ParseScope<'_>) {
    builder.push(item);
  }
  fn build_set(
    builder: Self::BuilderSet,
    scope: &mut BuilderScope<C>,
    before_build: Option<fn(&mut T::Builder, &mut BuilderScope<C>)>,
    after_build: Option<fn(&mut T, &mut BuilderScope<C>)>,
  ) -> Self {
    let mut set: Self = builder
      .into_iter()
      .map(|b| T::build_full(b, scope, before_build, after_build))
      .collect();
    set.sort_unstable();
    set
  }
  fn iter_set<K: 'static + Copy + core::fmt::Display>(
    &self,
    key: K,
  ) -> impl '_ + Iterator<Item = (K, &T)> {
    self.iter().map(move |t| (key, t))
  }
}

pub(crate) trait DynamicKey<C: 'static + Ctx> {
  type Id: core::hash::Hash + Eq;
  type T: 'static + ParsingBuilder<C> + Sized;
  type Set;
  type KeyFmt: 'static + core::fmt::Display;
  fn key2id(key: &str) -> Option<Self::Id>;
  #[expect(clippy::type_complexity)]
  fn build_set(
    builder: DynamicKeyBuilderSet<C, Self>,
    scope: &mut BuilderScope<C>,
    before_build: Option<
      fn(&mut <Self::T as ParsingBuilder<C>>::Builder, &mut BuilderScope<C>),
    >,
    after_build: Option<fn(&mut Self::T, &mut BuilderScope<C>)>,
  ) -> Self::Set;
  fn push_set(
    builder: &mut DynamicKeyBuilderSet<C, Self>,
    id: Self::Id,
    item: <Self::T as ParsingBuilder<C>>::Builder,
    scope: &ParseScope<'_>,
  ) {
    _ = scope;
    if builder.insert(id, item).is_some() {
      crate::error!("{} error={}", scope.loc, IdError::RepeatAttri);
    }
  }
  fn iter_set(set: &Self::Set) -> impl '_ + Iterator<Item = (Self::KeyFmt, &Self::T)>;
}
#[expect(type_alias_bounds)]
pub(crate) type DynamicKeyBuilderSet<C: 'static + Ctx, N: DynamicKey<C>> =
  HashMap<N::Id, <N::T as ParsingBuilder<C>>::Builder, RandomState>;

macro_rules! impl_self_builder {
  ($t:ty) => {
    impl<C: 'static + crate::Ctx> crate::ast::ParsingBuilder<C> for $t {
      type Builder = Self;
      #[inline]
      fn build(builder: Self::Builder, _scope: &mut crate::ast::BuilderScope<C>) -> Self {
        builder
      }
    }
  };
}
macro_rules! impl_simple {
  ($t:ty) => {
    impl<C: 'static + crate::Ctx> crate::ast::SimpleAttri<C> for $t {
      #[inline]
      fn nom_parse<'a>(
        i: &'a str,
        scope: &mut crate::ast::ParseScope<'_>,
      ) -> crate::ast::SimpleParseRes<'a, Self> {
        crate::ast::nom_parse_from_str::<C, _>(i, scope)
      }
      #[inline]
      fn fmt_self<T: core::fmt::Write, I: crate::ast::Indentation>(
        &self,
        f: &mut crate::ast::CodeFormatter<'_, T, I>,
      ) -> core::fmt::Result {
        use core::fmt::Write as _;
        write!(f, "{self}")
      }
    }
  };
}

pub(crate) use impl_self_builder;
pub(crate) use impl_simple;

/// Wrapper for simple attribute
pub type SimpleWrapper = String;
/// Wrapper for complex attribute
#[expect(clippy::field_scoped_visibility_modifiers)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ComplexWrapper(pub(crate) Vec<String>);
impl ComplexWrapper {
  #[expect(clippy::arithmetic_side_effects)]
  fn collect(vec: Vec<(usize, &str)>, scope: &mut ParseScope<'_>) -> Self {
    Self(
      vec
        .into_iter()
        .map(|(n, s)| {
          scope.loc.line_num += n;
          String::from(s)
        })
        .collect(),
    )
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
  pub title: Vec<String>,
  /// `attri_map`
  pub attri_map: Attributes,
}
impl Ord for GroupWrapper {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.title.cmp(&other.title)
  }
}
impl PartialOrd for GroupWrapper {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
/// type for Undefined `Attributes`
pub type Attributes = HashMap<String, AttriValues, foldhash::fast::FixedState>;
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

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SimpleDefined {
  /// Boolean `Simple`
  Boolean(Vec<Result<bool, String>>),
  /// string `Simple`
  String(Vec<String>),
  /// integer `Simple`
  Integer(Vec<Result<isize, String>>),
  /// float `Simple`
  Float(Vec<Result<f64, String>>),
}
impl PartialEq for SimpleDefined {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
      (Self::String(l0), Self::String(r0)) => l0 == r0,
      (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
      (Self::Float(l0), Self::Float(r0)) => {
        l0.len() == r0.len()
          && izip!(l0, r0).all(|lr| match lr {
            (Ok(l), Ok(r)) => f64_into_hash_ord_fn(l) == f64_into_hash_ord_fn(r),
            (Err(l), Err(r)) => l == r,
            _ => false,
          })
      }
      _ => false,
    }
  }
}
impl Eq for SimpleDefined {}
impl PartialOrd for SimpleDefined {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
impl Ord for SimpleDefined {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Self::Boolean(l), Self::Boolean(r)) => l.cmp(r),
      (Self::String(l), Self::String(r)) => l.cmp(r),
      (Self::Integer(l), Self::Integer(r)) => l.cmp(r),
      (Self::Float(l), Self::Float(r)) => match l.len().cmp(&r.len()) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => l
          .iter()
          .map(|res| match res {
            Ok(f) => Ok(f64_into_hash_ord_fn(f)),
            Err(s) => Err(s),
          })
          .cmp(r.iter().map(|res| match res {
            Ok(f) => Ok(f64_into_hash_ord_fn(f)),
            Err(s) => Err(s),
          })),
      },
      (Self::Boolean(_), _)
      | (_, Self::Float(_))
      | (Self::String(_), Self::Integer(_)) => Ordering::Less,
      (Self::Float(_), _)
      | (_, Self::Boolean(_))
      | (Self::Integer(_), Self::String(_)) => Ordering::Greater,
    }
  }
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
    key: &String,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    v.iter().try_for_each(|u| Format::liberty(u, key, f))
  }
  #[expect(clippy::all)]
  #[inline]
  fn fmt2<T: Write, I: Indentation, U: SimpleAttri<DefaultCtx>>(
    v: &Vec<Result<U, String>>,
    key: &String,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    v.iter().try_for_each(|res_u| {
      match res_u {
        Ok(u) => SimpleAttri::fmt_liberty(u, key, f),
        Err(u) => SimpleAttri::<DefaultCtx>::fmt_liberty(u, key, f),
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
  undefined: String,
) {
  if let Some(AttriValues::Simple(SimpleDefined::String(v))) = attri_map.get_mut(key) {
    v.push(undefined);
  } else {
    _ = attri_map.insert(
      String::from(key),
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
    _ = attri_map.insert(String::from(key), AttriValues::Complex(vec![undefined]));
  }
}
#[expect(clippy::too_many_lines)]
#[inline]
pub(crate) fn attributs_set_undefined_attri(
  attri_map: &mut Attributes,
  key: &str,
  group_name: &str,
  scope: &ParseScope<'_>,
  undefined: UndefinedAttriValue,
) {
  match scope.define_map.get(&(group_name.to_owned(), key.to_owned())) {
    None => {
      crate::warn!("{} undefined {}", scope.loc, key);
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
            crate::error!(
              "{} Key={key}, the old undefined attribute do NOT meet new one",
              scope.loc
            );
          }
        }
      } else {
        _ = attri_map.insert(
          String::from(key),
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
                crate::error!(
                  "{} Key={key}, the old attribute do NOT meet new one",
                  scope.loc
                );
              }
            }
            AttributeType::String => {
              if let AttriValues::Simple(SimpleDefined::String(v)) = value {
                v.push(u);
              } else {
                crate::error!(
                  "{} Key={key}, the old attribute do NOT meet new one",
                  scope.loc
                );
              }
            }
            AttributeType::Integer => {
              if let AttriValues::Simple(SimpleDefined::Integer(v)) = value {
                v.push(lexical_core::parse(u.as_bytes()).map_or(Err(u), Ok));
              } else {
                crate::error!(
                  "{} Key={key}, the old attribute do NOT meet new one",
                  scope.loc
                );
              }
            }
            AttributeType::Float => {
              if let AttriValues::Simple(SimpleDefined::Float(v)) = value {
                v.push(parse_f64(&u).map_or(Err(u), Ok));
              } else {
                crate::error!(
                  "{} Key={key}, the old attribute do NOT meet new one",
                  scope.loc
                );
              }
            }
          }
        } else {
          _ = attri_map.insert(
            String::from(key),
            match simple_type {
              AttributeType::Boolean => {
                AttriValues::Simple(SimpleDefined::Boolean(vec![
                  u.parse().map_or(Err(u), Ok),
                ]))
              }
              AttributeType::String => {
                AttriValues::Simple(SimpleDefined::String(vec![u]))
              }
              AttributeType::Integer => {
                AttriValues::Simple(SimpleDefined::Integer(vec![
                  lexical_core::parse(u.as_bytes()).map_or(Err(u), Ok),
                ]))
              }
              AttributeType::Float => AttriValues::Simple(SimpleDefined::Float(vec![
                parse_f64(&u).map_or(Err(u), Ok),
              ])),
            },
          );
        }
      } else {
        crate::error!("{} Key={key}, `defined` got wrong type", scope.loc);
      }
    }
    Some(DefinedType::Group) => {
      if let UndefinedAttriValue::Group(u) = undefined {
        if let Some(value) = attri_map.get_mut(key) {
          if let AttriValues::Group(v) = value {
            v.push(u);
          } else {
            crate::error!(
              "{} Key={key}, the old attribute do NOT meet new one",
              scope.loc
            );
          }
        } else {
          _ = attri_map.insert(String::from(key), AttriValues::Group(vec![u]));
        }
      } else {
        crate::error!("{} Key={key}, `defined_group` got wrong type", scope.loc);
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

#[expect(clippy::field_scoped_visibility_modifiers)]
#[derive(Debug, Default)]
pub(crate) struct ParseScope<'a> {
  pub(crate) loc: ParseLoc<'a>,
  pub(crate) include_files: IndexSet<PathBuf>,
  pub(crate) define_map: HashMap<(String, String), DefinedType, RandomState>,
  pub(crate) variables: HashMap<String, Formula, RandomState>,
}

#[derive(Debug, Default)]
pub struct ParseLoc<'a> {
  pub filename: Option<Cow<'a, Path>>,
  pub line_num: usize,
}

impl core::fmt::Display for ParseLoc<'_> {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if let Some(p) = &self.filename {
      write!(f, "File \"{}\", ", p.display())?;
    }
    write!(f, "line {}.", self.line_num)
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

pub(crate) type SimpleParseRes<'a, T> =
  IResult<&'a str, Result<T, String>, Error<&'a str>>;
pub(crate) type ComplexParseRes<'a, T> =
  IResult<&'a str, Result<T, (ComplexParseError, ComplexWrapper)>, Error<&'a str>>;
#[inline]
pub(crate) fn nom_parse_from<
  'a,
  C: 'static + Ctx,
  T: SimpleAttri<C>,
  E,
  F: Fn(&str) -> Result<T, E>,
>(
  i: &'a str,
  scope: &mut ParseScope<'_>,
  parse_fn: F,
) -> SimpleParseRes<'a, T> {
  let (input, s) = parser::simple(i, &mut scope.loc.line_num)
    .or(parser::complex_single(i, &mut scope.loc.line_num))?;
  parse_fn(s).map_or(Ok((input, Err(String::from(s)))), |simple| Ok((input, Ok(simple))))
}

#[inline]
pub(crate) fn nom_parse_from_str<'a, C: 'static + Ctx, T: SimpleAttri<C> + FromStr>(
  i: &'a str,
  scope: &mut ParseScope<'_>,
) -> SimpleParseRes<'a, T> {
  nom_parse_from(i, scope, T::from_str)
}

/// Simple Attribute in Liberty
pub(crate) trait SimpleAttri<C: 'static + Ctx>: Sized + ParsingBuilder<C> {
  /// `nom_parse`, auto implement
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope<'_>,
  ) -> SimpleParseRes<'a, Self::Builder>;
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
  fn fmt_liberty<T: Write, I: Indentation, K: core::fmt::Display>(
    &self,
    key: K,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    if self.is_set() {
      f.write_new_line_indentation()?;
      write!(f, "{key} : ")?;
      self.fmt_self(f)?;
      write!(f, ";")
    } else {
      Ok(())
    }
  }
}

/// `ComplexParseError`
#[derive(thiserror::Error, Debug)]
pub(crate) enum ComplexParseError {
  /// `ParseFloatError`
  #[error("{0}")]
  Float(#[from] fast_float2::Error),
  /// `ParseIntError`
  #[error("{0}")]
  Int(#[from] lexical_core::Error),
  /// complex length mismatch
  #[error("complex length mismatch")]
  LengthDismatch,
  /// other error
  #[error("other")]
  Other,
  /// unsurpport word
  #[error("unsurpport word")]
  UnsupportedWord,
}

impl From<strum::ParseError> for ComplexParseError {
  #[inline]
  fn from(_: strum::ParseError) -> Self {
    Self::UnsupportedWord
  }
}

#[inline]
pub(crate) fn join_fmt<
  'a,
  T: Sized + 'a,
  I: Iterator<Item = T>,
  W: Write,
  F: FnMut(T, &mut W) -> core::fmt::Result,
  S: FnMut(&mut W) -> core::fmt::Result,
>(
  iter: I,
  f: &mut W,
  func: F,
  sep: S,
) -> core::fmt::Result {
  write!(f, "\"")?;
  join_fmt_no_quote(iter, f, func, sep)?;
  write!(f, "\"")
}

#[inline]
pub(crate) fn join_fmt_no_quote<
  'a,
  T: Sized + 'a,
  I: Iterator<Item = T>,
  W: Write,
  F: FnMut(T, &mut W) -> core::fmt::Result,
  S: FnMut(&mut W) -> core::fmt::Result,
>(
  mut iter: I,
  f: &mut W,
  mut func: F,
  mut sep: S,
) -> core::fmt::Result {
  if let Some(first) = iter.next() {
    func(first, f)?;
    while let Some(t) = iter.next() {
      sep(f)?;
      func(t, f)?;
    }
  }
  Ok(())
}

/// Complex Attribute in Liberty
pub(crate) trait ComplexAttri<C: 'static + Ctx>:
  Sized + ParsingBuilder<C>
{
  /// basic `parser`
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    scope: &mut ParseScope<'_>,
  ) -> Result<Self::Builder, ComplexParseError>;
  /// `nom_parse`, auto implement
  #[expect(clippy::arithmetic_side_effects)]
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope<'_>,
  ) -> ComplexParseRes<'a, Self::Builder> {
    let (input, vec) = parser::complex(i, &mut scope.loc.line_num)?;
    let mut line_num = 0;
    let res = Self::parse(
      vec.iter().map(|(n, s)| {
        line_num += n;
        s
      }),
      scope,
    );
    scope.loc.line_num += line_num;
    match res {
      Ok(s) => Ok((input, Ok(s))),
      Err(e) => Ok((input, Err((e, ComplexWrapper::collect(vec, scope))))),
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
  fn fmt_liberty<T: Write, I: Indentation, K: core::fmt::Display>(
    &self,
    key: K,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    if self.is_set() {
      f.write_new_line_indentation()?;
      write!(f, "{key} (")?;
      // f.indent();
      self.fmt_self(f)?;
      // f.dedent();
      write!(f, ");")
    } else {
      Ok(())
    }
  }
}
/// Group Functions
pub(crate) trait GroupFn<C: 'static + Ctx>: ParsingBuilder<C> {
  /// `before_build` call back
  #[inline]
  #[expect(unused_variables)]
  fn before_build(builder: &mut Self::Builder, scope: &mut BuilderScope<C>) {}
  /// `after_build` call back
  #[inline]
  #[expect(unused_variables)]
  fn after_build(&mut self, scope: &mut BuilderScope<C>) {}
}
/// Export Group APIs
#[expect(private_bounds)]
pub trait Group<C: 'static + Ctx>: Sized + GroupAttri<C> {
  /// `test_wrapper`
  #[inline]
  fn display(&self) -> GroupDisplay<'_, C, Self> {
    GroupDisplay { inner: self, name: None, ___p: PhantomData }
  }
  /// `test_wrapper`
  #[inline]
  fn display_name<'a>(&'a self, name: &'a str) -> GroupDisplay<'a, C, Self> {
    GroupDisplay { inner: self, name: Some(name), ___p: PhantomData }
  }
}
/// `GroupAttri`, internal Group APIs
pub(crate) trait GroupAttri<C: 'static + Ctx>:
  Sized + ParsingBuilder<C, Builder: Default>
{
  /// `nom_parse`, will be implemented by macros
  fn nom_parse<'a, const IS_INCLUDED: bool>(
    builder: &mut Self::Builder,
    i: &'a str,
    group_name: &str,
    scope: &mut ParseScope<'_>,
  ) -> IResult<&'a str, Result<(), IdError>, Error<&'a str>>;
  fn include_file<'a>(
    builder: &mut Self::Builder,
    mut i: &'a str,
    group_name: &str,
    scope: &mut ParseScope<'_>,
  ) -> IResult<&'a str, Result<(), IdError>, Error<&'a str>> {
    let (_i, filename_s) = parser::complex_single(i, &mut scope.loc.line_num)?;
    i = _i;
    let filename = if Path::new(filename_s).is_absolute() {
      PathBuf::from(filename_s)
    } else if let Some(base_file) = &scope.loc.filename {
      if let Some(root) = base_file.parent() {
        root.join(filename_s)
      } else {
        return Err(nom::Err::Error(Error::new(
          "include_file: unable to open file",
          nom::error::ErrorKind::Eof,
        )));
      }
    } else {
      PathBuf::from(filename_s)
    };
    let s = std::fs::read_to_string(&filename).map_err(|_| {
      nom::Err::Error(Error::new(
        "include_file: unable to open file",
        nom::error::ErrorKind::Eof,
      ))
    })?;
    crate::info!("include file: {}", filename.display());
    if !scope.include_files.insert(filename.clone()) {
      return Err(nom::Err::Error(Error::new(
        "include_file: loop include",
        nom::error::ErrorKind::Eof,
      )));
    }
    let old_line_num = scope.loc.line_num;
    let old_filename = scope.loc.filename.take();
    scope.loc.filename = Some(filename.into());
    let input1 = match parser::comment_space_newline(&s) {
      Ok((input1, n)) => {
        scope.loc.line_num = 1 + n;
        input1
      }
      Err(_) => {
        return Err(nom::Err::Error(Error::new(
          "include_file: unable to parse file",
          nom::error::ErrorKind::Eof,
        )));
      }
    };
    _ = Self::nom_parse::<true>(builder, input1, group_name, scope).map_err(|e| {
      crate::error!("{e}");
      nom::Err::Error(Error::new(
        "include_file: unable to parse file",
        nom::error::ErrorKind::Eof,
      ))
    })?;
    _ = scope.include_files.pop();
    scope.loc.line_num = old_line_num;
    scope.loc.filename = old_filename;
    Ok((i, Ok(())))
  }
  /// `fmt_liberty`
  fn fmt_liberty<T: Write, I: Indentation, K: core::fmt::Display>(
    &self,
    key: K,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result;
}

/// Error for parser Group Index
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum IdError {
  /// TitleLenMismatch(want,got,title)
  #[error("title length dismatch (want={want},got={got}), title={title:?}")]
  LengthDismatch { want: usize, got: usize, title: Vec<String> },
  /// replace same id
  #[error("replace same id")]
  RepeatIdx,
  /// replace same attribute
  #[error("replace same attribute")]
  RepeatAttri,
  /// Int Error
  #[error("{0}")]
  Int(#[from] lexical_core::Error),
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
      title: v.into_iter().map(String::from).collect(),
    }
  }
}

/// If more than one `#[liberty(name)]`,
/// need to impl `NamedGroup` manually
pub(crate) trait NamedGroup<C: 'static + Ctx>: GroupAttri<C> {
  /// parse name from `v: &[&str]` and then set self
  fn parse_set_name(builder: &mut Self::Builder, v: Vec<&str>) -> Result<(), IdError>;
  /// `fmt_liberty`
  fn fmt_name<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result;
}

/// `NameAttri`
pub(crate) trait NameAttri: Sized {
  /// basic parser
  fn parse(v: Vec<&str>) -> Result<Self, IdError>;
  /// name `to_vec`
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result;
}

pub(crate) trait FlattenNameAttri: Sized {
  /// basic parser
  fn parse(v: Vec<&str>) -> Result<Vec<Self>, IdError>;
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result;
  fn pretend_group(parsed: Vec<Self>) -> Self;
  fn ungroup(&self) -> Option<impl Iterator<Item = Self>>;
}

/// Error for parser
#[derive(Debug, thiserror::Error)]
pub enum ParserError<'a> {
  /// TitleLenMismatch(want,got,title)
  #[error("{0} {1}")]
  IdError(ParseLoc<'a>, IdError),
  /// replace same id
  #[error("{0} {1}")]
  NomError(ParseLoc<'a>, String),
  #[error("File {0:?}. {1}")]
  IO(PathBuf, std::io::Error),
  /// something else
  #[error("{0} {1}")]
  Other(ParseLoc<'a>, String),
}

impl<'a> ParserError<'a> {
  #[inline]
  pub(crate) fn nom(loc: ParseLoc<'a>, e: nom::Err<Error<&str>>) -> Self {
    Self::NomError(
      loc,
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
pub struct GroupDisplay<'a, C: 'static + Ctx, G> {
  pub name: Option<&'a str>,
  pub inner: &'a G,
  ___p: PhantomData<C>,
}

impl<C: 'static + Ctx, G: GroupAttri<C>> core::fmt::Display for GroupDisplay<'_, C, G> {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ff = DefaultCodeFormatter::new(f);
    self.inner.fmt_liberty(
      self.name.unwrap_or(
        core::any::type_name::<G>()
          .replace(&format!("<{}>", core::any::type_name::<C>()), "")
          .as_str(),
      ),
      &mut ff,
    )
  }
}

#[cfg(test)]
#[inline]
pub(crate) fn test_parse<G: GroupAttri<DefaultCtx> + Group<DefaultCtx>>(
  input: &str,
) -> G {
  let mut scope = ParseScope::default();
  let mut builder = G::Builder::default();
  match G::nom_parse::<false>(&mut builder, input, "", &mut scope) {
    Ok((_, Ok(_))) => {}
    Ok((_, Err(e))) => panic!("{e}"),
    Err(e) => panic!("{e}"),
  }
  let mut builder_scope = BuilderScope::<DefaultCtx>::default();
  let g = <G as ParsingBuilder<DefaultCtx>>::build(builder, &mut builder_scope);
  println!("{}", g.display());
  g
}

#[cfg(test)]
#[inline]
pub(crate) fn test_parse_fmt<G: GroupAttri<DefaultCtx> + Group<DefaultCtx>>(
  input: &str,
  fmt_want: &str,
) -> G {
  let mut scope = ParseScope::default();
  let mut builder = G::Builder::default();
  match G::nom_parse::<false>(&mut builder, input, "", &mut scope) {
    Ok((_, Ok(_))) => {}
    Ok((_, Err(e))) => panic!("{e}"),
    Err(e) => panic!("{e}"),
  }
  let mut builder_scope = BuilderScope::<DefaultCtx>::default();
  let g = <G as ParsingBuilder<DefaultCtx>>::build(builder, &mut builder_scope);
  let fmt_str = g.display().to_string();
  println!("{fmt_str}");
  dev_utils::text_diff(fmt_want, fmt_str.as_str());
  g
}

#[cfg(test)]
#[inline]
pub(crate) fn test_parse_fmt_included<G: GroupAttri<DefaultCtx> + Group<DefaultCtx>>(
  input: &str,
  fmt_want: &str,
) -> G {
  let mut scope = ParseScope::default();
  let mut builder = G::Builder::default();
  match G::nom_parse::<true>(&mut builder, input, "", &mut scope) {
    Ok((_, Ok(_))) => {}
    Ok((_, Err(e))) => panic!("{e}"),
    Err(e) => panic!("{e}"),
  }
  let mut builder_scope = BuilderScope::<DefaultCtx>::default();
  let g = <G as ParsingBuilder<DefaultCtx>>::build(builder, &mut builder_scope);
  let fmt_str = g.display().to_string();
  println!("{fmt_str}");
  dev_utils::text_diff(fmt_want, fmt_str.as_str());
  g
}

#[cfg(test)]
#[inline]
pub(crate) fn test_parse_fmt_variables<G: GroupAttri<DefaultCtx> + Group<DefaultCtx>>(
  variable: &[&str],
  input: &str,
  fmt_want: &str,
) -> G {
  use biodivine_lib_bdd::BddVariableSet;

  let mut scope = ParseScope::default();
  let mut builder = G::Builder::default();
  match G::nom_parse::<false>(&mut builder, input, "", &mut scope) {
    Ok((_, Ok(_))) => {}
    Ok((_, Err(e))) => panic!("{e}"),
    Err(e) => panic!("{e}"),
  }
  let mut builder_scope = BuilderScope::<DefaultCtx>::default();
  builder_scope.cell_extra_ctx.logic_variables = BddVariableSet::new(variable);
  let g = <G as ParsingBuilder<DefaultCtx>>::build(builder, &mut builder_scope);
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
pub(crate) fn is_word(s: &str) -> bool {
  !s.is_empty() && s.chars().all(parser::char_in_word)
}

#[doc(hidden)]
#[expect(clippy::field_scoped_visibility_modifiers)]
#[derive(Default, Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GroupComments(pub(crate) HashMap<u64, String, mut_set::NoHashBuildHasher>);

#[inline]
pub(crate) fn fmt_comment_liberty<T: Write, I: Indentation>(
  comments: Option<&String>,
  f: &mut CodeFormatter<'_, T, I>,
) -> core::fmt::Result {
  if let Some(s) = comments {
    if s.is_empty() {
      Ok(())
    } else {
      f.write_new_line_indentation()?;
      if s.contains('\n') {
        write!(f, "/* ")?;
        join_fmt_no_quote(
          s.split('\n'),
          f,
          |line, ff| write!(ff, "{line}"),
          |ff| {
            ff.write_new_line_indentation()?;
            write!(ff, "** ")
          },
        )?;
        write!(f, " */")
      } else {
        write!(f, "* {s}")
      }
    }
  } else {
    Ok(())
  }
}

#[inline]
pub(crate) fn fmt_library_beginning<T: Write, I: Indentation>(
  comments: Option<&String>,
  f: &mut CodeFormatter<'_, T, I>,
) -> core::fmt::Result {
  if let Some(s) = comments {
    if s.is_empty() {
      Ok(())
    } else if s.contains('\n') {
      write!(f, "/** ")?;
      join_fmt_no_quote(
        s.split('\n'),
        f,
        |line, ff| write!(ff, "{line}"),
        |ff| write!(ff, "\n*** "),
      )?;
      write!(f, " */")
    } else {
      write!(f, "** {s}")
    }
  } else {
    write!(f, "** Generated by {} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"),)
  }
}
impl Format for SimpleWrapper {
  #[inline]
  fn liberty<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    SimpleAttri::<DefaultCtx>::fmt_liberty(self, key, f)
      .and(write!(f, "{DEFINED_COMMENT}"))
  }
}

impl Format for ComplexWrapper {
  #[inline]
  fn liberty<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    ComplexAttri::<DefaultCtx>::fmt_liberty(&self.0, key, f)
      .and(write!(f, "{DEFINED_COMMENT}"))
  }
}

impl Format for GroupWrapper {
  #[inline]
  fn liberty<T: Write, I: Indentation>(
    &self,
    key: &str,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    f.write_new_line_indentation()?;
    write!(f, "{key} (")?;
    join_fmt_no_quote(
      self.title.iter(),
      f,
      |s, ff| if is_word(s) { write!(ff, "{s}") } else { write!(ff, "\"{s}\"") },
      |ff| write!(ff, ", "),
    )?;
    write!(f, ") {{ {DEFINED_COMMENT}")?;
    f.indent();
    attributs_fmt_liberty(&self.attri_map, f)?;
    f.dedent();
    f.write_new_line_indentation()?;
    write!(f, "}}")
  }
}
