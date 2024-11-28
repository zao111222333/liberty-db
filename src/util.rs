// #![allow(clippy::non_ascii_literal)]
// use prettytable::format::{FormatBuilder, LinePosition, LineSeparator, TableFormat};

// lazy_static::lazy_static! {
// /// A table with delimiters made with box characters
// ///
// /// Reference: [FORMAT_BOX_CHARS](prettytable::format::consts::FORMAT_BOX_CHARS)
// ///
// /// # Example
// /// ```text
// ///  A │ B │ !(A&B)
// /// ───┼───┼───────
// ///  1 │ Z │ X
// ///  1 │ X │ X
// ///  1 │ R │ F
// ///  1 │ 1 │ 0
// ///  1 │ 0 │ 1
// ///  1 │ F │ R
// /// ```
// pub static ref FORMAT_NO_BORDER_BOX_CHARS: TableFormat = FormatBuilder::new()
//     .column_separator('│')
//     .separators(&[LinePosition::Title],
//       LineSeparator::new('─','┼','┼','┼'))
//     .padding(1, 1)
//     .build();
// }
