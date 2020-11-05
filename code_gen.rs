use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use std::{ffi::OsString, fs, path::Path};
use syn::LitInt;

pub fn code_gen(out_dir: OsString) {
    #[cfg(feature = "tuple_iter")]
    gen_tuple_iter(&out_dir);
    #[cfg(feature = "tuple_utils")]
    gen_tuple_map(&out_dir);
}

#[cfg(feature = "tuple_iter")]
fn gen_tuple_iter(out_dir: &OsString) {
    let items = (2..33usize).into_iter().map(gen_tuple_iter_size);
    let tks = quote! { #(#items)* };
    let code = tks.to_string();
    let dest_path = Path::new(out_dir).join("tuple_iter.rs");
    fs::write(&dest_path, code).unwrap();
}

#[cfg(feature = "tuple_iter")]
fn gen_tuple_iter_size(size: usize) -> TokenStream {
    let size_lit = LitInt::new(size.to_string().as_str(), Span::call_site());
    let last_lit = LitInt::new((size - 1).to_string().as_str(), Span::call_site());
    let iter_struct_name = format_ident!("Tuple{}Iter", size);
    let into_iter_struct_name = format_ident!("Tuple{}IntoIter", size);
    let t = format_ident!("T");
    let ts = (0..size).into_iter().map(|_| &t).collect::<Vec<_>>();
    let ut = quote! { MaybeUninit<T> };
    let uts = (0..size).into_iter().map(|_| &ut);
    let utts = (0..size)
        .into_iter()
        .map(|i| LitInt::new(i.to_string().as_str(), Span::call_site()))
        .map(|i| quote! { MaybeUninit::new(t.#i) });
    let iter_match = (0..size)
        .into_iter()
        .map(|i| LitInt::new(i.to_string().as_str(), Span::call_site()))
        .map(|i| quote! { #i => &self.1 .#i, })
        .collect::<Vec<_>>();
    let into_match = (0..size)
        .into_iter()
        .map(|i| {
            (
                LitInt::new(i.to_string().as_str(), Span::call_site()),
                LitInt::new((i + 1).to_string().as_str(), Span::call_site()),
            )
        })
        .map(|(i, n)| quote! { #i => std::mem::replace(&mut self.#n, MaybeUninit::uninit()), })
        .collect::<Vec<_>>();
    let from = quote! { iter.next().unwrap() };
    let froms = (0..size).into_iter().map(|_| &from);

    let derive_iter = if size > 12 {
        quote! {}
    } else {
        quote! {#[derive(Debug, Clone, Copy)]}
    };
    let derive_into = if size > 12 {
        quote! {}
    } else {
        quote! {#[derive(Debug)]}
    };

    let tks = quote! {
        #derive_iter
        #[doc(hidden)]
        pub struct #iter_struct_name<'a, T>(usize, &'a (#(#ts),*));
        impl<'a, T> #iter_struct_name<'a, T> {
            #[inline]
            pub fn new(t: &'a (#(#ts),*)) -> Self {
                Self(0, t)
            }
        }
        #derive_into
        #[doc(hidden)]
        pub struct #into_iter_struct_name<T>(usize, #(#uts),*);
        impl<T> #into_iter_struct_name<T> {
            #[inline]
            pub fn new(t: (#(#ts),*)) -> Self {
                Self(0, #(#utts),*)
            }
        }

        impl<'a, T> Iterator for #iter_struct_name<'a, T> {
            type Item = &'a T;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                let res = match self.0 {
                    #(#iter_match)*
                    _ => return None
                };
                self.0 += 1;
                Some(res)
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let exact = self.len();
                (exact, Some(exact))
            }

            #[inline]
            fn count(self) -> usize {
                self.len()
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                if n < self.0 { return None }
                let res = match self.0 {
                    #(#iter_match)*
                    _ => return None
                };
                self.0 = min(n + 1, #size_lit);
                Some(res)
            }

            #[inline]
            fn last(self) -> Option<Self::Item> {
                if self.len() == 0 { return None }
                Some(&self.1 .#last_lit)
            }
        }
        impl<'a, T> ExactSizeIterator for #iter_struct_name<'a, T> {
            #[inline]
            fn len(&self) -> usize { #size_lit - self.0 }
        }
        impl<'a, T> FusedIterator for #iter_struct_name<'a, T> { }
        impl<'a, T> AsRef<(#(#ts),*)> for #iter_struct_name<'a, T> {
            #[inline]
            fn as_ref(&self) -> &(#(#ts),*) {
                self.1
            }
        }
        impl<'a, T: 'a> TupleIter<'a> for (#(#ts),*) {
            type Iter = #iter_struct_name<'a, T>;

            #[inline]
            fn iter(&'a self) -> Self::Iter {
                #iter_struct_name::new(self)
            }
        }

        impl<T> Iterator for #into_iter_struct_name<T> {
            type Item = T;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                let res = match self.0 {
                    #(#into_match)*
                    _ => return None
                };
                self.0 += 1;
                Some(unsafe { res.assume_init() })
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let exact = self.len();
                (exact, Some(exact))
            }

            #[inline]
            fn count(self) -> usize {
                self.len()
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                if n < self.0 { return None }
                let res = match self.0 {
                    #(#into_match)*
                    _ => return None
                };
                self.0 = min(n + 1, #size_lit);
                Some(unsafe { res.assume_init() })
            }

            #[inline]
            fn last(self) -> Option<Self::Item> {
                if self.len() == 0 { return None }
                Some(unsafe { self.#size_lit.assume_init() })
            }
        }
        impl<T> ExactSizeIterator for #into_iter_struct_name<T> {
            #[inline]
            fn len(&self) -> usize { #size_lit - self.0 }
        }
        impl<T> FusedIterator for #into_iter_struct_name<T> { }
        impl<T> TupleIntoIter for (#(#ts),*) {
            type Iter = #into_iter_struct_name<T>;

            #[inline]
            fn into_iter(self) -> Self::Iter {
                #into_iter_struct_name::new(self)
            }
        }

        impl<T> TupleFromIter<T> for (#(#ts),*) {
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
                let mut iter = iter.into_iter();
                (#(#froms),*)
            }
        }
    };
    tks
}

#[cfg(feature = "tuple_utils")]
fn gen_tuple_map(out_dir: &OsString) {
    let items = (2..33usize).into_iter().map(gen_tuple_map_size);
    let tks = quote! { #(#items)* };
    let code = tks.to_string();
    let dest_path = Path::new(out_dir).join("tuple_utils.rs");
    fs::write(&dest_path, code).unwrap();
}

#[cfg(feature = "tuple_utils")]
fn gen_tuple_map_size(size: usize) -> TokenStream {
    let items = if size > 16 { vec![] } else { (0..size).into_iter().map(|n| gen_tuple_map_n_size(size, n)).collect() };

    let as_ref_name = format_ident!("Tuple{}AsRef", size);
    let as_mut_name = format_ident!("Tuple{}AsMut", size);
    let map_name = format_ident!("Tuple{}Map", size);

    let t = format_ident!("T");
    let u = format_ident!("U");

    let ts = (0..size).into_iter().map(|_| &t).collect::<Vec<_>>();
    let us = (0..size).into_iter().map(|_| &u).collect::<Vec<_>>();
    let nts = (0..size)
        .into_iter()
        .map(|i| format_ident!("T{}", i))
        .collect::<Vec<_>>();
    let ref_nts = (0..size)
        .into_iter()
        .map(|i| &nts[i])
        .map(|id| quote! { &#id })
        .collect::<Vec<_>>();
    let mut_nts = (0..size)
        .into_iter()
        .map(|i| &nts[i])
        .map(|id| quote! { &mut #id })
        .collect::<Vec<_>>();

    let ref_impl = (0..size)
        .into_iter()
        .map(|i| LitInt::new(i.to_string().as_str(), Span::call_site()))
        .map(|l| {
            quote! {
                &self.#l
            }
        });
    let mut_impl = (0..size)
        .into_iter()
        .map(|i| LitInt::new(i.to_string().as_str(), Span::call_site()))
        .map(|l| {
            quote! {
                &mut self.#l
            }
        });
    let map_impl = (0..size)
        .into_iter()
        .map(|i| LitInt::new(i.to_string().as_str(), Span::call_site()))
        .map(|l| {
            quote! {
                f(self.#l)
            }
        });

    let ref_doc = format!("AsRef for Tuple{}", size);
    let mut_doc = format!("AsMut for Tuple{}", size);
    let map_doc = format!("Mapping for Tuple{}", size);

    let tks = quote! {
        #(#items)*

        #[doc = #ref_doc]
        pub trait #as_ref_name<#(#nts),*> {
            #[doc = #ref_doc]
            fn as_ref(&self) -> (#(#ref_nts),*);
        }
        impl<#(#nts),*> #as_ref_name<#(#nts),*> for (#(#nts),*) {
            fn as_ref(&self) -> (#(#ref_nts),*) {
                (#(#ref_impl),*)
            }
        }

        #[doc = #mut_doc]
        pub trait #as_mut_name<#(#nts),*> {
            #[doc = #mut_doc]
            fn as_mut(&mut self) -> (#(#mut_nts),*);
        }
        impl<#(#nts),*> #as_mut_name<#(#nts),*> for (#(#nts),*) {
            fn as_mut(&mut self) -> (#(#mut_nts),*) {
                (#(#mut_impl),*)
            }
        }

        #[doc = #map_doc]
        pub trait #map_name<T> {
            #[doc = #map_doc]
            fn map<U>(self, f: impl FnMut(T) -> U) -> (#(#us),*);
        }
        impl<T> #map_name<T> for (#(#ts),*) {
            fn map<U>(self, mut f: impl FnMut(T) -> U) -> (#(#us),*) {
                (#(#map_impl),*)
            }
        }
    };
    tks
}

#[cfg(feature = "tuple_utils")]
fn gen_tuple_map_n_size(size: usize, n: usize) -> TokenStream {
    let t = format_ident!("T{}", n);
    let map_n_name = format_ident!("Tuple{}Map{}", size, n);
    let map_n = format_ident!("map{}", n);

    let rts = (0..size)
        .into_iter()
        .map(|i| format_ident!("T{}", i))
        .collect::<Vec<_>>();
    let ts = (0..size)
        .into_iter()
        .map(|i| {
            if i == n {
                format_ident!("U")
            } else {
                format_ident!("T{}", i)
            }
        })
        .collect::<Vec<_>>();

    let impls = (0..size)
        .into_iter()
        .map(|i| (i, LitInt::new(i.to_string().as_str(), Span::call_site())))
        .map(|(i, l)| {
            if i == n {
                quote! { f(self.#l) }
            } else {
                quote! { self.#l }
            }
        });

    let doc = format!("Mapping `.{}` for Tuple{}", n, size);

    let tks = quote! {
        #[doc=#doc]
        pub trait #map_n_name<#(#rts),*> {
            #[doc=#doc]
            fn #map_n<U>(self, f: impl FnOnce(#t) -> U) -> (#(#ts),*);
        }
        impl<#(#rts),*> #map_n_name<#(#rts),*> for (#(#rts),*) {
            fn #map_n<U>(self, f: impl FnOnce(#t) -> U) -> (#(#ts),*) {
                (#(#impls),*)
            }
        }
    };
    tks
}
