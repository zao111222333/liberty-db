use crate::common::traits::Builder;

use super::Timing;

#[derive(Debug, Clone)]
pub struct TimingBuilder<'a>{
    pub content: &'a Timing<'a>,
}
impl Builder for TimingBuilder<'_> {
    fn build(&self){
    }
}