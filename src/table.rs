use crate::{expression::logic, pin::Pin, timing::TimingType};

enum TableInfo {
  Power(PowerInfo),
  Timing(TimingInfo),
}

struct TimingInfo {
  timing_type: TimingType,
}
struct PowerInfo {}
