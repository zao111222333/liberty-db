//! All trait `impl` for
//! `Timing`.

// mod check {
//   //! impl [Check](crate::common::Check) for
//   //! `Timing`.

//   use crate::{common::traits::Check, timing::Timing};

//   // TODO:
//   /// Timing arcs with a timing type of clear or preset require a `timing_sense` attribute.
//   /// <a name ="reference_link" href="
//   /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
//   /// ?field=test
//   /// &bgn
//   /// =214.5
//   /// &end
//   /// =214.5
//   /// ">Reference</a>
//   const fn need_timing_sense_when_timing_type_is_clear_or_preset(timing: &Timing) {
//     if timing.timing_sense.is_none() {}
//   }

//   // TODO:
//   /// If `related_pin` is an output pin, you must define a `timing_sense` attribute for that pin.
//   /// <a name ="reference_link" href="
//   /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
//   /// ?field=test
//   /// &bgn
//   /// =214.6
//   /// &end
//   /// =214.6
//   /// ">Reference</a>
//   const fn need_timing_sense_when_related_pin_is_output() {}

//   impl Check for Timing {
//     #[inline]
//     fn check(&self) {
//       need_timing_sense_when_timing_type_is_clear_or_preset(self);
//       need_timing_sense_when_related_pin_is_output();
//     }
//   }
// }
