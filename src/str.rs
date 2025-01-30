#![expect(
  clippy::unnecessary_safety_doc,
  clippy::string_slice,
  clippy::partialeq_ne_impl
)]
use alloc::{borrow::Cow, rc::Rc, sync::Arc};
use core::{fmt, mem::MaybeUninit, ptr::NonNull};

pub use arcstr::{literal, ArcStr};

/// [`arcstr::ArcStr`] wrapper
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
#[repr(transparent)]
#[expect(clippy::unsafe_derive_deserialize)]
pub struct String(pub ArcStr);

impl core::ops::Deref for String {
  type Target = ArcStr;
  #[inline]
  fn deref(&self) -> &ArcStr {
    &self.0
  }
}

impl core::ops::DerefMut for String {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl fmt::Debug for String {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    <ArcStr as fmt::Debug>::fmt(&self.0, f)
  }
}

impl fmt::Display for String {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    <ArcStr as fmt::Display>::fmt(&self.0, f)
  }
}

impl AsRef<str> for String {
  #[inline]
  fn as_ref(&self) -> &str {
    self
  }
}

impl AsRef<[u8]> for String {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    self.as_bytes()
  }
}

impl core::str::FromStr for String {
  type Err = core::convert::Infallible;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self::from(s))
  }
}

impl core::borrow::Borrow<str> for String {
  #[inline]
  fn borrow(&self) -> &str {
    self
  }
}

macro_rules! impl_index {
    ($($IdxT:ty,)*) => {$(
        impl core::ops::Index<$IdxT> for String {
            type Output = str;
            #[inline]
            fn index(&self, i: $IdxT) -> &Self::Output {
                &self.as_str()[i]
            }
        }
    )*};
}

impl_index! {
    core::ops::RangeFull,
    core::ops::Range<usize>,
    core::ops::RangeFrom<usize>,
    core::ops::RangeTo<usize>,
    core::ops::RangeInclusive<usize>,
    core::ops::RangeToInclusive<usize>,
}

macro_rules! impl_peq {
    (@one $a:ty, $b:ty) => {
        impl<'a> PartialEq<$b> for $a {
            #[inline]
            fn eq(&self, s: &$b) -> bool {
                PartialEq::eq(&self[..], &s[..])
            }
            #[inline]
            fn ne(&self, s: &$b) -> bool {
                PartialEq::ne(&self[..], &s[..])
            }
        }
    };
    ($(($a:ty, $b:ty),)+) => {$(
        impl_peq!(@one $a, $b);
        impl_peq!(@one $b, $a);
    )+};
}

impl_peq! {
    (String, str),
    (String, &'a str),
    (String, String),
    (String, Cow<'a, str>),
    (String, Box<str>),
    (String, Arc<str>),
    (String, Rc<str>),
    (String, Arc<String>),
    (String, Rc<String>),
}

impl From<ArcStr> for String {
  #[inline]
  fn from(s: ArcStr) -> Self {
    Self(s)
  }
}

impl From<String> for ArcStr {
  #[inline]
  fn from(s: String) -> Self {
    s.0
  }
}

impl From<&str> for String {
  #[inline]
  fn from(s: &str) -> Self {
    Self(ArcStr::from(s))
  }
}

impl From<String> for String {
  #[inline]
  fn from(v: String) -> Self {
    Self(ArcStr::from(v))
  }
}

impl From<&mut str> for String {
  #[inline]
  fn from(s: &mut str) -> Self {
    Self(ArcStr::from(s))
  }
}

impl From<Box<str>> for String {
  #[inline]
  fn from(s: Box<str>) -> Self {
    Self(ArcStr::from(s))
  }
}
impl From<String> for Box<str> {
  #[inline]
  fn from(s: String) -> Self {
    s.0.into()
  }
}
impl From<String> for Rc<str> {
  #[inline]
  fn from(s: String) -> Self {
    s.0.into()
  }
}
impl From<String> for Arc<str> {
  #[inline]
  fn from(s: String) -> Self {
    s.0.into()
  }
}
impl From<Rc<str>> for String {
  #[inline]
  fn from(s: Rc<str>) -> Self {
    Self(ArcStr::from(s))
  }
}
impl From<Arc<str>> for String {
  #[inline]
  fn from(s: Arc<str>) -> Self {
    Self(ArcStr::from(s))
  }
}
impl<'a> From<Cow<'a, str>> for String {
  #[inline]
  fn from(s: Cow<'a, str>) -> Self {
    Self(ArcStr::from(s))
  }
}
impl<'a> From<&'a String> for Cow<'a, str> {
  #[inline]
  fn from(s: &'a String) -> Self {
    Cow::Borrowed(s)
  }
}

impl From<String> for Cow<'_, str> {
  #[inline]
  fn from(s: String) -> Self {
    s.0.into()
  }
}

