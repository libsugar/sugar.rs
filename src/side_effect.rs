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
impl<T> Effect<T> for Option<T> {
    #[inline(always)]
    fn effect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Some(ref v) = self {
            f(v);
        }
        self
    }
}
impl<T, E> Effect<T> for Result<T, E> {
    #[inline(always)]
    fn effect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Ok(ref v) = self {
            f(v);
        }
        self
    }
}
impl<T, S: core::ops::Deref<Target = T>> Effect<T> for &S {
    #[inline(always)]
    fn effect<F: FnOnce(&T)>(self, f: F) -> Self {
        f(self.deref());
        self
    }
}
