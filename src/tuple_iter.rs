use std::mem::MaybeUninit;
use std::iter::FusedIterator;
use std::cmp::min;

include!(concat!(env!("OUT_DIR"), "/tuple_iter.rs"));

pub trait TupleIter<'a> {
    type Iter: Iterator + 'a;

    fn iter(&'a self) -> Self::Iter;
}

pub trait TupleIntoIter {
    type Iter: Iterator;

    fn into_iter(self) -> Self::Iter;
}

pub trait TupleFromIter<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self;
}
