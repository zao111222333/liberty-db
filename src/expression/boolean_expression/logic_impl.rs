#![allow(clippy::items_after_test_module)]
#![allow(clippy::multiple_inherent_impl)]
use super::logic::{State, Static};
use core::ops::{BitAnd, BitOr, BitXor, Not};

impl BitAnd for Static {
  type Output = Self;
  #[must_use]
  #[inline]
  fn bitand(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (_, Self::L) | (Self::L, _) => Self::L,
      (Self::Z, _) | (_, Self::Z) => Self::X,
      (l, Self::H) => l,
      (Self::H, r) => r,
      _ => Self::Z,
    }
  }
}

impl BitOr for Static {
  type Output = Self;
  #[must_use]
  #[inline]
  fn bitor(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Self::H, _) | (_, Self::H) => Self::H,
      (Self::L, Self::L) => Self::L,
      _ => Self::Z,
    }
  }
}

impl BitXor for Static {
  type Output = Self;
  #[must_use]
  #[inline]
  fn bitxor(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Self::H, Self::L) | (Self::L, Self::H) => Self::H,
      (Self::H, Self::H) | (Self::L, Self::L) => Self::L,
      _ => Self::Z,
    }
  }
}

impl Not for Static {
  type Output = Self;
  #[must_use]
  #[inline]
  fn not(self) -> Self::Output {
    match self {
      Self::X => Self::X,
      Self::Z => Self::Z,
      Self::H => Self::L,
      Self::L => Self::H,
    }
  }
}
#[cfg(test)]
mod test {
  use super::*;
  use crate::IntoEnumIterator;

  impl State {
    fn combine_op2(self, rhs: Self, f: fn(Static, Static) -> Static) -> Self {
      Self::combine_bgn_end(f(self.bgn(), rhs.bgn()), f(self.end(), rhs.end()))
    }
  }

  #[inline]
  fn static_bitand(l: Static, r: Static) -> Static {
    l.bitand(r)
  }
  #[inline]
  fn static_bitor(l: Static, r: Static) -> Static {
    l.bitor(r)
  }
  #[inline]
  fn static_bitxor(l: Static, r: Static) -> Static {
    l.bitxor(r)
  }
  #[inline]
  fn static_not(l: Static) -> Static {
    l.not()
  }

  #[test]
  fn gen_match_op2() {
    for l in State::iter() {
      for r in State::iter() {
        println!(
          "(Self::{l},Self::{r})=>Self::{},",
          State::combine_op2(l, r, static_bitor)
        );
      }
    }
  }
  #[test]
  fn gen_match_op1() {
    for l in State::iter() {
      println!(
        "Self::{l}=>Self::{},",
        State::combine_bgn_end(l.bgn().not(), l.end().not())
      );
    }
  }
  #[test]
  fn gen_lut_op1() {
    for l in State::iter() {
      println!("Self::{},", State::combine_bgn_end(l.bgn().not(), l.end().not()));
    }
  }
  #[test]
  fn gen_lut_op2() {
    for l in State::iter() {
      for r in State::iter() {
        println!("Self::{},", State::combine_op2(l, r, static_bitor));
      }
    }
  }

  #[test]
  fn op2() {
    use std::time::SystemTime;
    let fns: [(
      &str,
      fn(Static, Static) -> Static,
      fn(State, State) -> State,
      fn(State, State) -> State,
    ); 3] = [
      ("and", static_bitand, State::lut_bitand, State::match_bitand),
      ("or", static_bitor, State::lut_bitor, State::match_bitor),
      ("xor", static_bitxor, State::lut_bitxor, State::match_bitxor),
    ];
    let n = 1000000;
    for (name, static_op2, lut_op2, match_op2) in fns {
      println!("=============== {name}");
      for l in State::iter() {
        for r in State::iter() {
          assert_eq!(State::combine_op2(l, r, static_op2), lut_op2(l, r));
          assert_eq!(State::combine_op2(l, r, static_op2), match_op2(l, r));
        }
      }
      let start_combine = SystemTime::now();
      for _ in 0..n {
        for l in State::iter() {
          for r in State::iter() {
            _ = criterion::black_box(State::combine_op2(l, r, static_op2));
          }
        }
      }
      let runtime_combine = SystemTime::now().duration_since(start_combine).unwrap();
      println!("runtime combine {runtime_combine:?}");
      let start_match = SystemTime::now();
      for _ in 0..n {
        for l in State::iter() {
          for r in State::iter() {
            _ = criterion::black_box(match_op2(l, r));
          }
        }
      }
      let runtime_match = SystemTime::now().duration_since(start_match).unwrap();
      println!("runtime match   {runtime_match:?}");
      let start_lut = SystemTime::now();
      for _ in 0..n {
        for l in State::iter() {
          for r in State::iter() {
            _ = criterion::black_box(lut_op2(l, r));
          }
        }
      }
      let runtime_lut = SystemTime::now().duration_since(start_lut).unwrap();
      println!("runtime lut     {runtime_lut:?}");
      #[cfg(not(debug_assertions))]
      {
        assert!(runtime_lut < runtime_match);
        assert!(runtime_match < runtime_combine);
      }
    }
  }

  #[test]
  fn op1() {
    use std::time::SystemTime;

    let n = 20000000;
    println!("=============== not");
    for l in State::iter() {
      let combine = State::combine_bgn_end(l.bgn().not(), l.end().not());
      assert_eq!(combine, State::match_not(l));
      assert_eq!(combine, State::lut_not(l));
    }
    let start_combine = SystemTime::now();
    for _ in 0..n {
      for l in State::iter() {
        let combine = State::combine_bgn_end(l.bgn().not(), l.end().not());
        _ = criterion::black_box(combine);
      }
    }
    let runtime_combine = SystemTime::now().duration_since(start_combine).unwrap();
    println!("runtime combine {runtime_combine:?}");
    let start_match = SystemTime::now();
    for _ in 0..n {
      for l in State::iter() {
        _ = criterion::black_box(State::match_not(l));
      }
    }
    let runtime_match = SystemTime::now().duration_since(start_match).unwrap();
    println!("runtime match   {runtime_match:?}");
    let start_lut = SystemTime::now();
    for _ in 0..n {
      for l in State::iter() {
        for r in State::iter() {
          _ = criterion::black_box(State::lut_not(l));
        }
      }
    }
    let runtime_lut = SystemTime::now().duration_since(start_lut).unwrap();
    println!("runtime lut     {runtime_lut:?}");
    #[cfg(not(debug_assertions))]
    {
      assert!(runtime_lut > runtime_combine);
      assert!(runtime_combine.as_secs_f32() / runtime_match.as_secs_f32() > 0.8);
    }
  }
}

