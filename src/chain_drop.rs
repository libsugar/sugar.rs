//! Chain Drop

/// Drop self
pub trait Void {
    /// Drop self
    fn void(self);
}

impl<T> Void for T {
    fn void(self) {}
}
