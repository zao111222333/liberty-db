use crate::common::traits::Builder;

use super::Timing;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TimingBuilder {
  pub content: Box<Timing>,
}
impl Builder for TimingBuilder {
  fn build(&self) {}
}
