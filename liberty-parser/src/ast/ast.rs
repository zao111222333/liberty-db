use compact_str::CompactString;


/// name1 \[name2 name3 ... \]
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct NameList{
    name1: CompactString,
    name2_n: Vec<CompactString>,
}
///
// #[derive(Debug, Clone, PartialEq)]
// #[derive(Default)]
// pub struct Group{
// }

pub trait Group {
    ///
    fn name(&self) -> &CompactString;
    // fn summarize(&self) -> String;
}
