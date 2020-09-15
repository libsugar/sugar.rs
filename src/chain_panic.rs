/// Panic Self
pub trait Panic: Sized + Sync + Send + 'static {
    #[inline]
    /// Panic Self
    fn panic(self) -> ! {
        panic!(self)
    }
}
impl<T: Sync + Send + 'static> Panic for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "123")]
    fn test_panic() {
        "123".panic();
    }
}
