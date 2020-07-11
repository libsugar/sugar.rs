//! No nesting combine
//!
//! # Tuple example
//!
//! ```
//! # use batch_oper::combin::*;
//! let a: (i32, u8) = 1.with(2u8);
//! assert_eq!(a, (1, 2));
//!
//! let b: (i32, u8, f64) = a.with(3f64);
//! assert_eq!(b, (1, 2, 3.0));
//!
//! let c: (usize, i32, u8, f64) = b.after(0usize);
//! assert_eq!(c, (0, 1, 2, 3.0));
//! ```
//! ```ignore
//! // feature = "combin-mutual"
//! let m: (u8, u8, u8, u8) = (1, 2).with((3, 4));
//! assert_eq!(m, (1, 2, 3, 4));
//! ```
//!
//! # Array example
//!
//! ```
//! # use batch_oper::combin::*;
//! let a: [u8; 2] = 1.with(2);
//! assert_eq!(a, [1, 2]);
//! 
//! let b: [u8; 3] = a.with(3);
//! assert_eq!(b, [1, 2, 3]);
//! 
//! let c: [u8; 4] = b.after(0);
//! assert_eq!(c, [0, 1, 2, 3]);
//! ```
//! ```ignore
//! // feature = "combin-mutual"
//! let m: [u8; 4] = [1, 2].with([3, 4]);
//! assert_eq!(m, [1, 2, 3, 4]);
//! ```
//!
//! *[With](trait.With.html) is a alias for [Before](trait.Before.html)*
#![allow(unused_macros)]
/// No nesting combine  
/// Add at the end  
pub trait After<T, Output> {
    /// No nesting combine  
    /// Add at the end
    fn after(self, v: T) -> Output;
}
/// No nesting combine
pub trait Before<T, Output> {
    /// No nesting combine
    fn before(self, v: T) -> Output;
}

/// No nesting combine  
/// Same to [Before](trait.Before.html)  
pub trait With<T, Output> {
    /// No nesting combine  
    /// Same to [Before](trait.Before.html)  
    fn with(self, v: T) -> Output;
}
impl<A: Before<B, O>, B, O> With<B, O> for A {
    #[inline(always)]
    fn with(self, v: B) -> O {
        self.before(v)
    }
}

