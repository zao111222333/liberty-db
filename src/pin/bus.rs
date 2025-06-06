use crate::{Ctx, ast};
use core::fmt;

/// If your library contains bused pins, you must define type groups and define the structural
/// constraints of each bus type in the library. The type group is defined at the library group
/// level, as shown here:
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=91.7+136.10&end=91.9+136.25
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
/// ``` text
/// Example 17 Bused Pins
/// library (ExamBus) {
///   type (bus4) { /* bus name */
///     bit_width : 4 ; /* number of bused pins */
///     ...
///     ...
///   }
///   cell (bused cell) {
///     ...
///     bus (A) {
///       ...
///       bus_type : bus4 ; /* bus name */
///       ...
///     }
///   }
/// }
/// ```
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Pin: serde::Serialize + serde::de::DeserializeOwned")]
pub struct BusType<C: Ctx> {
  #[id(borrow = str)]
  #[liberty(name)]
  pub name: String,
  /// group comments
  #[liberty(comments)]
  comments: ast::GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Pin,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: ast::Attributes,
  /// Only the array base type is supported.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=91.27&end=91.28
  /// ">Reference</a>
  #[liberty(simple)]
  pub base_type: BaseType,
  /// Indicates the member number assigned to the most significant bit (MSB) of
  /// successive array members. The default is 0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=91.29&end=91.31
  /// ">Reference</a>
  #[liberty(simple)]
  #[liberty(default = 0)]
  pub bit_from: usize,
  /// Indicates the member number assigned to the least significant bit (LSB) of
  /// successive array members. The default is 0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=91.32&end=91.34
  /// ">Reference</a>
  #[liberty(simple)]
  #[liberty(default = 0)]
  pub bit_to: usize,
  /// Designates the number of bus members. The default is 1.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=91.35&end=91.36
  /// ">Reference</a>
  #[liberty(simple)]
  #[liberty(default = 1)]
  pub bit_width: usize,
  /// Indicates that only the bit data type is supported.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=91.37&end=91.38
  /// ">Reference</a>
  #[liberty(simple)]
  pub data_type: DataType,
  /// A true value indicates that member number assignment is from high to low. A
  /// false value indicates that member number assignment is from low to high.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=92.2&end=92.4
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub downto: Option<bool>,
}

impl<C: Ctx> ast::GroupFn<C> for BusType<C> {
  fn before_build(builder: &mut Self::Builder, scope: &mut ast::BuilderScope<C>) {
    _ = scope.bus_type.insert(
      builder.name.clone(),
      BusTypeCtx {
        base_type: builder.base_type,
        bit_from: builder.bit_from,
        bit_to: builder.bit_to,
        bit_width: builder.bit_width,
        data_type: builder.data_type,
        downto: builder.downto,
      },
    );
  }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SimpleBusType {
  pub name: String,
  pub ctx: Option<BusTypeCtx>,
}

#[derive(Debug, Clone, Copy)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusTypeCtx {
  /// Only the array base type is supported.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=91.27&end=91.28
  /// ">Reference</a>
  pub base_type: BaseType,
  /// Indicates the member number assigned to the most significant bit (MSB) of
  /// successive array members. The default is 0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=91.29&end=91.31
  /// ">Reference</a>
  pub bit_from: usize,
  /// Indicates the member number assigned to the least significant bit (LSB) of
  /// successive array members. The default is 0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=91.32&end=91.34
  /// ">Reference</a>
  pub bit_to: usize,
  /// Designates the number of bus members. The default is 1.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=91.35&end=91.36
  /// ">Reference</a>
  pub bit_width: usize,
  /// Indicates that only the bit data type is supported.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=91.37&end=91.38
  /// ">Reference</a>
  pub data_type: DataType,
  /// A true value indicates that member number assignment is from high to low. A
  /// false value indicates that member number assignment is from low to high.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=92.2&end=92.4
  /// ">Reference</a>
  pub downto: Option<bool>,
}

impl<C: Ctx> ast::ParsingBuilder<C> for SimpleBusType {
  type Builder = String;
  #[expect(clippy::renamed_function_params)]
  fn build(name: Self::Builder, scope: &mut ast::BuilderScope<C>) -> Self {
    Self { ctx: scope.bus_type.get(&name).copied(), name }
  }
}
impl<C: Ctx> ast::SimpleAttri<C> for SimpleBusType {
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ast::ParseScope,
  ) -> ast::SimpleParseRes<'a, Self::Builder> {
    ast::nom_parse_from_str::<C, _>(i, scope)
  }
  fn fmt_self<T: fmt::Write, I: ast::Indentation>(
    &self,
    f: &mut ast::CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    ast::SimpleAttri::<C>::fmt_self(&self.name, f)
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, strum::Display, strum::EnumString, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BaseType {
  #[default]
  #[strum(serialize = "array")]
  Array,
}
crate::ast::impl_self_builder!(BaseType);
crate::ast::impl_simple!(BaseType);

#[derive(Debug, Clone, Copy, Eq, PartialEq, strum::Display, strum::EnumString, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DataType {
  #[default]
  #[strum(serialize = "bit")]
  Bit,
}
crate::ast::impl_self_builder!(DataType);
crate::ast::impl_simple!(DataType);