impl BitAnd for State {
  type Output = Self;
  #[must_use]
  #[inline]
  fn bitand(self, rhs: Self) -> Self::Output {
    Self::lut_bitand(self, rhs)
  }
}

impl BitOr for State {
  type Output = Self;
  #[must_use]
  #[inline]
  fn bitor(self, rhs: Self) -> Self::Output {
    Self::lut_bitor(self, rhs)
  }
}

impl BitXor for State {
  type Output = Self;
  #[must_use]
  #[inline]
  fn bitxor(self, rhs: Self) -> Self::Output {
    Self::lut_bitxor(self, rhs)
  }
}

impl Not for State {
  type Output = Self;
  #[must_use]
  #[inline]
  fn not(self) -> Self::Output {
    Self::match_not(self)
  }
}

impl State {
  const LUT_AND: [Self; 196] = [
    Self::L,
    Self::L,
    Self::L,
    Self::L,
    Self::L,
    Self::L,
    Self::L,
    Self::L,
    Self::L,
    Self::L,
    Self::L,
    Self::L,
    Self::L,
    Self::L,
    Self::L,
    Self::H,
    Self::X,
    Self::X,
    Self::LH,
    Self::HL,
    Self::LX,
    Self::LX,
    Self::HX,
    Self::HX,
    Self::XL,
    Self::XL,
    Self::XH,
    Self::XH,
    Self::L,
    Self::X,
    Self::X,
    Self::X,
    Self::LX,
    Self::XL,
    Self::LZ,
    Self::LX,
    Self::X,
    Self::X,
    Self::ZL,
    Self::XL,
    Self::X,
    Self::X,
    Self::L,
    Self::X,
    Self::X,
    Self::X,
    Self::LX,
    Self::XL,
    Self::LX,
    Self::LX,
    Self::X,
    Self::X,
    Self::XL,
    Self::XL,
    Self::X,
    Self::X,
    Self::L,
    Self::LH,
    Self::LX,
    Self::LX,
    Self::LH,
    Self::L,
    Self::LX,
    Self::LX,
    Self::LX,
    Self::LX,
    Self::L,
    Self::L,
    Self::LH,
    Self::LH,
    Self::L,
    Self::HL,
    Self::XL,
    Self::XL,
    Self::L,
    Self::HL,
    Self::L,
    Self::L,
    Self::HL,
    Self::HL,
    Self::XL,
    Self::XL,
    Self::XL,
    Self::XL,
    Self::L,
    Self::LX,
    Self::LZ,
    Self::LX,
    Self::LX,
    Self::L,
    Self::LZ,
    Self::LX,
    Self::LZ,
    Self::LX,
    Self::L,
    Self::L,
    Self::LX,
    Self::LX,
    Self::L,
    Self::LX,
    Self::LX,
    Self::LX,
    Self::LX,
    Self::L,
    Self::LX,
    Self::LX,
    Self::LX,
    Self::LX,
    Self::L,
    Self::L,
    Self::LX,
    Self::LX,
    Self::L,
    Self::HX,
    Self::X,
    Self::X,
    Self::LX,
    Self::HL,
    Self::LZ,
    Self::LX,
    Self::HZ,
    Self::HX,
    Self::XL,
    Self::XL,
    Self::X,
    Self::X,
    Self::L,
    Self::HX,
    Self::X,
    Self::X,
    Self::LX,
    Self::HL,
    Self::LX,
    Self::LX,
    Self::HX,
    Self::HX,
    Self::XL,
    Self::XL,
    Self::X,
    Self::X,
    Self::L,
    Self::XL,
    Self::ZL,
    Self::XL,
    Self::L,
    Self::XL,
    Self::L,
    Self::L,
    Self::XL,
    Self::XL,
    Self::ZL,
    Self::XL,
    Self::ZL,
    Self::XL,
    Self::L,
    Self::XL,
    Self::XL,
    Self::XL,
    Self::L,
    Self::XL,
    Self::L,
    Self::L,
    Self::XL,
    Self::XL,
    Self::XL,
    Self::XL,
    Self::XL,
    Self::XL,
    Self::L,
    Self::XH,
    Self::X,
    Self::X,
    Self::LH,
    Self::XL,
    Self::LX,
    Self::LX,
    Self::X,
    Self::X,
    Self::ZL,
    Self::XL,
    Self::ZH,
    Self::XH,
    Self::L,
    Self::XH,
    Self::X,
    Self::X,
    Self::LH,
    Self::XL,
    Self::LX,
    Self::LX,
    Self::X,
    Self::X,
    Self::XL,
    Self::XL,
    Self::XH,
    Self::XH,
  ];
  #[must_use]
  #[inline]
  #[expect(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::as_conversions
  )]
  const fn lut_bitand(self, rhs: Self) -> Self {
    Self::LUT_AND[(self as usize) * 14 + (rhs as usize)]
  }
  #[must_use]
  #[inline]
  #[cfg(test)]
  const fn match_bitand(self, rhs: Self) -> Self {
    #[expect(clippy::match_same_arms)]
    match (self, rhs) {
      (Self::L, Self::L) => Self::L,
      (Self::L, Self::H) => Self::L,
      (Self::L, Self::X) => Self::L,
      (Self::L, Self::Z) => Self::L,
      (Self::L, Self::LH) => Self::L,
      (Self::L, Self::HL) => Self::L,
      (Self::L, Self::LX) => Self::L,
      (Self::L, Self::LZ) => Self::L,
      (Self::L, Self::HX) => Self::L,
      (Self::L, Self::HZ) => Self::L,
      (Self::L, Self::XL) => Self::L,
      (Self::L, Self::ZL) => Self::L,
      (Self::L, Self::XH) => Self::L,
      (Self::L, Self::ZH) => Self::L,
      (Self::H, Self::L) => Self::L,
      (Self::H, Self::H) => Self::H,
      (Self::H, Self::X) => Self::X,
      (Self::H, Self::Z) => Self::X,
      (Self::H, Self::LH) => Self::LH,
      (Self::H, Self::HL) => Self::HL,
      (Self::H, Self::LX) => Self::LX,
      (Self::H, Self::LZ) => Self::LX,
      (Self::H, Self::HX) => Self::HX,
      (Self::H, Self::HZ) => Self::HX,
      (Self::H, Self::XL) => Self::XL,
      (Self::H, Self::ZL) => Self::XL,
      (Self::H, Self::XH) => Self::XH,
      (Self::H, Self::ZH) => Self::XH,
      (Self::X, Self::L) => Self::L,
      (Self::X, Self::H) => Self::X,
      (Self::X, Self::X) => Self::X,
      (Self::X, Self::Z) => Self::X,
      (Self::X, Self::LH) => Self::LX,
      (Self::X, Self::HL) => Self::XL,
      (Self::X, Self::LX) => Self::LZ,
      (Self::X, Self::LZ) => Self::LX,
      (Self::X, Self::HX) => Self::X,
      (Self::X, Self::HZ) => Self::X,
      (Self::X, Self::XL) => Self::ZL,
      (Self::X, Self::ZL) => Self::XL,
      (Self::X, Self::XH) => Self::X,
      (Self::X, Self::ZH) => Self::X,
      (Self::Z, Self::L) => Self::L,
      (Self::Z, Self::H) => Self::X,
      (Self::Z, Self::X) => Self::X,
      (Self::Z, Self::Z) => Self::X,
      (Self::Z, Self::LH) => Self::LX,
      (Self::Z, Self::HL) => Self::XL,
      (Self::Z, Self::LX) => Self::LX,
      (Self::Z, Self::LZ) => Self::LX,
      (Self::Z, Self::HX) => Self::X,
      (Self::Z, Self::HZ) => Self::X,
      (Self::Z, Self::XL) => Self::XL,
      (Self::Z, Self::ZL) => Self::XL,
      (Self::Z, Self::XH) => Self::X,
      (Self::Z, Self::ZH) => Self::X,
      (Self::LH, Self::L) => Self::L,
      (Self::LH, Self::H) => Self::LH,
      (Self::LH, Self::X) => Self::LX,
      (Self::LH, Self::Z) => Self::LX,
      (Self::LH, Self::LH) => Self::LH,
      (Self::LH, Self::HL) => Self::L,
      (Self::LH, Self::LX) => Self::LX,
      (Self::LH, Self::LZ) => Self::LX,
      (Self::LH, Self::HX) => Self::LX,
      (Self::LH, Self::HZ) => Self::LX,
      (Self::LH, Self::XL) => Self::L,
      (Self::LH, Self::ZL) => Self::L,
      (Self::LH, Self::XH) => Self::LH,
      (Self::LH, Self::ZH) => Self::LH,
      (Self::HL, Self::L) => Self::L,
      (Self::HL, Self::H) => Self::HL,
      (Self::HL, Self::X) => Self::XL,
      (Self::HL, Self::Z) => Self::XL,
      (Self::HL, Self::LH) => Self::L,
      (Self::HL, Self::HL) => Self::HL,
      (Self::HL, Self::LX) => Self::L,
      (Self::HL, Self::LZ) => Self::L,
      (Self::HL, Self::HX) => Self::HL,
      (Self::HL, Self::HZ) => Self::HL,
      (Self::HL, Self::XL) => Self::XL,
      (Self::HL, Self::ZL) => Self::XL,
      (Self::HL, Self::XH) => Self::XL,
      (Self::HL, Self::ZH) => Self::XL,
      (Self::LX, Self::L) => Self::L,
      (Self::LX, Self::H) => Self::LX,
      (Self::LX, Self::X) => Self::LZ,
      (Self::LX, Self::Z) => Self::LX,
      (Self::LX, Self::LH) => Self::LX,
      (Self::LX, Self::HL) => Self::L,
      (Self::LX, Self::LX) => Self::LZ,
      (Self::LX, Self::LZ) => Self::LX,
      (Self::LX, Self::HX) => Self::LZ,
      (Self::LX, Self::HZ) => Self::LX,
      (Self::LX, Self::XL) => Self::L,
      (Self::LX, Self::ZL) => Self::L,
      (Self::LX, Self::XH) => Self::LX,
      (Self::LX, Self::ZH) => Self::LX,
      (Self::LZ, Self::L) => Self::L,
      (Self::LZ, Self::H) => Self::LX,
      (Self::LZ, Self::X) => Self::LX,
      (Self::LZ, Self::Z) => Self::LX,
      (Self::LZ, Self::LH) => Self::LX,
      (Self::LZ, Self::HL) => Self::L,
      (Self::LZ, Self::LX) => Self::LX,
      (Self::LZ, Self::LZ) => Self::LX,
      (Self::LZ, Self::HX) => Self::LX,
      (Self::LZ, Self::HZ) => Self::LX,
      (Self::LZ, Self::XL) => Self::L,
      (Self::LZ, Self::ZL) => Self::L,
      (Self::LZ, Self::XH) => Self::LX,
      (Self::LZ, Self::ZH) => Self::LX,
      (Self::HX, Self::L) => Self::L,
      (Self::HX, Self::H) => Self::HX,
      (Self::HX, Self::X) => Self::X,
      (Self::HX, Self::Z) => Self::X,
      (Self::HX, Self::LH) => Self::LX,
      (Self::HX, Self::HL) => Self::HL,
      (Self::HX, Self::LX) => Self::LZ,
      (Self::HX, Self::LZ) => Self::LX,
      (Self::HX, Self::HX) => Self::HZ,
      (Self::HX, Self::HZ) => Self::HX,
      (Self::HX, Self::XL) => Self::XL,
      (Self::HX, Self::ZL) => Self::XL,
      (Self::HX, Self::XH) => Self::X,
      (Self::HX, Self::ZH) => Self::X,
      (Self::HZ, Self::L) => Self::L,
      (Self::HZ, Self::H) => Self::HX,
      (Self::HZ, Self::X) => Self::X,
      (Self::HZ, Self::Z) => Self::X,
      (Self::HZ, Self::LH) => Self::LX,
      (Self::HZ, Self::HL) => Self::HL,
      (Self::HZ, Self::LX) => Self::LX,
      (Self::HZ, Self::LZ) => Self::LX,
      (Self::HZ, Self::HX) => Self::HX,
      (Self::HZ, Self::HZ) => Self::HX,
      (Self::HZ, Self::XL) => Self::XL,
      (Self::HZ, Self::ZL) => Self::XL,
      (Self::HZ, Self::XH) => Self::X,
      (Self::HZ, Self::ZH) => Self::X,
      (Self::XL, Self::L) => Self::L,
      (Self::XL, Self::H) => Self::XL,
      (Self::XL, Self::X) => Self::ZL,
      (Self::XL, Self::Z) => Self::XL,
      (Self::XL, Self::LH) => Self::L,
      (Self::XL, Self::HL) => Self::XL,
      (Self::XL, Self::LX) => Self::L,
      (Self::XL, Self::LZ) => Self::L,
      (Self::XL, Self::HX) => Self::XL,
      (Self::XL, Self::HZ) => Self::XL,
      (Self::XL, Self::XL) => Self::ZL,
      (Self::XL, Self::ZL) => Self::XL,
      (Self::XL, Self::XH) => Self::ZL,
      (Self::XL, Self::ZH) => Self::XL,
      (Self::ZL, Self::L) => Self::L,
      (Self::ZL, Self::H) => Self::XL,
      (Self::ZL, Self::X) => Self::XL,
      (Self::ZL, Self::Z) => Self::XL,
      (Self::ZL, Self::LH) => Self::L,
      (Self::ZL, Self::HL) => Self::XL,
      (Self::ZL, Self::LX) => Self::L,
      (Self::ZL, Self::LZ) => Self::L,
      (Self::ZL, Self::HX) => Self::XL,
      (Self::ZL, Self::HZ) => Self::XL,
      (Self::ZL, Self::XL) => Self::XL,
      (Self::ZL, Self::ZL) => Self::XL,
      (Self::ZL, Self::XH) => Self::XL,
      (Self::ZL, Self::ZH) => Self::XL,
      (Self::XH, Self::L) => Self::L,
      (Self::XH, Self::H) => Self::XH,
      (Self::XH, Self::X) => Self::X,
      (Self::XH, Self::Z) => Self::X,
      (Self::XH, Self::LH) => Self::LH,
      (Self::XH, Self::HL) => Self::XL,
      (Self::XH, Self::LX) => Self::LX,
      (Self::XH, Self::LZ) => Self::LX,
      (Self::XH, Self::HX) => Self::X,
      (Self::XH, Self::HZ) => Self::X,
      (Self::XH, Self::XL) => Self::ZL,
      (Self::XH, Self::ZL) => Self::XL,
      (Self::XH, Self::XH) => Self::ZH,
      (Self::XH, Self::ZH) => Self::XH,
      (Self::ZH, Self::L) => Self::L,
      (Self::ZH, Self::H) => Self::XH,
      (Self::ZH, Self::X) => Self::X,
      (Self::ZH, Self::Z) => Self::X,
      (Self::ZH, Self::LH) => Self::LH,
      (Self::ZH, Self::HL) => Self::XL,
      (Self::ZH, Self::LX) => Self::LX,
      (Self::ZH, Self::LZ) => Self::LX,
      (Self::ZH, Self::HX) => Self::X,
      (Self::ZH, Self::HZ) => Self::X,
      (Self::ZH, Self::XL) => Self::XL,
      (Self::ZH, Self::ZL) => Self::XL,
      (Self::ZH, Self::XH) => Self::XH,
      (Self::ZH, Self::ZH) => Self::XH,
    }
  }
}

