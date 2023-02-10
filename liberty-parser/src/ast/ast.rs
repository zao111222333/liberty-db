


/// name1 \[name2 name3 ... \]
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct NameList{
    name1: String,
    name2_n: Vec<String>,
}
// #[derive(Debug, Clone, PartialEq)]
// #[derive(Default)]
// pub struct Group{
// }

pub trait Group {
        fn name(&self) -> &String;
    // fn summarize(&self) -> String;
}
