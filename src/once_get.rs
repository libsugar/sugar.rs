//! ```
//! # use batch_oper::once_get::*;
//! let mut a = None;
//! let b = a.get_or_init(|| 1);
//! assert_eq!(*b, 1);
//! ```

/// Get Once
pub trait OnceGet<T> {
    /// Get ref, init it with f if was empty
    fn get_or_init<F: FnOnce() -> T>(&mut self, f: F) -> &T;
    /// Get mut ref, init it with f if was empty
    fn get_mut_or_init<F: FnOnce() -> T>(&mut self, f: F) -> &mut T;
}

impl<T> OnceGet<T> for Option<T> {
    fn get_or_init<F: FnOnce() -> T>(&mut self, f: F) -> &T {
        if let Some(v) = self {
            return v;
        }
        *self = Some(f());
        self.as_ref().unwrap()
    }

    fn get_mut_or_init<F: FnOnce() -> T>(&mut self, f: F) -> &mut T {
        if let Some(v) = self {
            return v;
        }
        *self = Some(f());
        self.as_mut().unwrap()
    }
}

#[test]
fn test() {
    let mut a = None;
    let b = a.get_or_init(|| 1);
    assert_eq!(*b, 1);
}