impl State {
  const LUT_OR: [Self; 196] = [
    Self::L,
    Self::H,
    Self::X,
    Self::X,
    Self::LH,
    Self::HL,
    Self::LZ,
    Self::LZ,
    Self::HZ,
    Self::HZ,
    Self::ZL,
    Self::ZL,
    Self::ZH,
    Self::ZH,
    Self::H,
    Self::H,
    Self::H,
    Self::H,
    Self::H,
    Self::H,
    Self::H,
    Self::H,
    Self::H,
    Self::H,
    Self::H,
    Self::H,
    Self::H,
    Self::H,
    Self::X,
    Self::H,
    Self::X,
    Self::X,
    Self::ZH,
    Self::HZ,
    Self::X,
    Self::X,
    Self::HZ,
    Self::HZ,
    Self::X,
    Self::X,
    Self::ZH,
    Self::ZH,
    Self::X,
    Self::H,
    Self::X,
    Self::X,
    Self::ZH,
    Self::HZ,
    Self::X,
    Self::X,
    Self::HZ,
    Self::HZ,
    Self::X,
    Self::X,
    Self::ZH,
    Self::ZH,
    Self::LH,
    Self::H,
    Self::ZH,
    Self::ZH,
    Self::LH,
    Self::H,
    Self::LH,
    Self::LH,
    Self::H,
    Self::H,
    Self::ZH,
    Self::ZH,
    Self::ZH,
    Self::ZH,
    Self::HL,
    Self::H,
    Self::HZ,
    Self::HZ,
    Self::H,
    Self::HL,
    Self::HZ,
    Self::HZ,
    Self::HZ,
    Self::HZ,
    Self::HL,
    Self::HL,
    Self::H,
    Self::H,
    Self::LZ,
    Self::H,
    Self::X,
    Self::X,
    Self::LH,
    Self::HZ,
    Self::LZ,
    Self::LZ,
    Self::HZ,
    Self::HZ,
    Self::X,
    Self::X,
    Self::ZH,
    Self::ZH,
    Self::LZ,
    Self::H,
    Self::X,
    Self::X,
    Self::LH,
    Self::HZ,
    Self::LZ,
    Self::LZ,
    Self::HZ,
    Self::HZ,
    Self::X,
    Self::X,
    Self::ZH,
    Self::ZH,
    Self::HZ,
    Self::H,
    Self::HZ,
    Self::HZ,
    Self::H,
    Self::HZ,
    Self::HZ,
    Self::HZ,
    Self::HZ,
    Self::HZ,
    Self::HZ,
    Self::HZ,
    Self::H,
    Self::H,
    Self::HZ,
    Self::H,
    Self::HZ,
    Self::HZ,
    Self::H,
    Self::HZ,
    Self::HZ,
    Self::HZ,
    Self::HZ,
    Self::HZ,
    Self::HZ,
    Self::HZ,
    Self::H,
    Self::H,
    Self::ZL,
    Self::H,
    Self::X,
    Self::X,
    Self::ZH,
    Self::HL,
    Self::X,
    Self::X,
    Self::HZ,
    Self::HZ,
    Self::ZL,
    Self::ZL,
    Self::ZH,
    Self::ZH,
    Self::ZL,
    Self::H,
    Self::X,
    Self::X,
    Self::ZH,
    Self::HL,
    Self::X,
    Self::X,
    Self::HZ,
    Self::HZ,
    Self::ZL,
    Self::ZL,
    Self::ZH,
    Self::ZH,
    Self::ZH,
    Self::H,
    Self::ZH,
    Self::ZH,
    Self::ZH,
    Self::H,
    Self::ZH,
    Self::ZH,
    Self::H,
    Self::H,
    Self::ZH,
    Self::ZH,
    Self::ZH,
    Self::ZH,
    Self::ZH,
    Self::H,
    Self::ZH,
    Self::ZH,
    Self::ZH,
    Self::H,
    Self::ZH,
    Self::ZH,
    Self::H,
    Self::H,
    Self::ZH,
    Self::ZH,
    Self::ZH,
    Self::ZH,
  ];
  #[must_use]
  #[inline]
  #[expect(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::as_conversions
  )]
  const fn lut_bitor(self, rhs: Self) -> Self {
    Self::LUT_OR[(self as usize) * 14 + (rhs as usize)]
  }
  #[must_use]
  #[inline]
  #[cfg(test)]
  const fn match_bitor(self, rhs: Self) -> Self {
    #[expect(clippy::match_same_arms)]
    match (self, rhs) {
      (Self::L, Self::L) => Self::L,
      (Self::L, Self::H) => Self::H,
      (Self::L, Self::X) => Self::X,
      (Self::L, Self::Z) => Self::X,
      (Self::L, Self::LH) => Self::LH,
      (Self::L, Self::HL) => Self::HL,
      (Self::L, Self::LX) => Self::LZ,
      (Self::L, Self::LZ) => Self::LZ,
      (Self::L, Self::HX) => Self::HZ,
      (Self::L, Self::HZ) => Self::HZ,
      (Self::L, Self::XL) => Self::ZL,
      (Self::L, Self::ZL) => Self::ZL,
      (Self::L, Self::XH) => Self::ZH,
      (Self::L, Self::ZH) => Self::ZH,
      (Self::H, Self::L) => Self::H,
      (Self::H, Self::H) => Self::H,
      (Self::H, Self::X) => Self::H,
      (Self::H, Self::Z) => Self::H,
      (Self::H, Self::LH) => Self::H,
      (Self::H, Self::HL) => Self::H,
      (Self::H, Self::LX) => Self::H,
      (Self::H, Self::LZ) => Self::H,
      (Self::H, Self::HX) => Self::H,
      (Self::H, Self::HZ) => Self::H,
      (Self::H, Self::XL) => Self::H,
      (Self::H, Self::ZL) => Self::H,
      (Self::H, Self::XH) => Self::H,
      (Self::H, Self::ZH) => Self::H,
      (Self::X, Self::L) => Self::X,
      (Self::X, Self::H) => Self::H,
      (Self::X, Self::X) => Self::X,
      (Self::X, Self::Z) => Self::X,
      (Self::X, Self::LH) => Self::ZH,
      (Self::X, Self::HL) => Self::HZ,
      (Self::X, Self::LX) => Self::X,
      (Self::X, Self::LZ) => Self::X,
      (Self::X, Self::HX) => Self::HZ,
      (Self::X, Self::HZ) => Self::HZ,
      (Self::X, Self::XL) => Self::X,
      (Self::X, Self::ZL) => Self::X,
      (Self::X, Self::XH) => Self::ZH,
      (Self::X, Self::ZH) => Self::ZH,
      (Self::Z, Self::L) => Self::X,
      (Self::Z, Self::H) => Self::H,
      (Self::Z, Self::X) => Self::X,
      (Self::Z, Self::Z) => Self::X,
      (Self::Z, Self::LH) => Self::ZH,
      (Self::Z, Self::HL) => Self::HZ,
      (Self::Z, Self::LX) => Self::X,
      (Self::Z, Self::LZ) => Self::X,
      (Self::Z, Self::HX) => Self::HZ,
      (Self::Z, Self::HZ) => Self::HZ,
      (Self::Z, Self::XL) => Self::X,
      (Self::Z, Self::ZL) => Self::X,
      (Self::Z, Self::XH) => Self::ZH,
      (Self::Z, Self::ZH) => Self::ZH,
      (Self::LH, Self::L) => Self::LH,
      (Self::LH, Self::H) => Self::H,
      (Self::LH, Self::X) => Self::ZH,
      (Self::LH, Self::Z) => Self::ZH,
      (Self::LH, Self::LH) => Self::LH,
      (Self::LH, Self::HL) => Self::H,
      (Self::LH, Self::LX) => Self::LH,
      (Self::LH, Self::LZ) => Self::LH,
      (Self::LH, Self::HX) => Self::H,
      (Self::LH, Self::HZ) => Self::H,
      (Self::LH, Self::XL) => Self::ZH,
      (Self::LH, Self::ZL) => Self::ZH,
      (Self::LH, Self::XH) => Self::ZH,
      (Self::LH, Self::ZH) => Self::ZH,
      (Self::HL, Self::L) => Self::HL,
      (Self::HL, Self::H) => Self::H,
      (Self::HL, Self::X) => Self::HZ,
      (Self::HL, Self::Z) => Self::HZ,
      (Self::HL, Self::LH) => Self::H,
      (Self::HL, Self::HL) => Self::HL,
      (Self::HL, Self::LX) => Self::HZ,
      (Self::HL, Self::LZ) => Self::HZ,
      (Self::HL, Self::HX) => Self::HZ,
      (Self::HL, Self::HZ) => Self::HZ,
      (Self::HL, Self::XL) => Self::HL,
      (Self::HL, Self::ZL) => Self::HL,
      (Self::HL, Self::XH) => Self::H,
      (Self::HL, Self::ZH) => Self::H,
      (Self::LX, Self::L) => Self::LZ,
      (Self::LX, Self::H) => Self::H,
      (Self::LX, Self::X) => Self::X,
      (Self::LX, Self::Z) => Self::X,
      (Self::LX, Self::LH) => Self::LH,
      (Self::LX, Self::HL) => Self::HZ,
      (Self::LX, Self::LX) => Self::LZ,
      (Self::LX, Self::LZ) => Self::LZ,
      (Self::LX, Self::HX) => Self::HZ,
      (Self::LX, Self::HZ) => Self::HZ,
      (Self::LX, Self::XL) => Self::X,
      (Self::LX, Self::ZL) => Self::X,
      (Self::LX, Self::XH) => Self::ZH,
      (Self::LX, Self::ZH) => Self::ZH,
      (Self::LZ, Self::L) => Self::LZ,
      (Self::LZ, Self::H) => Self::H,
      (Self::LZ, Self::X) => Self::X,
      (Self::LZ, Self::Z) => Self::X,
      (Self::LZ, Self::LH) => Self::LH,
      (Self::LZ, Self::HL) => Self::HZ,
      (Self::LZ, Self::LX) => Self::LZ,
      (Self::LZ, Self::LZ) => Self::LZ,
      (Self::LZ, Self::HX) => Self::HZ,
      (Self::LZ, Self::HZ) => Self::HZ,
      (Self::LZ, Self::XL) => Self::X,
      (Self::LZ, Self::ZL) => Self::X,
      (Self::LZ, Self::XH) => Self::ZH,
      (Self::LZ, Self::ZH) => Self::ZH,
      (Self::HX, Self::L) => Self::HZ,
      (Self::HX, Self::H) => Self::H,
      (Self::HX, Self::X) => Self::HZ,
      (Self::HX, Self::Z) => Self::HZ,
      (Self::HX, Self::LH) => Self::H,
      (Self::HX, Self::HL) => Self::HZ,
      (Self::HX, Self::LX) => Self::HZ,
      (Self::HX, Self::LZ) => Self::HZ,
      (Self::HX, Self::HX) => Self::HZ,
      (Self::HX, Self::HZ) => Self::HZ,
      (Self::HX, Self::XL) => Self::HZ,
      (Self::HX, Self::ZL) => Self::HZ,
      (Self::HX, Self::XH) => Self::H,
      (Self::HX, Self::ZH) => Self::H,
      (Self::HZ, Self::L) => Self::HZ,
      (Self::HZ, Self::H) => Self::H,
      (Self::HZ, Self::X) => Self::HZ,
      (Self::HZ, Self::Z) => Self::HZ,
      (Self::HZ, Self::LH) => Self::H,
      (Self::HZ, Self::HL) => Self::HZ,
      (Self::HZ, Self::LX) => Self::HZ,
      (Self::HZ, Self::LZ) => Self::HZ,
      (Self::HZ, Self::HX) => Self::HZ,
      (Self::HZ, Self::HZ) => Self::HZ,
      (Self::HZ, Self::XL) => Self::HZ,
      (Self::HZ, Self::ZL) => Self::HZ,
      (Self::HZ, Self::XH) => Self::H,
      (Self::HZ, Self::ZH) => Self::H,
      (Self::XL, Self::L) => Self::ZL,
      (Self::XL, Self::H) => Self::H,
      (Self::XL, Self::X) => Self::X,
      (Self::XL, Self::Z) => Self::X,
      (Self::XL, Self::LH) => Self::ZH,
      (Self::XL, Self::HL) => Self::HL,
      (Self::XL, Self::LX) => Self::X,
      (Self::XL, Self::LZ) => Self::X,
      (Self::XL, Self::HX) => Self::HZ,
      (Self::XL, Self::HZ) => Self::HZ,
      (Self::XL, Self::XL) => Self::ZL,
      (Self::XL, Self::ZL) => Self::ZL,
      (Self::XL, Self::XH) => Self::ZH,
      (Self::XL, Self::ZH) => Self::ZH,
      (Self::ZL, Self::L) => Self::ZL,
      (Self::ZL, Self::H) => Self::H,
      (Self::ZL, Self::X) => Self::X,
      (Self::ZL, Self::Z) => Self::X,
      (Self::ZL, Self::LH) => Self::ZH,
      (Self::ZL, Self::HL) => Self::HL,
      (Self::ZL, Self::LX) => Self::X,
      (Self::ZL, Self::LZ) => Self::X,
      (Self::ZL, Self::HX) => Self::HZ,
      (Self::ZL, Self::HZ) => Self::HZ,
      (Self::ZL, Self::XL) => Self::ZL,
      (Self::ZL, Self::ZL) => Self::ZL,
      (Self::ZL, Self::XH) => Self::ZH,
      (Self::ZL, Self::ZH) => Self::ZH,
      (Self::XH, Self::L) => Self::ZH,
      (Self::XH, Self::H) => Self::H,
      (Self::XH, Self::X) => Self::ZH,
      (Self::XH, Self::Z) => Self::ZH,
      (Self::XH, Self::LH) => Self::ZH,
      (Self::XH, Self::HL) => Self::H,
      (Self::XH, Self::LX) => Self::ZH,
      (Self::XH, Self::LZ) => Self::ZH,
      (Self::XH, Self::HX) => Self::H,
      (Self::XH, Self::HZ) => Self::H,
      (Self::XH, Self::XL) => Self::ZH,
      (Self::XH, Self::ZL) => Self::ZH,
      (Self::XH, Self::XH) => Self::ZH,
      (Self::XH, Self::ZH) => Self::ZH,
      (Self::ZH, Self::L) => Self::ZH,
      (Self::ZH, Self::H) => Self::H,
      (Self::ZH, Self::X) => Self::ZH,
      (Self::ZH, Self::Z) => Self::ZH,
      (Self::ZH, Self::LH) => Self::ZH,
      (Self::ZH, Self::HL) => Self::H,
      (Self::ZH, Self::LX) => Self::ZH,
      (Self::ZH, Self::LZ) => Self::ZH,
      (Self::ZH, Self::HX) => Self::H,
      (Self::ZH, Self::HZ) => Self::H,
      (Self::ZH, Self::XL) => Self::ZH,
      (Self::ZH, Self::ZL) => Self::ZH,
      (Self::ZH, Self::XH) => Self::ZH,
      (Self::ZH, Self::ZH) => Self::ZH,
    }
  }
}

