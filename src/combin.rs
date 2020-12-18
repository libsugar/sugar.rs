//! No nesting combine
//!
//! # Tuple example
//!
//! ```
//! # use libsugar::combin::*;
//! let a: (i32, u8) = 1.with(2u8);
//! assert_eq!(a, (1, 2));
//! ```

/// Make A Tuple  
/// `A.after(B) -> (B, A)`
pub trait After<T, Output> {
    /// Make A Tuple  
/// `A.after(B) -> (B, A)`
    fn after(self, v: T) -> Output;
}

impl<S, T> After<T, (T, S)> for S {
    fn after(self, v: T) -> (T, S) {
        (v, self)
    }
}

/// Make A Tuple  
/// `A.after(B) -> (A, B)`
pub trait With<T, Output> {
    /// Make A Tuple  
    /// `A.after(B) -> (A, B)`
    fn with(self, v: T) -> Output;
}

impl<S, T> With<T, (S, T)> for S {
    fn with(self, v: T) -> (S, T) {
        (self, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combin_tuple() {
        let a: (i32, u8) = 1.with(2u8);
        assert_eq!(a, (1, 2));
    }

}