impl From<&String> for String {
  #[inline]
  fn from(s: &String) -> Self {
    Self(ArcStr::from(s))
  }
}
impl From<&Self> for String {
  #[inline]
  fn from(s: &Self) -> Self {
    s.clone()
  }
}

impl String {
  /// Construct a new empty string.
  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self(ArcStr::new())
  }

  /// Attempt to copy the provided string into a newly allocated `ArcStr`, but
  /// return `None` if we cannot allocate the required memory.
  ///
  /// # Examples
  ///
  /// ```
  /// # use arcstr::ArcStr;
  ///
  /// # fn do_stuff_with(s: ArcStr) {}
  ///
  /// let some_big_str = "please pretend this is a very long string";
  /// if let Some(s) = ArcStr::try_alloc(some_big_str) {
  ///     do_stuff_with(s);
  /// } else {
  ///     // Complain about allocation failure, somehow.
  /// }
  /// ```
  #[inline]
  #[must_use]
  pub fn try_alloc(copy_from: &str) -> Option<Self> {
    ArcStr::try_alloc(copy_from).map(Into::into)
  }

  /// Attempt to allocate memory for an [`ArcStr`] of length `n`, and use the
  /// provided callback to fully initialize the provided buffer with valid
  /// UTF-8 text.
  ///
  /// This function returns `None` if memory allocation fails, see
  /// [`ArcStr::init_with_unchecked`] for a version which calls
  /// [`handle_alloc_error`](alloc::alloc::handle_alloc_error).
  ///
  /// # Safety
  /// The provided `initializer` callback must fully initialize the provided
  /// buffer with valid UTF-8 text.
  ///
  /// # Examples
  ///
  /// ```
  /// # use arcstr::ArcStr;
  /// # use core::mem::MaybeUninit;
  /// let arcstr = unsafe {
  ///     ArcStr::try_init_with_unchecked(10, |s: &mut [MaybeUninit<u8>]| {
  ///         s.fill(MaybeUninit::new(b'a'));
  ///     }).unwrap()
  /// };
  /// assert_eq!(arcstr, "aaaaaaaaaa")
  /// ```
  #[inline]
  #[must_use]
  pub unsafe fn try_init_with_unchecked<F>(n: usize, initializer: F) -> Option<Self>
  where
    F: FnOnce(&mut [MaybeUninit<u8>]),
  {
    ArcStr::try_init_with_unchecked(n, initializer).map(Into::into)
  }

  /// Allocate memory for an [`ArcStr`] of length `n`, and use the provided
  /// callback to fully initialize the provided buffer with valid UTF-8 text.
  ///
  /// This function calls
  /// [`handle_alloc_error`](alloc::alloc::handle_alloc_error) if memory
  /// allocation fails, see [`ArcStr::try_init_with_unchecked`] for a version
  /// which returns `None`
  ///
  /// # Safety
  /// The provided `initializer` callback must fully initialize the provided
  /// buffer with valid UTF-8 text.
  ///
  /// # Examples
  ///
  /// ```
  /// # use arcstr::ArcStr;
  /// # use core::mem::MaybeUninit;
  /// let arcstr = unsafe {
  ///     ArcStr::init_with_unchecked(10, |s: &mut [MaybeUninit<u8>]| {
  ///         s.fill(MaybeUninit::new(b'a'));
  ///     })
  /// };
  /// assert_eq!(arcstr, "aaaaaaaaaa")
  /// ```
  #[inline]
  #[must_use]
  pub unsafe fn init_with_unchecked<F>(n: usize, initializer: F) -> Self
  where
    F: FnOnce(&mut [MaybeUninit<u8>]),
  {
    ArcStr::init_with_unchecked(n, initializer).into()
  }

  /// Attempt to allocate memory for an [`ArcStr`] of length `n`, and use the
  /// provided callback to initialize the provided (initially-zeroed) buffer
  /// with valid UTF-8 text.
  ///
  /// Note: This function is provided with a zeroed buffer, and performs UTF-8
  /// validation after calling the initializer. While both of these are fast
  /// operations, some high-performance use cases will be better off using
  /// [`ArcStr::try_init_with_unchecked`] as the building block.
  ///
  /// # Errors
  /// The provided `initializer` callback must initialize the provided buffer
  /// with valid UTF-8 text, or a UTF-8 error will be returned.
  ///
  /// # Examples
  ///
  /// ```
  /// # use arcstr::ArcStr;
  ///
  /// let s = ArcStr::init_with(5, |slice| {
  ///     slice
  ///         .iter_mut()
  ///         .zip(b'0'..b'5')
  ///         .for_each(|(db, sb)| *db = sb);
  /// }).unwrap();
  /// assert_eq!(s, "01234");
  /// ```
  #[inline]
  pub fn init_with<F>(n: usize, initializer: F) -> Result<Self, core::str::Utf8Error>
  where
    F: FnOnce(&mut [u8]),
  {
    ArcStr::init_with(n, initializer).map(Into::into)
  }

  /// Extract a string slice containing our data.
  ///
  /// Note: This is an equivalent to our `Deref` implementation, but can be
  /// more readable than `&*s` in the cases where a manual invocation of
  /// `Deref` would be required.
  ///
  /// # Examples
  // TODO: find a better example where `&*` would have been required.
  /// ```
  /// # use arcstr::ArcStr;
  /// let s = ArcStr::from("abc");
  /// assert_eq!(s.as_str(), "abc");
  /// ```
  #[inline]
  #[must_use]
  pub fn as_str(&self) -> &str {
    self
  }

  /// Returns the length of this `ArcStr` in bytes.
  ///
  /// # Examples
  ///
  /// ```
  /// # use arcstr::ArcStr;
  /// let a = ArcStr::from("foo");
  /// assert_eq!(a.len(), 3);
  /// ```
  #[inline]
  #[must_use]
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Returns true if this `ArcStr` is empty.
  ///
  /// # Examples
  ///
  /// ```
  /// # use arcstr::ArcStr;
  /// assert!(!ArcStr::from("foo").is_empty());
  /// assert!(ArcStr::new().is_empty());
  /// ```
  #[inline]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Convert us to a `std::string::String`.
  ///
  /// This is provided as an inherent method to avoid needing to route through
  /// the `Display` machinery, but is equivalent to `ToString::to_string`.
  ///
  /// # Examples
  ///
  /// ```
  /// # use arcstr::ArcStr;
  /// let s = ArcStr::from("abc");
  /// assert_eq!(s.to_string(), "abc");
  /// ```
  #[inline]
  #[must_use]
  #[expect(clippy::inherent_to_string_shadow_display)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }

  /// Extract a byte slice containing the string's data.
  ///
  /// # Examples
  ///
  /// ```
  /// # use arcstr::ArcStr;
  /// let foobar = ArcStr::from("foobar");
  /// assert_eq!(foobar.as_bytes(), b"foobar");
  /// ```
  #[inline]
  #[must_use]
  pub fn as_bytes(&self) -> &[u8] {
    self.0.as_bytes()
  }

  /// Return the raw pointer this `ArcStr` wraps, for advanced use cases.
  ///
  /// Note that in addition to the `NonNull` constraint expressed in the type
  /// signature, we also guarantee the pointer has an alignment of at least 8
  /// bytes, even on platforms where a lower alignment would be acceptable.
  ///
  /// # Examples
  ///
  /// ```
  /// # use arcstr::ArcStr;
  /// let s = ArcStr::from("abcd");
  /// let p = ArcStr::into_raw(s);
  /// // Some time later...
  /// let s = unsafe { ArcStr::from_raw(p) };
  /// assert_eq!(s, "abcd");
  /// ```
  #[inline]
  #[must_use]
  pub fn into_raw(this: Self) -> NonNull<()> {
    ArcStr::into_raw(this.0)
  }

  /// The opposite version of [`Self::into_raw`]. Still intended only for
  /// advanced use cases.
  ///
  /// # Safety
  ///
  /// This function must be used on a valid pointer returned from
  /// [`ArcStr::into_raw`]. Additionally, you must ensure that a given `ArcStr`
  /// instance is only dropped once.
  ///
  /// # Examples
  ///
  /// ```
  /// # use arcstr::ArcStr;
  /// let s = ArcStr::from("abcd");
  /// let p = ArcStr::into_raw(s);
  /// // Some time later...
  /// let s = unsafe { ArcStr::from_raw(p) };
  /// assert_eq!(s, "abcd");
  /// ```
  #[inline]
  #[must_use]
  pub unsafe fn from_raw(ptr: NonNull<()>) -> Self {
    ArcStr::from_raw(ptr).into()
  }

  /// Returns true if the two `ArcStr`s point to the same allocation.
  ///
  /// Note that functions like `PartialEq` check this already, so there's
  /// no performance benefit to doing something like `ArcStr::ptr_eq(&a1, &a2) || (a1 == a2)`.
  ///
  /// Caveat: `const`s aren't guaranteed to only occur in an executable a
  /// single time, and so this may be non-deterministic for `ArcStr` defined
  /// in a `const` with [`arcstr::literal!`][crate::literal], unless one
  /// was created by a `clone()` on the other.
  ///
  /// # Examples
  ///
  /// ```
  /// use arcstr::ArcStr;
  ///
  /// let foobar = ArcStr::from("foobar");
  /// let same_foobar = foobar.clone();
  /// let other_foobar = ArcStr::from("foobar");
  /// assert!(ArcStr::ptr_eq(&foobar, &same_foobar));
  /// assert!(!ArcStr::ptr_eq(&foobar, &other_foobar));
  ///
  /// const YET_AGAIN_A_DIFFERENT_FOOBAR: ArcStr = arcstr::literal!("foobar");
  /// let strange_new_foobar = YET_AGAIN_A_DIFFERENT_FOOBAR.clone();
  /// let wild_blue_foobar = strange_new_foobar.clone();
  /// assert!(ArcStr::ptr_eq(&strange_new_foobar, &wild_blue_foobar));
  /// ```
  #[inline]
  #[must_use]
  pub fn ptr_eq(lhs: &Self, rhs: &Self) -> bool {
    core::ptr::eq(lhs.0.as_ptr(), rhs.0.as_ptr())
  }

  /// Returns the number of references that exist to this `ArcStr`. If this is
  /// a static `ArcStr` (For example, one from
  /// [`arcstr::literal!`][crate::literal]), returns `None`.
  ///
  /// Despite the difference in return type, this is named to match the method
  /// from the stdlib's Arc:
  /// [`Arc::strong_count`][alloc::sync::Arc::strong_count].
  ///
  /// If you aren't sure how to handle static `ArcStr` in the context of this
  /// return value, `ArcStr::strong_count(&s).unwrap_or(usize::MAX)` is
  /// frequently reasonable.
  ///
  /// # Safety
  ///
  /// This method by itself is safe, but using it correctly requires extra
  /// care. Another thread can change the strong count at any time, including
  /// potentially between calling this method and acting on the result.
  ///
  /// However, it may never change from `None` to `Some` or from `Some` to
  /// `None` for a given `ArcStr` — whether or not it is static is determined
  /// at construction, and never changes.
  ///
  /// # Examples
  ///
  /// ### Dynamic `ArcStr`
  /// ```
  /// # use arcstr::ArcStr;
  /// let foobar = ArcStr::from("foobar");
  /// assert_eq!(Some(1), ArcStr::strong_count(&foobar));
  /// let also_foobar = ArcStr::clone(&foobar);
  /// assert_eq!(Some(2), ArcStr::strong_count(&foobar));
  /// assert_eq!(Some(2), ArcStr::strong_count(&also_foobar));
  /// ```
  ///
  /// ### Static `ArcStr`
  /// ```
  /// # use arcstr::ArcStr;
  /// let baz = arcstr::literal!("baz");
  /// assert_eq!(None, ArcStr::strong_count(&baz));
  /// // Similarly:
  /// assert_eq!(None, ArcStr::strong_count(&ArcStr::default()));
  /// ```
  #[inline]
  #[must_use]
  pub fn strong_count(this: &Self) -> Option<usize> {
    ArcStr::strong_count(&this.0)
  }

  /// Convert the `ArcStr` into a "static" `ArcStr`, even if it was originally
  /// created from runtime values. The `&'static str` is returned.
  ///
  /// This is useful if you want to use [`ArcStr::as_static`] or
  /// [`ArcStr::is_static`] on a value only known at runtime.
  ///
  /// If the `ArcStr` is already static, then this is a noop.
  ///
  /// # Caveats
  /// Calling this function on an `ArcStr` will cause us to never free it, thus
  /// leaking it's memory. Doing this excessively can lead to problems.
  ///
  /// # Examples
  /// ```no_run
  /// # // This isn't run because it needs a leakcheck suppression,
  /// # // which I can't seem to make work in CI (no symbols for
  /// # // doctests?). Instead, we test this in tests/arc_str.rs
  /// # use arcstr::ArcStr;
  /// let s = ArcStr::from("foobar");
  /// assert!(!ArcStr::is_static(&s));
  /// assert!(ArcStr::as_static(&s).is_none());
  ///
  /// let leaked: &'static str = s.leak();
  /// assert_eq!(leaked, s);
  /// assert!(ArcStr::is_static(&s));
  /// assert_eq!(ArcStr::as_static(&s), Some("foobar"));
  /// ```
  #[inline]
  #[must_use]
  pub fn leak(&self) -> &'static str {
    self.0.leak()
  }
  /// Returns true if `this` is a "static" `ArcStr`. For example, if it was
  /// created from a call to [`arcstr::literal!`][crate::literal]),
  /// returned by `ArcStr::new`, etc.
  ///
  /// Static `ArcStr`s can be converted to `&'static str` for free using
  /// [`ArcStr::as_static`], without leaking memory — they're static constants
  /// in the program (somewhere).
  ///
  /// # Examples
  ///
  /// ```
  /// # use arcstr::ArcStr;
  /// const STATIC: ArcStr = arcstr::literal!("Electricity!");
  /// assert!(ArcStr::is_static(&STATIC));
  ///
  /// let still_static = arcstr::literal!("Shocking!");
  /// assert!(ArcStr::is_static(&still_static));
  /// assert!(
  ///     ArcStr::is_static(&still_static.clone()),
  ///     "Cloned statics are still static"
  /// );
  ///
  /// let nonstatic = ArcStr::from("Grounded...");
  /// assert!(!ArcStr::is_static(&nonstatic));
  /// ```
  #[inline]
  #[must_use]
  pub fn is_static(this: &Self) -> bool {
    ArcStr::is_static(&this.0)
  }

  /// Returns true if `this` is a "static"/`"literal"` `ArcStr`. For example, if
  /// it was created from a call to [`literal!`][crate::literal]), returned by
  /// `ArcStr::new`, etc.
  ///
  /// Static `ArcStr`s can be converted to `&'static str` for free using
  /// [`ArcStr::as_static`], without leaking memory — they're static constants
  /// in the program (somewhere).
  ///
  /// # Examples
  ///
  /// ```
  /// # use arcstr::ArcStr;
  /// const STATIC: ArcStr = arcstr::literal!("Electricity!");
  /// assert_eq!(ArcStr::as_static(&STATIC), Some("Electricity!"));
  ///
  /// // Note that they don't have to be consts, just made using `literal!`:
  /// let still_static = arcstr::literal!("Shocking!");
  /// assert_eq!(ArcStr::as_static(&still_static), Some("Shocking!"));
  /// // Cloning a static still produces a static.
  /// assert_eq!(ArcStr::as_static(&still_static.clone()), Some("Shocking!"));
  ///
  /// // But it won't work for strings from other sources.
  /// let nonstatic = ArcStr::from("Grounded...");
  /// assert_eq!(ArcStr::as_static(&nonstatic), None);
  /// ```
  #[inline]
  #[must_use]
  pub fn as_static(this: &Self) -> Option<&'static str> {
    ArcStr::as_static(&this.0)
  }

  /// Creates an `ArcStr` by repeating the source string `n` times
  ///
  /// # Errors
  ///
  /// This function returns an error if the capacity overflows or allocation
  /// fails.
  ///
  /// # Examples
  ///
  /// ```
  /// use arcstr::ArcStr;
  ///
  /// let source = "A";
  /// let repeated = ArcStr::try_repeat(source, 10);
  /// assert_eq!(repeated.unwrap(), "AAAAAAAAAA");
  /// ```
  #[inline]
  #[must_use]
  pub fn try_repeat(source: &str, n: usize) -> Option<Self> {
    ArcStr::try_repeat(source, n).map(Into::into)
  }

  /// Creates an `ArcStr` by repeating the source string `n` times
  ///
  /// # Panics
  ///
  /// This function panics if the capacity overflows, see
  /// [`try_repeat`](ArcStr::try_repeat) if this is undesirable.
  ///
  /// # Examples
  ///
  /// Basic usage:
  /// ```
  /// use arcstr::ArcStr;
  ///
  /// let source = "A";
  /// let repeated = ArcStr::repeat(source, 10);
  /// assert_eq!(repeated, "AAAAAAAAAA");
  /// ```
  ///
  /// A panic upon overflow:
  /// ```should_panic
  /// # use arcstr::ArcStr;
  ///
  /// // this will panic at runtime
  /// let huge = ArcStr::repeat("A", usize::MAX);
  /// ```
  #[inline]
  #[must_use]
  pub fn repeat(source: &str, n: usize) -> Self {
    ArcStr::repeat(source, n).into()
  }
}
