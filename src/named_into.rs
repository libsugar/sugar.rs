macro_rules! named_into  {
    ( $n:ident; $($t:tt)* ) => {
        pub trait $n: Sized {
            $($t)*
        }
        impl<T> $n for T {}
    };
}

named_into! { IntoBox;
    #[inline(always)]
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

named_into! { IntoCell;
    #[inline(always)]
    fn cell(self) -> core::cell::Cell<Self> {
        core::cell::Cell::new(self)
    }
}
named_into! { IntoRefCell;
    #[inline(always)]
    fn ref_cell(self) -> core::cell::RefCell<Self> {
        core::cell::RefCell::new(self)
    }
}
named_into! { IntoUnsafeCell;
    #[inline(always)]
    unsafe fn unsafe_cell(self) -> core::cell::UnsafeCell<Self> {
        core::cell::UnsafeCell::new(self)
    }
}

#[cfg(feature = "std")]
named_into! { IntoRc;
    #[inline(always)]
    fn rc(self) -> std::rc::Rc<Self> {
        std::rc::Rc::new(self)
    }
    #[inline(always)]
    fn rc_refcell(self) -> std::rc::Rc<core::cell::RefCell<Self>> {
        std::rc::Rc::new(core::cell::RefCell::new(self))
    }
    #[inline(always)]
    fn rc_cell(self) -> std::rc::Rc<core::cell::Cell<Self>> {
        std::rc::Rc::new(core::cell::Cell::new(self))
    }
}

#[cfg(feature = "std")]
named_into! { IntoArc;
    #[inline(always)]
    fn arc(self) -> std::sync::Arc<Self> {
        std::sync::Arc::new(self)
    }
    #[inline(always)]
    fn arc_mutex(self) -> std::sync::Arc<std::sync::Mutex<Self>> {
        std::sync::Arc::new(std::sync::Mutex::new(self))
    }
    #[inline(always)]
    fn arc_rwlock(self) -> std::sync::Arc<std::sync::RwLock<Self>> {
        std::sync::Arc::new(std::sync::RwLock::new(self))
    }
}

#[cfg(feature = "std")]
named_into! { IntoMutex;
    #[inline(always)]
    fn mutex(self) -> std::sync::Mutex<Self> {
        std::sync::Mutex::new(self)
    }
}
#[cfg(feature = "std")]
named_into! { IntoRwLock;
    #[inline(always)]
    fn rwlock(self) -> std::sync::RwLock<Self> {
        std::sync::RwLock::new(self)
    }
}

pub trait IntoPin: Sized + core::ops::Deref {
    #[inline(always)]
    fn pin(self) -> core::pin::Pin<Self>
    where
        <Self as core::ops::Deref>::Target: Unpin,
    {
        core::pin::Pin::new(self)
    }
    unsafe fn pin_unchecked(self) -> core::pin::Pin<Self> {
        core::pin::Pin::new_unchecked(self)
    }
}
impl<T: core::ops::Deref> IntoPin for T {}

#[cfg(feature = "std")]
named_into! { IntoPinArc;
    #[inline(always)]
    fn pin_arc(self) -> core::pin::Pin<std::sync::Arc<Self>> {
        std::sync::Arc::pin(self)
    }
}

named_into! { IntoSome;
    #[inline(always)]
    fn some(self) -> core::option::Option<Self> {
        Some(self)
    }
}

pub trait IntoOk<E>: Sized {
    #[inline(always)]
    fn ok(self) -> core::result::Result<Self, E> {
        Ok(self)
    }
}
impl<T, E> IntoOk<E> for T {}
pub trait IntoErr<T>: Sized {
    #[inline(always)]
    fn err(self) -> core::result::Result<T, Self> {
        Err(self)
    }
}
impl<T, E> IntoErr<T> for E {}

pub trait IntoDuration {
    fn secs(self) -> core::time::Duration;
    fn millis(self) -> core::time::Duration;
    fn micros(self) -> core::time::Duration;
    fn nanos(self) -> core::time::Duration;
}
impl IntoDuration for u64 {
    #[inline(always)]
    fn secs(self) -> core::time::Duration {
        core::time::Duration::from_secs(self)
    }
    #[inline(always)]
    fn millis(self) -> core::time::Duration {
        core::time::Duration::from_millis(self)
    }
    #[inline(always)]
    fn micros(self) -> core::time::Duration {
        core::time::Duration::from_micros(self)
    }
    #[inline(always)]
    fn nanos(self) -> core::time::Duration {
        core::time::Duration::from_nanos(self)
    }
}

pub trait ToOk<E: Default> {
    type T;
    fn to_ok(self) -> Result<Self::T, E>;
}
impl<T, E: Default> ToOk<E> for Option<T> {
    type T = T;

    #[inline(always)]
    fn to_ok(self) -> Result<Self::T, E> {
        match self {
            Some(v) => Ok(v),
            None => Err(Default::default()),
        }
    }
}

pub trait ToErr<T: Default> {
    type E;
    fn to_err(self) -> Result<T, Self::E>;
}
impl<T: Default, E> ToErr<T> for Option<E> {
    type E = E;

    #[inline(always)]
    fn to_err(self) -> Result<T, Self::E> {
        match self {
            Some(v) => Err(v),
            None => Ok(Default::default()),
        }
    }
}

