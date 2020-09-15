use std::fmt::Display;

/// Chain call version of `todo!()`
pub trait Todo {
    #[inline]
    /// Chain call version of `todo!()`
    fn todo(&self) {
        todo!()
    }
}
impl<T> Todo for T {}

/// Chain call version of `todo!(msg)`
pub trait TodoMsg {
    #[inline]
    /// Chain call version of `todo!(msg)`
    fn todo_msg<T: Display>(&self, msg: T) {
        todo!("{}", msg)
    }
}
impl<T> TodoMsg for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_todo() {
        1.todo();
    }

    #[test]
    #[should_panic]
    fn test_todo_msg() {
        1.todo_msg("asd");
    }
}
