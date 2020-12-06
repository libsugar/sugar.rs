/// new a `Box<T>`  
/// ```
/// # use batch_oper::*;
/// # let xxx = 1;
/// new!(xxx)
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # let xxx = 1;
/// Box::new(xxx)
/// # ;
/// ```
#[macro_export]
macro_rules! new {
    () => {
        Box::new(Default::default())
    };
    ($e:expr) => {
        Box::new($e)
    };
}

/// new a `Box<[T]>`  
/// ```
/// # use batch_oper::*;
/// # let a = 1; let b = 2; let c = 3;
/// arr![a, b, c]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # let a = 1; let b = 2; let c = 3;
/// Box::new([a, b, c])
/// # ;
/// ```
#[macro_export]
macro_rules! arr {
    [ $($e:expr),* $(,)? ] => { Box::new([$($e),*]) };
}

/// new a `VecDeque<T>`
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1; let b = 2; let c = 3;
/// deque![a, b, c]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1; let b = 2; let c = 3;
/// VecDeque::from(vec![a, b, c])
/// # ;
/// ```
#[macro_export]
macro_rules! deque {
    [] => { std::collections::VecDeque::new() };
    [ $elem:expr; $n:expr ] => { std::collections::VecDeque::from(vec![$elem; $n]) };
    [ $($e:expr),* $(,)? ] => { std::collections::VecDeque::from(vec![$($e),+]) };
}

/// new a `LinkedList<T>`  
///
/// ----------  
/// ### Push Back  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1; let b = 2; let c = 3;
/// list![a, b, c]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1; let b = 2; let c = 3;
/// {
///     let mut l = LinkedList::new();
///     l.push_back(a);
///     l.push_back(b);
///     l.push_back(c);
///     l
/// }
///
/// # ;
/// ```
///   
/// ----------  
/// ### Push Front  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1; let b = 2; let c = 3;
/// list![<- a, b, c]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1; let b = 2; let c = 3;
/// {
///     let mut l = LinkedList::new();
///     l.push_front(a);
///     l.push_front(b);
///     l.push_front(c);
///     l
/// }
///
/// # ;
/// ```
///   
/// ----------  
/// ### From Elem  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1;
/// list![a; 3]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1;
/// {
///     let mut l = LinkedList::new();
///     for _ in 0..3 {
///         l.push_back(a);
///     }
///     l
/// }
///
/// # ;
/// ```
#[macro_export]
macro_rules! list {
    [] => { std::collections::LinkedList::new() };
    [ $elem:expr; $n:expr ] => {{
        let mut l = std::collections::LinkedList::new();
        for _ in 0..$n {
            l.push_back($elem);
        }
        l
    }};
    [ $($e:expr),* $(,)? ] => {{
        let mut l = std::collections::LinkedList::new();
        $( l.push_back($e); )*
        l
    }};
    [ <- $($e:expr),* $(,)? ] => {{
        let mut l = std::collections::LinkedList::new();
        $( l.push_front($e); )*
        l
    }};
}

/// new a `HashMap<K, V>`  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// map! {
///     ka => va,
///     kb => vb,
/// }
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// {
///     let mut m = HashMap::new();
///     m.insert(ka, va);
///     m.insert(kb, vb);
///     m
/// }
/// # ;
/// ```
/// ---
/// map like
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// map! { let BTreeMap::new();
///     ka => va,
///     kb => vb,
/// }
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// {
///     let mut m = BTreeMap::new();
///     m.insert(ka, va);
///     m.insert(kb, vb);
///     m
/// }
/// # ;
/// ```
#[macro_export]
macro_rules! map {
    { } => { std::collections::HashMap::new() };
    { $($k:expr => $v:expr),* $(,)? } => {{
        let mut m = std::collections::HashMap::new();
        $(
            m.insert($k, $v);
        )*
        m
    }};
    { let $m:expr; $($k:expr => $v:expr),* $(,)? } => {{
        let mut m = $m;
        $(
            m.insert($k, $v);
        )*
        m
    }};
}

/// new a `BTreeMap<K, V>`  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// btmap! {
///     ka => va,
///     kb => vb,
/// }
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// {
///     let mut m = BTreeMap::new();
///     m.insert(ka, va);
///     m.insert(kb, vb);
///     m
/// }
/// # ;
/// ```
#[macro_export]
macro_rules! btmap {
    { } => { std::collections::BTreeMap::new() };
    { $($k:expr => $v:expr),* $(,)? } => {{
        let mut m = std::collections::BTreeMap::new();
        $(
            m.insert($k, $v);
        )*
        m
    }};
}

/// append items to a map  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// let mut m = HashMap::new();
/// map_append! { m;
///     ka => va,
///     kb => vb,
/// }
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// let mut m = HashMap::new();
/// m.insert(ka, va);
/// m.insert(kb, vb);
/// # ;
/// ```
#[macro_export]
macro_rules! map_append {
    { $m:expr; $($k:expr => $v:expr),* $(,)? } => {
        $(
            $m.insert($k, $v);
        )*
    };
}

/// new a `HashSet<V>`  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// set![a, b]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// {
///     let mut s = HashSet::new();
///     s.insert(a);
///     s.insert(b);
///     s
/// }
/// # ;
/// ```
/// ---
/// set like
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// set![let BTreeSet::new(); 1, 2]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// {
///     let mut s = BTreeSet::new();
///     s.insert(a);
///     s.insert(b);
///     s
/// }
/// # ;
/// ```
#[macro_export]
macro_rules! set {
    { } => { std::collections::HashSet::new() };
    { $($e:expr),* $(,)? } => {{
        let mut s = std::collections::HashSet::new();
        $(
            s.insert($e);
        )*
        s
    }};
    { let $s:expr; $($e:expr),* $(,)? } => {{
        let mut s = $s;
        $(
            s.insert($e);
        )*
        s
    }};
}

/// new a `BTreeSet<V>`  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// btset![a, b]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// {
///     let mut s = BTreeSet::new();
///     s.insert(a);
///     s.insert(b);
///     s
/// }
/// # ;
/// ```
#[macro_export]
macro_rules! btset {
    { } => { std::collections::BTreeSet::new() };
    { $($e:expr),* $(,)? } => {{
        let mut s = std::collections::BTreeSet::new();
        $(
            s.insert($e);
        )*
        s
    }};
}

/// new a `BinaryHeap<V>`  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// heap![a, b]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// BinaryHeap::from(vec![a, b])
/// # ;
/// ```
#[macro_export]
macro_rules! heap {
    [ ] => { std::collections::BinaryHeap::new() };
    [ $elem:expr; $n:expr ] => { std::collections::BinaryHeap::from(vec![$elem; $n]) };
    [ $($e:expr),+ $(,)? ] => { std::collections::BinaryHeap::from(vec![$($e),+]) };
}