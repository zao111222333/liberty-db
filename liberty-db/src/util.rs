pub(crate) mod format{
    use prettytable::format::{TableFormat, FormatBuilder, LinePosition, LineSeparator};

    lazy_static::lazy_static!{
        /// A table with delimiters made with box characters
        ///
        /// Reference: [FORMAT_BOX_CHARS](prettytable::format::consts::FORMAT_BOX_CHARS)
        /// 
        /// # Example
        /// ```text
        ///  A │ B │ !(A&B)
        /// ───┼───┼───────
        ///  1 │ Z │ X
        ///  1 │ X │ X 
        ///  1 │ R │ F 
        ///  1 │ 1 │ 0 
        ///  1 │ 0 │ 1 
        ///  1 │ F │ R 
        /// ```
        pub static ref FORMAT_NO_BORDER_BOX_CHARS: TableFormat = FormatBuilder::new()
            .column_separator('│')
            .separators(&[LinePosition::Title],
            LineSeparator::new('─','┼','┼','┼'))
            .padding(1, 1)
            .build();
        }
}

pub(crate) mod misc{
    pub(crate) fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
        let mut indices = (0..data.len()).collect::<Vec<_>>();
        indices.sort_by_key(|&i| &data[i]);
        indices
    }
}