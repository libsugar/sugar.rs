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
/// # use batch_oper::Also;
/// let v = 1.also(|v| { println!("{}", v) });
/// assert_eq!(v, 1);
/// ```
pub trait Also: Sized {
    /// Create an implicit variable, do some extra thing, and return it
    /// ## Example
    /// ```rust
    /// # use batch_oper::Also;
    /// let v = 1.also(|v| { println!("{}", v) });
    /// assert_eq!(v, 1);
    /// ```
    fn also<F: FnOnce(&Self)>(self, f: F) -> Self;
}
impl<T> Also for T {
    fn also<F: FnOnce(&Self)>(self, f: F) -> Self {
        f(&self);
        self
    }
}

/// Create an implicit variable, do some extra thing, and return it
/// ## Example
/// ```rust
/// # use batch_oper::AlsoMut;
/// let v = 1.also_mut(|v| {
///     println!("{}", v);
///     *v += 1;
/// });
/// assert_eq!(v, 2);
/// ```
pub trait AlsoMut: Sized {
    /// Create an implicit variable, do some extra thing, and return it
    /// ## Example
    /// ```rust
    /// # use batch_oper::AlsoMut;
    /// let v = 1.also_mut(|v| {
    ///     println!("{}", v);
    ///     *v += 1;
    /// });
    /// assert_eq!(v, 2);
    /// ```
    fn also_mut<F: FnOnce(&mut Self)>(self, f: F) -> Self;
}
impl<T> AlsoMut for T {
    fn also_mut<F: FnOnce(&mut Self)>(mut self, f: F) -> Self {
        f(&mut self);
        self
    }
}
