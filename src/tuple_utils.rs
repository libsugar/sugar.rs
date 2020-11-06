//! Utility tools for tuples

include!(concat!(env!("OUT_DIR"), "/tuple_utils.rs"));

/// AsRef for Tuple
pub trait TupleAsRef<'a> {
    type OutTuple: 'a;

    /// AsRef for Tuple
    fn as_ref(&'a self) -> Self::OutTuple;
}

/// AsMut for Tuple
pub trait TupleAsMut<'a> {
    type OutTuple: 'a;

    /// AsMut for Tuple
    fn as_mut(&'a mut self) -> Self::OutTuple;
}

/// Tuple meta
pub trait Tuple {
    /// Tuple size
    fn size(&self) -> usize;
}

/// Mark traits for all tuples with all item is same type
pub trait TupleSame<T>: Tuple { }
