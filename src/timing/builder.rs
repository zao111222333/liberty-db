use crate::common::traits::Builder;

use super::Timing;

#[derive(Debug, Clone)]
pub struct TimingBuilder {
  pub content: Box<Timing>,
}
impl Builder for TimingBuilder {
  fn build(&self) {}
}