impl State {
  const LUT_XOR: [Self; 196] = [
    Self::L,
    Self::H,
    Self::X,
    Self::X,
    Self::LH,
    Self::HL,
    Self::LZ,
    Self::LZ,
    Self::HZ,
    Self::HZ,
    Self::ZL,
    Self::ZL,
    Self::ZH,
    Self::ZH,
    Self::H,
    Self::L,
    Self::X,
    Self::X,
    Self::HL,
    Self::LH,
    Self::HZ,
    Self::HZ,
    Self::LZ,
    Self::LZ,
    Self::ZH,
    Self::ZH,
    Self::ZL,
    Self::ZL,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::LH,
    Self::HL,
    Self::X,
    Self::X,
    Self::L,
    Self::H,
    Self::LZ,
    Self::LZ,
    Self::HZ,
    Self::HZ,
    Self::ZH,
    Self::ZH,
    Self::ZL,
    Self::ZL,
    Self::HL,
    Self::LH,
    Self::X,
    Self::X,
    Self::H,
    Self::L,
    Self::HZ,
    Self::HZ,
    Self::LZ,
    Self::LZ,
    Self::ZL,
    Self::ZL,
    Self::ZH,
    Self::ZH,
    Self::LZ,
    Self::HZ,
    Self::X,
    Self::X,
    Self::LZ,
    Self::HZ,
    Self::LZ,
    Self::LZ,
    Self::HZ,
    Self::HZ,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::LZ,
    Self::HZ,
    Self::X,
    Self::X,
    Self::LZ,
    Self::HZ,
    Self::LZ,
    Self::LZ,
    Self::HZ,
    Self::HZ,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::HZ,
    Self::LZ,
    Self::X,
    Self::X,
    Self::HZ,
    Self::LZ,
    Self::HZ,
    Self::HZ,
    Self::LZ,
    Self::LZ,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::HZ,
    Self::LZ,
    Self::X,
    Self::X,
    Self::HZ,
    Self::LZ,
    Self::HZ,
    Self::HZ,
    Self::LZ,
    Self::LZ,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::ZL,
    Self::ZH,
    Self::X,
    Self::X,
    Self::ZH,
    Self::ZL,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::ZL,
    Self::ZL,
    Self::ZH,
    Self::ZH,
    Self::ZL,
    Self::ZH,
    Self::X,
    Self::X,
    Self::ZH,
    Self::ZL,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::ZL,
    Self::ZL,
    Self::ZH,
    Self::ZH,
    Self::ZH,
    Self::ZL,
    Self::X,
    Self::X,
    Self::ZL,
    Self::ZH,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::ZH,
    Self::ZH,
    Self::ZL,
    Self::ZL,
    Self::ZH,
    Self::ZL,
    Self::X,
    Self::X,
    Self::ZL,
    Self::ZH,
    Self::X,
    Self::X,
    Self::X,
    Self::X,
    Self::ZH,
    Self::ZH,
    Self::ZL,
    Self::ZL,
  ];
  #[must_use]
  #[inline]
  #[expect(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::as_conversions
  )]
  const fn lut_bitxor(self, rhs: Self) -> Self {
    Self::LUT_XOR[(self as usize) * 14 + (rhs as usize)]
  }
  #[must_use]
  #[inline]
  #[cfg(test)]
  const fn match_bitxor(self, rhs: Self) -> Self {
    #[expect(clippy::match_same_arms)]
    match (self, rhs) {
      (Self::L, Self::L) => Self::L,
      (Self::L, Self::H) => Self::H,
      (Self::L, Self::X) => Self::X,
      (Self::L, Self::Z) => Self::X,
      (Self::L, Self::LH) => Self::LH,
      (Self::L, Self::HL) => Self::HL,
      (Self::L, Self::LX) => Self::LZ,
      (Self::L, Self::LZ) => Self::LZ,
      (Self::L, Self::HX) => Self::HZ,
      (Self::L, Self::HZ) => Self::HZ,
      (Self::L, Self::XL) => Self::ZL,
      (Self::L, Self::ZL) => Self::ZL,
      (Self::L, Self::XH) => Self::ZH,
      (Self::L, Self::ZH) => Self::ZH,
      (Self::H, Self::L) => Self::H,
      (Self::H, Self::H) => Self::L,
      (Self::H, Self::X) => Self::X,
      (Self::H, Self::Z) => Self::X,
      (Self::H, Self::LH) => Self::HL,
      (Self::H, Self::HL) => Self::LH,
      (Self::H, Self::LX) => Self::HZ,
      (Self::H, Self::LZ) => Self::HZ,
      (Self::H, Self::HX) => Self::LZ,
      (Self::H, Self::HZ) => Self::LZ,
      (Self::H, Self::XL) => Self::ZH,
      (Self::H, Self::ZL) => Self::ZH,
      (Self::H, Self::XH) => Self::ZL,
      (Self::H, Self::ZH) => Self::ZL,
      (Self::X, Self::L) => Self::X,
      (Self::X, Self::H) => Self::X,
      (Self::X, Self::X) => Self::X,
      (Self::X, Self::Z) => Self::X,
      (Self::X, Self::LH) => Self::X,
      (Self::X, Self::HL) => Self::X,
      (Self::X, Self::LX) => Self::X,
      (Self::X, Self::LZ) => Self::X,
      (Self::X, Self::HX) => Self::X,
      (Self::X, Self::HZ) => Self::X,
      (Self::X, Self::XL) => Self::X,
      (Self::X, Self::ZL) => Self::X,
      (Self::X, Self::XH) => Self::X,
      (Self::X, Self::ZH) => Self::X,
      (Self::Z, Self::L) => Self::X,
      (Self::Z, Self::H) => Self::X,
      (Self::Z, Self::X) => Self::X,
      (Self::Z, Self::Z) => Self::X,
      (Self::Z, Self::LH) => Self::X,
      (Self::Z, Self::HL) => Self::X,
      (Self::Z, Self::LX) => Self::X,
      (Self::Z, Self::LZ) => Self::X,
      (Self::Z, Self::HX) => Self::X,
      (Self::Z, Self::HZ) => Self::X,
      (Self::Z, Self::XL) => Self::X,
      (Self::Z, Self::ZL) => Self::X,
      (Self::Z, Self::XH) => Self::X,
      (Self::Z, Self::ZH) => Self::X,
      (Self::LH, Self::L) => Self::LH,
      (Self::LH, Self::H) => Self::HL,
      (Self::LH, Self::X) => Self::X,
      (Self::LH, Self::Z) => Self::X,
      (Self::LH, Self::LH) => Self::L,
      (Self::LH, Self::HL) => Self::H,
      (Self::LH, Self::LX) => Self::LZ,
      (Self::LH, Self::LZ) => Self::LZ,
      (Self::LH, Self::HX) => Self::HZ,
      (Self::LH, Self::HZ) => Self::HZ,
      (Self::LH, Self::XL) => Self::ZH,
      (Self::LH, Self::ZL) => Self::ZH,
      (Self::LH, Self::XH) => Self::ZL,
      (Self::LH, Self::ZH) => Self::ZL,
      (Self::HL, Self::L) => Self::HL,
      (Self::HL, Self::H) => Self::LH,
      (Self::HL, Self::X) => Self::X,
      (Self::HL, Self::Z) => Self::X,
      (Self::HL, Self::LH) => Self::H,
      (Self::HL, Self::HL) => Self::L,
      (Self::HL, Self::LX) => Self::HZ,
      (Self::HL, Self::LZ) => Self::HZ,
      (Self::HL, Self::HX) => Self::LZ,
      (Self::HL, Self::HZ) => Self::LZ,
      (Self::HL, Self::XL) => Self::ZL,
      (Self::HL, Self::ZL) => Self::ZL,
      (Self::HL, Self::XH) => Self::ZH,
      (Self::HL, Self::ZH) => Self::ZH,
      (Self::LX, Self::L) => Self::LZ,
      (Self::LX, Self::H) => Self::HZ,
      (Self::LX, Self::X) => Self::X,
      (Self::LX, Self::Z) => Self::X,
      (Self::LX, Self::LH) => Self::LZ,
      (Self::LX, Self::HL) => Self::HZ,
      (Self::LX, Self::LX) => Self::LZ,
      (Self::LX, Self::LZ) => Self::LZ,
      (Self::LX, Self::HX) => Self::HZ,
      (Self::LX, Self::HZ) => Self::HZ,
      (Self::LX, Self::XL) => Self::X,
      (Self::LX, Self::ZL) => Self::X,
      (Self::LX, Self::XH) => Self::X,
      (Self::LX, Self::ZH) => Self::X,
      (Self::LZ, Self::L) => Self::LZ,
      (Self::LZ, Self::H) => Self::HZ,
      (Self::LZ, Self::X) => Self::X,
      (Self::LZ, Self::Z) => Self::X,
      (Self::LZ, Self::LH) => Self::LZ,
      (Self::LZ, Self::HL) => Self::HZ,
      (Self::LZ, Self::LX) => Self::LZ,
      (Self::LZ, Self::LZ) => Self::LZ,
      (Self::LZ, Self::HX) => Self::HZ,
      (Self::LZ, Self::HZ) => Self::HZ,
      (Self::LZ, Self::XL) => Self::X,
      (Self::LZ, Self::ZL) => Self::X,
      (Self::LZ, Self::XH) => Self::X,
      (Self::LZ, Self::ZH) => Self::X,
      (Self::HX, Self::L) => Self::HZ,
      (Self::HX, Self::H) => Self::LZ,
      (Self::HX, Self::X) => Self::X,
      (Self::HX, Self::Z) => Self::X,
      (Self::HX, Self::LH) => Self::HZ,
      (Self::HX, Self::HL) => Self::LZ,
      (Self::HX, Self::LX) => Self::HZ,
      (Self::HX, Self::LZ) => Self::HZ,
      (Self::HX, Self::HX) => Self::LZ,
      (Self::HX, Self::HZ) => Self::LZ,
      (Self::HX, Self::XL) => Self::X,
      (Self::HX, Self::ZL) => Self::X,
      (Self::HX, Self::XH) => Self::X,
      (Self::HX, Self::ZH) => Self::X,
      (Self::HZ, Self::L) => Self::HZ,
      (Self::HZ, Self::H) => Self::LZ,
      (Self::HZ, Self::X) => Self::X,
      (Self::HZ, Self::Z) => Self::X,
      (Self::HZ, Self::LH) => Self::HZ,
      (Self::HZ, Self::HL) => Self::LZ,
      (Self::HZ, Self::LX) => Self::HZ,
      (Self::HZ, Self::LZ) => Self::HZ,
      (Self::HZ, Self::HX) => Self::LZ,
      (Self::HZ, Self::HZ) => Self::LZ,
      (Self::HZ, Self::XL) => Self::X,
      (Self::HZ, Self::ZL) => Self::X,
      (Self::HZ, Self::XH) => Self::X,
      (Self::HZ, Self::ZH) => Self::X,
      (Self::XL, Self::L) => Self::ZL,
      (Self::XL, Self::H) => Self::ZH,
      (Self::XL, Self::X) => Self::X,
      (Self::XL, Self::Z) => Self::X,
      (Self::XL, Self::LH) => Self::ZH,
      (Self::XL, Self::HL) => Self::ZL,
      (Self::XL, Self::LX) => Self::X,
      (Self::XL, Self::LZ) => Self::X,
      (Self::XL, Self::HX) => Self::X,
      (Self::XL, Self::HZ) => Self::X,
      (Self::XL, Self::XL) => Self::ZL,
      (Self::XL, Self::ZL) => Self::ZL,
      (Self::XL, Self::XH) => Self::ZH,
      (Self::XL, Self::ZH) => Self::ZH,
      (Self::ZL, Self::L) => Self::ZL,
      (Self::ZL, Self::H) => Self::ZH,
      (Self::ZL, Self::X) => Self::X,
      (Self::ZL, Self::Z) => Self::X,
      (Self::ZL, Self::LH) => Self::ZH,
      (Self::ZL, Self::HL) => Self::ZL,
      (Self::ZL, Self::LX) => Self::X,
      (Self::ZL, Self::LZ) => Self::X,
      (Self::ZL, Self::HX) => Self::X,
      (Self::ZL, Self::HZ) => Self::X,
      (Self::ZL, Self::XL) => Self::ZL,
      (Self::ZL, Self::ZL) => Self::ZL,
      (Self::ZL, Self::XH) => Self::ZH,
      (Self::ZL, Self::ZH) => Self::ZH,
      (Self::XH, Self::L) => Self::ZH,
      (Self::XH, Self::H) => Self::ZL,
      (Self::XH, Self::X) => Self::X,
      (Self::XH, Self::Z) => Self::X,
      (Self::XH, Self::LH) => Self::ZL,
      (Self::XH, Self::HL) => Self::ZH,
      (Self::XH, Self::LX) => Self::X,
      (Self::XH, Self::LZ) => Self::X,
      (Self::XH, Self::HX) => Self::X,
      (Self::XH, Self::HZ) => Self::X,
      (Self::XH, Self::XL) => Self::ZH,
      (Self::XH, Self::ZL) => Self::ZH,
      (Self::XH, Self::XH) => Self::ZL,
      (Self::XH, Self::ZH) => Self::ZL,
      (Self::ZH, Self::L) => Self::ZH,
      (Self::ZH, Self::H) => Self::ZL,
      (Self::ZH, Self::X) => Self::X,
      (Self::ZH, Self::Z) => Self::X,
      (Self::ZH, Self::LH) => Self::ZL,
      (Self::ZH, Self::HL) => Self::ZH,
      (Self::ZH, Self::LX) => Self::X,
      (Self::ZH, Self::LZ) => Self::X,
      (Self::ZH, Self::HX) => Self::X,
      (Self::ZH, Self::HZ) => Self::X,
      (Self::ZH, Self::XL) => Self::ZH,
      (Self::ZH, Self::ZL) => Self::ZH,
      (Self::ZH, Self::XH) => Self::ZL,
      (Self::ZH, Self::ZH) => Self::ZL,
    }
  }
}

