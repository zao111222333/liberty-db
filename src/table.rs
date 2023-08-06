use crate::{ast::HashedGroup, expression::logic, pin::Pin, timing::TimingType};

struct TableId {
  vec: logic::Vector,
  pin: <Pin as HashedGroup>::Id,
  related_pin: <Pin as HashedGroup>::Id,
  info: TableInfo,
}

enum TableInfo {
  Power(PowerInfo),
  Timing(TimingInfo),
}

struct TimingInfo {
  timing_type: TimingType,
}
struct PowerInfo {}