macro_rules! do_impl_tuple_old {
    { } => { };
    { $($t:ident),* } => {
        do_impl_tuple_old! { , $($t),* }
        do_impl_tuple_old! { ; $($t),* }
    };
    { , $h:ident $(, $($t:ident),*)? } => {
        do_impl_tuple_old! { $($($t),*)? }
    };
    { ; $($t:ident),* } => {
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        #[allow(non_snake_case)]
        impl<$($t),*, Ta> After<Ta, (Ta, $($t),*)> for ($($t),*) {
            #[inline(always)]
            fn after(self, v: Ta) -> (Ta, $($t),*) {
                let ($($t),*) = self;
                (v, $($t),*)
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        #[allow(non_snake_case)]
        impl<$($t),*, Ta> Before<Ta, ($($t),*, Ta)> for ($($t),*) {
            #[inline(always)]
            fn before(self, v: Ta) -> ($($t),*, Ta) {
                let ($($t),*) = self;
                ($($t),*, v)
            }
        }
    };
}
do_impl_tuple_old! {
    Z, Y, X, W, V, U, T, S, R, Q, P, O, N, M, L, K, J, I, H, G, F, E, D, C, B, A,
    z, y, x, w, v, u, t, s, r, q, p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a
}

#[cfg(feature = "combin-mutual")]
mod mutual_tuple {
    use super::*;
    
    macro_rules! do_impl_tuple {
        { $($o:ident),* ; $h:ident } => { };
        { ; $($s:ident),* } => { };
        { $($o:ident),* ; $($s:ident),* } => {
            do_impl_tuple_loop!{ $($o),* ; $($s),* }
            do_impl_tuple_loop_2!{ $($o),* ; $($s),* }
            do_impl_tuple_impl!{ $($o),* ; $($s),* }
        };
    }
    macro_rules! do_impl_tuple_loop {
        { $h:ident $(, $($o:ident),*)? ; $($s:ident),* } => {
            do_impl_tuple!{ $($($o),*)? ; $($s),* }
        };
    }
    macro_rules! do_impl_tuple_2 {
        { $($o:ident),* ; $h:ident } => { };
        { ; $($s:ident),* } => { };
        { $($o:ident),* ; $($s:ident),* } => {
            do_impl_tuple_loop_2!{ $($o),* ; $($s),* }
            do_impl_tuple_impl!{ $($o),* ; $($s),* }
        };
    }
    macro_rules! do_impl_tuple_loop_2 {
        { $($o:ident),* ; $h:ident $(, $($s:ident),*)? } => {
            do_impl_tuple_2!{ $($o),* ; $($($s),*)? }
        };
    }
    macro_rules! do_impl_tuple_impl {
        { $($t:ident),* ; $($s:ident),* } => {
            #[allow(non_camel_case_types)]
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<$($t),*, $($s),*> After<($($s),*), ($($s),*, $($t),*)> for ($($t),*) {
                #[inline(always)]
                fn after(self, v: ($($s),*)) -> ($($s),*, $($t),*) {
                    let ($($t),*) = self;
                    let ($($s),*) = v;
                    ($($s),*, $($t),*)
                }
            }
            #[allow(non_camel_case_types)]
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<$($t),*, $($s),*> Before<($($s),*), ($($t),*, $($s),*)> for ($($t),*) {
                #[inline(always)]
                fn before(self, v: ($($s),*)) -> ($($t),*, $($s),*) {
                    let ($($t),*) = self;
                    let ($($s),*) = v;
                    ($($t),*, $($s),*)
                }
            }
        };
    }
    do_impl_tuple! {
        z, y, x, w, v, u, t, s, r, q, p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a;
        Tz, Ty, Tx, Tw, Tv, Tu, Tt, Ts, Tr, Tq, Tp, To, Tn, Tm, Tl, Tk, Tj, Ti, Th, Tg, Tf, Te, Td, Tc, Tb, Ta
    }
}
#[cfg(feature = "combin-mutual")]
pub use mutual_tuple::*;

macro_rules! do_impl_array_old {
    { } => { };
    { : $i:ident } => { 1 };
    { $($t:ident),* } => {
        do_impl_array_old! { , $($t),* }
        do_impl_array_old! { ; $($t),* }
    };
    { , $h:ident $(, $($t:ident),*)? } => {
        do_impl_array_old! { $($($t),*)? }
    };
    { ; $($t:ident),* } => {
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        #[allow(non_snake_case)]
        impl<T> After<T, [T; 1 + $(do_impl_array_old!(:$t) + )* 0 ]> for [T; $(do_impl_array_old!(:$t) + )* 0 ] {
            #[inline(always)]
            fn after(self, v: T) -> [T; 1 + $(do_impl_array_old!(:$t) + )* 0 ] {
                let [$($t),*] = self;
                [v, $($t),*]
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        #[allow(non_snake_case)]
        impl<T> Before<T, [T; 1 + $(do_impl_array_old!(:$t) + )* 0 ]> for [T; $(do_impl_array_old!(:$t) + )* 0 ] {
            #[inline(always)]
            fn before(self, v: T) -> [T; 1 + $(do_impl_array_old!(:$t) + )* 0 ] {
                let [$($t),*] = self;
                [$($t),*, v]
            }
        }
    };
}
do_impl_array_old! {
    z, y, x, w, v, u, t, s, r, q, p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a,
    Z, Y, X, W, V, U, T, S, R, Q, P, O, N, M, L, K, J, I, H, G, F, E, D, C, B, A
}

macro_rules! do_impl_array_one {
    { $i:ident } => { 1 }
}

#[cfg(feature = "combin-mutual")]
mod mutual_array {
    use super::*;

    macro_rules! do_impl_array_one {
        { $i:ident } => { 1 }
    }
    macro_rules! do_impl_array {
        { $($o:ident),* ; $h:ident } => { };
        { ; $($s:ident),* } => { };
        { $($o:ident),* ; $($s:ident),* } => {
            do_impl_array_loop!{ $($o),* ; $($s),* }
            do_impl_array_loop_2!{ $($o),* ; $($s),* }
            do_impl_array_impl!{ $($o),* ; $($s),* }
        };
    }
    macro_rules! do_impl_array_loop {
        { $h:ident $(, $($o:ident),*)? ; $($s:ident),* } => {
            do_impl_array!{ $($($o),*)? ; $($s),* }
        };
    }
    macro_rules! do_impl_array_2 {
        { $($o:ident),* ; $h:ident } => { };
        { ; $($s:ident),* } => { };
        { $($o:ident),* ; $($s:ident),* } => {
            do_impl_array_loop_2!{ $($o),* ; $($s),* }
            do_impl_array_impl!{ $($o),* ; $($s),* }
        };
    }
    macro_rules! do_impl_array_loop_2 {
        { $($o:ident),* ; $h:ident $(, $($s:ident),*)? } => {
            do_impl_array_2!{ $($o),* ; $($($s),*)? }
        };
    }
    macro_rules! do_impl_array_impl {
        { $($t:ident),* ; $($s:ident),* } => {
            #[allow(non_camel_case_types)]
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<T> After<[T; $(do_impl_array_one!($s) +)* 0], [T; $(do_impl_array_one!($t) +)* $(do_impl_array_one!($s) +)* 0]> for [T; $(do_impl_array_one!($t) +)* 0] {
                #[inline(always)]
                fn after(self, v: [T; $(do_impl_array_one!($s) +)* 0]) -> [T; $(do_impl_array_one!($t) +)* $(do_impl_array_one!($s) +)* 0] {
                    let [$($t),*] = self;
                    let [$($s),*] = v;
                    [$($s),*, $($t),*]
                }
            }
            #[allow(non_camel_case_types)]
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<T> Before<[T; $(do_impl_array_one!($s) +)* 0], [T; $(do_impl_array_one!($t) +)* $(do_impl_array_one!($s) +)* 0]> for  [T; $(do_impl_array_one!($t) +)* 0] {
                #[inline(always)]
                fn before(self, v: [T; $(do_impl_array_one!($s) +)* 0]) ->  [T; $(do_impl_array_one!($t) +)* $(do_impl_array_one!($s) +)* 0] {
                    let [$($t),*] = self;
                    let [$($s),*] = v;
                    [$($t),*, $($s),*]
                }
            }
        };
    }
    do_impl_array! {
        z, y, x, w, v, u, t, s, r, q, p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a;
        Tz, Ty, Tx, Tw, Tv, Tu, Tt, Ts, Tr, Tq, Tp, To, Tn, Tm, Tl, Tk, Tj, Ti, Th, Tg, Tf, Te, Td, Tc, Tb, Ta
    }

    macro_rules! do_impl_array_t {
    { } => { };
    { $($t:ident),* } => {
        do_impl_array_t! { , $($t),* }
        do_impl_array_t! { ; $($t),* }
    };
    { , $h:ident $(, $($t:ident),*)? } => {
        do_impl_array_t! { $($($t),*)? }
    };
    { ; $($s:ident),* } => {
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        #[allow(non_snake_case)]
        impl<T> After<[T; $(do_impl_array_one!($s) +)* 0], [T; $(do_impl_array_one!($s) +)* 1]> for T {
            fn after(self, v: [T; $(do_impl_array_one!($s) +)* 0]) -> [T; $(do_impl_array_one!($s) +)* 1] {
                let [$($s),*] = v;
                [$($s),*, self]
            }
        }
        impl<T> Before<[T; $(do_impl_array_one!($s) +)* 0], [T; $(do_impl_array_one!($s) +)* 1]> for T {
            fn before(self, v: [T; $(do_impl_array_one!($s) +)* 0]) -> [T; $(do_impl_array_one!($s) +)* 1] {
                let [$($s),*] = v;
                [self, $($s),*]
            }
        }
    };
}
do_impl_array_t! {
    z, y, x, w, v, u, t, s, r, q, p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a
}
}
#[cfg(feature = "combin-mutual")]
pub use mutual_array::*;

impl<T> After<T, [T; 1 + 1]> for T {
    fn after(self, v: T) -> [T; 1 + 1] {
        [v, self]
    }
}
impl<T> Before<T, [T; 1 + 1]> for T {
    fn before(self, v: T) -> [T; 1 + 1] {
        [self, v]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combin_tuple() {
        let a: (i32, u8) = 1.with(2u8);
        assert_eq!(a, (1, 2));
        let b: (i32, u8, f64) = a.with(3f64);
        assert_eq!(b, (1, 2, 3.0));
        let c: (usize, i32, u8, f64) = b.after(0usize);
        assert_eq!(c, (0, 1, 2, 3.0));
    }

    #[test]
    #[cfg(feature = "combin-mutual")]
    fn test_combin_tuple_2() {
        let a: (u8, u8, u8) = 1.with((2, 3));
        assert_eq!(a, (1, 2, 3));
    }

    #[test]
    #[cfg(feature = "combin-mutual")]
    fn test_combin_tuple_m() {
        let a: (u8, u8, u8, u8) = (1, 2).with((3, 4));
        assert_eq!(a, (1, 2, 3, 4));
        let b: (u8, u8, u8, u8) = (3, 4).after((1, 2));
        assert_eq!(b, (1, 2, 3, 4));
    }

    #[test]
    fn test_combin_array() {
        let a: [u8; 2] = 1.with(2);
        assert_eq!(a, [1, 2]);
        let b: [u8; 3] = a.with(3);
        assert_eq!(b, [1, 2, 3]);
        let c: [u8; 4] = b.after(0);
        assert_eq!(c, [0, 1, 2, 3]);
    }

    #[test]
    #[cfg(feature = "combin-mutual")]
    fn test_combin_array_2() {
        let a: [u8; 3] = 1.with([2, 3]);
        assert_eq!(a, [1, 2, 3]);
    }

    #[test]
    #[cfg(feature = "combin-mutual")]
    fn test_combin_array_m() {
        let a: [u8; 4] = [1, 2].with([3, 4]);
        assert_eq!(a, [1, 2, 3, 4]);
        let b: [u8; 4] = [3, 4].after([1, 2]);
        assert_eq!(b, [1, 2, 3, 4]);
    }
}