impl State {
  #[cfg(test)]
  const LUT_NOT: [Self; 14] = [
    Self::H,
    Self::L,
    Self::X,
    Self::X,
    Self::HL,
    Self::LH,
    Self::HX,
    Self::HZ,
    Self::LX,
    Self::LZ,
    Self::XH,
    Self::ZH,
    Self::XL,
    Self::ZL,
  ];
  #[must_use]
  #[inline]
  #[cfg(test)]
  fn combine_not(self) -> Self {
    Self::combine_bgn_end(self.bgn().not(), self.end().not())
  }
  #[must_use]
  #[inline]
  #[cfg(test)]
  #[expect(clippy::indexing_slicing, clippy::as_conversions)]
  const fn lut_not(self) -> Self {
    Self::LUT_NOT[self as usize]
  }
  #[must_use]
  #[inline]
  const fn match_not(self) -> Self {
    #[expect(clippy::match_same_arms)]
    match self {
      Self::L => Self::H,
      Self::H => Self::L,
      Self::X => Self::X,
      Self::Z => Self::X,
      Self::LH => Self::HL,
      Self::HL => Self::LH,
      Self::LX => Self::HX,
      Self::LZ => Self::HZ,
      Self::HX => Self::LX,
      Self::HZ => Self::LZ,
      Self::XL => Self::XH,
      Self::ZL => Self::ZH,
      Self::XH => Self::XL,
      Self::ZH => Self::ZL,
    }
  }
}
