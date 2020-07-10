//! Some extension functions that are convenient for side effects

/// using
/// ## Usage
/// ```rust
/// # use batch_oper::*;
/// let v = (1, 2);
/// let v2 = (3, 4);
/// using!((a, b) = v, (c, d) = v2; {
///   println!("{} {} {} {}", a, b, c, d)
///   # ;
///   # assert_eq!(a, 1);
///   # assert_eq!(b, 2);
///   # assert_eq!(c, 3);
///   # assert_eq!(d, 4);
/// })
/// ```
/// *equivalent to*
/// ```no_run
/// let v = (1, 2);
/// let v2 = (3, 4);
/// {
///   let (a, b) = v;
///   let (c, d) = v2;
///   {
///     println!("{} {} {} {}", a, b, c, d)
///   }
/// }
///   ```
#[macro_export(local_inner_macros)]
macro_rules! using {
    { $($p:pat = $v:expr),* ; $b:block } => {
        { $(let $p = $v ;)* $b }
    };
}

/// Create an implicit variable, perform some side effects, and return it
/// ## Example
/// ```rust
/// # use batch_oper::effect;
/// let v = 1;
/// let v = effect(v, |v| { assert_eq!(*v, 1) });
/// assert_eq!(v, 1);
/// ```
#[inline(always)]
pub fn effect<T>(v: T, f: impl FnOnce(&T)) -> T {
    f(&v);
    v
}
/// Create an implicit variable, and make a mapping for it
/// ## Example
/// ```rust
/// # use batch_oper::using;
/// let v = 1;
/// let mut v = using(v, |v| { v + 1 });
/// assert_eq!(v, 2);
/// using(&mut v, |v| { *v = 3 });
/// assert_eq!(v, 3);
/// ```
#[inline(always)]
pub fn using<T, R>(v: T, f: impl FnOnce(T) -> R) -> R {
    f(v)
}

/// Create an implicit variable, perform some side effects, and return it
/// ## Example
/// ```rust
/// # use batch_oper::Effect;
/// let v = Some(1);
/// let v = v.effect(|v| { assert_eq!(*v, 1) });
/// assert_eq!(v, Some(1));
/// ```
#[deprecated = "Wait HKT"]
pub trait Effect<T> {
    /// Create an implicit variable, perform some side effects, and return it
    /// ## Example
    /// ```rust
    /// # use batch_oper::Effect;
    /// let v = Some(1);
    /// let v = v.effect(|v| { assert_eq!(*v, 1) });
    /// assert_eq!(v, Some(1));
    /// ```
    fn effect<F: FnOnce(&T)>(self, f: F) -> Self;
}
#[allow(deprecated)]
impl<T> Effect<T> for Option<T> {
    #[inline(always)]
    fn effect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Some(ref v) = self {
            f(v);
        }
        self
    }
}
#[allow(deprecated)]
impl<T, E> Effect<T> for Result<T, E> {
    #[inline(always)]
    fn effect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Ok(ref v) = self {
            f(v);
        }
        self
    }
}
#[allow(deprecated)]
impl<T, S: core::ops::Deref<Target = T>> Effect<T> for &S {
    #[inline(always)]
    fn effect<F: FnOnce(&T)>(self, f: F) -> Self {
        f(self.deref());
        self
    }
}

/// Create an implicit variable, and make a mapping for it
/// ## Example
/// ```rust
/// # use batch_oper::Used;
/// let v = 1.used(|v| { v + 1 });
/// assert_eq!(v, 2);
/// ```
pub trait Used: Sized {
    /// Create an implicit variable, and make a mapping for it
    /// ## Example
    /// ```rust
    /// # use batch_oper::Used;
    /// let v = 1.used(|v| { v + 1 });
    /// assert_eq!(v, 2);
    /// ```
    fn used<F: FnOnce(Self) -> R, R>(self, f: F) -> R;
}
impl<T> Used for T {
    fn used<F: FnOnce(Self) -> R, R>(self, f: F) -> R {
        f(self)
    }
}

/// Create an implicit variable, do some extra thing, and return it
/// ## Example
/// ```rust
/// # use batch_oper::Extra;
/// let v = 1.extra(|v| { println!("{}", v) });
/// assert_eq!(v, 1);
/// ```
pub trait Extra: Sized {
    /// Create an implicit variable, do some extra thing, and return it
    /// ## Example
    /// ```rust
    /// # use batch_oper::Extra;
    /// let v = 1.extra(|v| { println!("{}", v) });
    /// assert_eq!(v, 1);
    /// ```
    fn extra<F: FnOnce(&Self)>(self, f: F) -> Self;
}
impl<T> Extra for T {
    fn extra<F: FnOnce(&Self)>(self, f: F) -> Self {
        f(&self);
        self
    }
}

/// Create an implicit variable, do some extra thing, and return it
/// ## Example
/// ```rust
/// # use batch_oper::ExtraMut;
/// let v = 1.extra_mut(|v| {
///     println!("{}", v);
///     *v += 1;
/// });
/// assert_eq!(v, 2);
/// ```
pub trait ExtraMut: Sized {
    /// Create an implicit variable, do some extra thing, and return it
    /// ## Example
    /// ```rust
    /// # use batch_oper::ExtraMut;
    /// let v = 1.extra_mut(|v| {
    ///     println!("{}", v);
    ///     *v += 1;
    /// });
    /// assert_eq!(v, 2);
    /// ```
    fn extra_mut<F: FnOnce(&mut Self)>(self, f: F) -> Self;
}
impl<T> ExtraMut for T {
    fn extra_mut<F: FnOnce(&mut Self)>(mut self, f: F) -> Self {
        f(&mut self);
        self
    }
}
