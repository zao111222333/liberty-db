//! All trait `impl` for
//! `Timing`.

// mod check {
//   //! impl [Check](crate::common::Check) for
//   //! `Timing`.

//   use crate::{common::traits::Check, timing::Timing};

use crate::Ctx;

use super::{Timing, TimingType};

// TODO:
/// Timing arcs with a timing type of `clear` or `preset` require a `timing_sense` attribute.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=330.5&end=330.6
/// ">Reference</a>
pub(super) fn need_timing_sense_when_timing_type_is_clear_or_preset<C: Ctx>(
  timing: &Timing<C>,
) {
  if matches!(timing.timing_type, Some(TimingType::CLEAR | TimingType::PRESET))
    && timing.timing_sense.is_none()
  {
    crate::error!(
      "Build Error: Timing arcs with a timing type of `clear` or `preset` require a `timing_sense` attribute."
    );
  }
}

/// If `related_pin` is an output pin, you must define a `timing_sense` attribute for that pin.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=330.6&end=330.7
/// ">Reference</a>
pub(super) const fn need_timing_sense_when_related_pin_is_output<C: Ctx>(
  _timing: &Timing<C>,
) {
  // TODO
  //   if timing.related_pin && timing.timing_sense.is_none() {
  //     crate::error!("Build Error: If `related_pin` is an output pin, you must define a `timing_sense` attribute for that pin.");
  //   }
}
