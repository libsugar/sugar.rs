//! batch_oper provides some batch operation macro for some operations
//! ## Usage
//! - **Basic**  
//!   - batch `||`  
//!     ```rust  
//!     bop!(|| 4; == 2, > 3);
//!     ```
//!     *equivalent to*
//!     ```rust
//!     4 == 2 || 4 > 3
//!     ```
//!   - batch `&&`  
//!     ```rust  
//!     bop!(&& 4; == 2, > 3);
//!     ```
//!     *equivalent to*
//!     ```rust
//!     4 == 2 && 4 > 3
//!     ```
//!   - `!`
//!     ```rust
//!     bop!(|| a; == 1;!, == 2);
//!     ```
//!     *equivalent to*
//!     ```rust
//!     1 == a || a == 2
//!     ```
//! - **Set**
//!   ```rust
//!   let mut a = 1;
//!   bop!(= a; + 1, - 2;!, * 3);
//!   ```
//!   *equivalent to*
//!   ```rust
//!   let mut a = 1;
//!   a = a + 1;
//!   a = 2 - a;
//!   a = a * 3;
//!   ```
//! - **Let**
//!   ```rust
//!   bop! { let a|u8 = 1, mut b = 2 }
//!   ```
//!   *equivalent to*
//!   ```rust
//!   let a: u8 = 1;
//!   let mut b = 2;
//!   ```
//! - **Let chain**
//!   - basic
//!     ```rust
//!     let a = Some(1);
//!     let b = Some(2);
//!     
//!     let _: i32 = bop!(match && Some(va) = a, Some(vb) = b => {
//!         1
//!     } else {
//!         2
//!     });
//!     ```
//!     *equivalent to*
//!     ```rust
//!     let a = Some(1);
//!     let b = Some(2);
//!     
//!     let _: i32 = loop {
//!         if let Some(va) = a {
//!             if let Some(vb) = b {
//!                 break { 1 };
//!             }
//!         }
//!         break { 2 };
//!     };
//!     ```
//!   - `bool`
//!     ```rust
//!     let _: bool = bop!(bool match && Some(va) = a, Some(vb) = b => {
//!         1
//!     } else {
//!         2
//!     });
//!     ```
//!     *equivalent to*
//!     ```rust
//!     let _: bool = loop {
//!         if let Some(va) = a {
//!             if let Some(vb) = b {
//!                 { 1 };
//!                 break true;
//!             }
//!         }
//!         { 2 };
//!         break false;
//!     };
//!     ```
//!   - `!loop`
//!     ```rust
//!     let _: i32 = bop!(!loop match && Some(va) = a, Some(vb) = b => {
//!         1
//!     } else {
//!         2
//!     });
//!     ```
//!     *equivalent to*
//!     ```rust
//!     let _: i32 = if let Some(va) = a {
//!         if let Some(vb) = b {
//!             { 1 }
//!         } else { { 2 } }
//!     } else  { { 2 } }
//!     ```
//!   - `!loop bool`
//!     ```rust
//!     let _: bool = bop!(!loop match && Some(va) = a, Some(vb) = b => {
//!         1
//!     } else {
//!         2
//!     });
//!     ```
//!     *equivalent to*
//!     ```rust
//!     let _: bool = if let Some(va) = a {
//!         if let Some(vb) = b {
//!             { 1 }; true
//!         } else { { 2 }; false }
//!     } else  { { 2 }; false }
//!     ```
//! - **In**
//!   ```rust
//!   let r = 0..5;
//!   let c = bop!(&1, &2 => in && r);
//!   ```
//!   *equivalent to*
//!   ```rust
//!   let r = 0..5;
//!   let c = r.contains(&1) && r.contains(&2);
//!   ```
//!   - `||`
//!     ```rust
//!     let c = bop!(&1, &2 => in || r);
//!     ```
//!     *equivalent to*
//!     ```rust
//!     let c = r.contains(&1) || r.contains(&2);
//!     ```
//!   - custom funcion name
//!     ```rust
//!     let c = bop!(has; &1, &2 => in && r);
//!     ```
//!     *equivalent to*
//!     ```rust
//!     let c = r.has(&1) && r.has(&2);
//!     ```
//! - `Using`
//!   ```rust
//!   let v = (1, 2);
//!   let v2 = (3, 4);
//!   using!((a, b) = v, (c, d) = v2; {
//!     println!("{} {} {} {}", a, b, c, d)
//!   })
//!   ```
//!   *equivalent to*
//!   ```rust
//!   let v = (1, 2);
//!   let v2 = (3, 4);
//!   {
//!     let (a, b) = v;
//!     let (c, d) = v2;
//!     {
//!       println!("{} {} {} {}", a, b, c, d)
//!     }
//!   }
//!   ```

macro_rules! _matchand {
    { ; $b:block $($el:block)? } => { $b };
    { $p:pat = $e:expr; { } $($pp:pat = $ee:expr; { })+ ; $b:block $($el:block)?} => {
        if let $p = $e { _matchand!($($pp = $ee ; {})* ; $b $($el)?) } $(else $el)?
    };
    { $p:pat = $e:expr; { } ; $b:block $($el:block)? } => { if let $p = $e $b $(else $el)? };
}
macro_rules! _select_op {
    { $x:expr ; $op:tt $a:expr } => { $x $op $a };
    { $x:expr ; $op:tt $a:expr ; !  } => { $a $op $x };
}
/// ## Usage
/// - **Basic**  
///   - batch `||`  
///     ```rust  
///     bop!(|| 4; == 2, > 3);
///     ```
///     *equivalent to*
///     ```rust
///     4 == 2 || 4 > 3
///     ```
///   - batch `&&`  
///     ```rust  
///     bop!(&& 4; == 2, > 3);
///     ```
///     *equivalent to*
///     ```rust
///     4 == 2 && 4 > 3
///     ```
///   - `!`
///     ```rust
///     bop!(|| a; == 1;!, == 2);
///     ```
///     *equivalent to*
///     ```rust
///     1 == a || a == 2
///     ```
/// - **Set**
///   ```rust
///   let mut a = 1;
///   bop!(= a; + 1, - 2;!, * 3);
///   ```
///   *equivalent to*
///   ```rust
///   let mut a = 1;
///   a = a + 1;
///   a = 2 - a;
///   a = a * 3;
///   ```
/// - **Let**
///   ```rust
///   bop! { let a|u8 = 1, mut b = 2 }
///   ```
///   *equivalent to*
///   ```rust
///   let a: u8 = 1;
///   let mut b = 2;
///   ```
/// - **Let chain**
///   - basic
///     ```rust
///     let a = Some(1);
///     let b = Some(2);
///     
///     let _: i32 = bop!(match && Some(va) = a, Some(vb) = b => {
///         1
///     } else {
///         2
///     });
///     ```
///     *equivalent to*
///     ```rust
///     let a = Some(1);
///     let b = Some(2);
///     
///     let _: i32 = loop {
///         if let Some(va) = a {
///             if let Some(vb) = b {
///                 break { 1 };
///             }
///         }
///         break { 2 };
///     };
///     ```
///   - `bool`
///     ```rust
///     let _: bool = bop!(bool match && Some(va) = a, Some(vb) = b => {
///         1
///     } else {
///         2
///     });
///     ```
///     *equivalent to*
///     ```rust
///     let _: bool = loop {
///         if let Some(va) = a {
///             if let Some(vb) = b {
///                 { 1 };
///                 break true;
///             }
///         }
///         { 2 };
///         break false;
///     };
///     ```
///   - `!loop`
///     ```rust
///     let _: i32 = bop!(!loop match && Some(va) = a, Some(vb) = b => {
///         1
///     } else {
///         2
///     });
///     ```
///     *equivalent to*
///     ```rust
///     let _: i32 = if let Some(va) = a {
///         if let Some(vb) = b {
///             { 1 }
///         } else { { 2 } }
///     } else  { { 2 } }
///     ```
///   - `!loop bool`
///     ```rust
///     let _: bool = bop!(!loop match && Some(va) = a, Some(vb) = b => {
///         1
///     } else {
///         2
///     });
///     ```
///     *equivalent to*
///     ```rust
///     let _: bool = if let Some(va) = a {
///         if let Some(vb) = b {
///             { 1 }; true
///         } else { { 2 }; false }
///     } else  { { 2 }; false }
///     ```
/// - **In**
///   ```rust
///   let r = 0..5;
///   let c = bop!(&1, &2 => in && r);
///   ```
///   *equivalent to*
///   ```rust
///   let r = 0..5;
///   let c = r.contains(&1) && r.contains(&2);
///   ```
///   - `||`
///     ```rust
///     let c = bop!(&1, &2 => in || r);
///     ```
///     *equivalent to*
///     ```rust
///     let c = r.contains(&1) || r.contains(&2);
///     ```
///   - custom funcion name
///     ```rust
///     let c = bop!(has; &1, &2 => in && r);
///     ```
///     *equivalent to*
///     ```rust
///     let c = r.has(&1) && r.has(&2);
///     ```
#[macro_export]
macro_rules! bop {
    {} => { };

    // let op
    { let $($p:pat $(| $t:ty)? $(= $e:expr)?),*} => { $(let $p $(: $t)? $(= $e)?;)* };

    // if let op
    { match && $($p:pat = $e:expr),* => $b:block else $el:block } => {
        loop { _matchand!( $( $p = $e ; { } )* ; { break $b ; }) ; break $el; }
    };
    { bool match && $($p:pat = $e:expr),* => $b:block $(else $el:block)? } => {
        loop { _matchand!( $( $p = $e ; { } )* ; { $b ; break true; }) ; $($el ;)? break false; }
    };
    { !loop match && $($p:pat = $e:expr),* => $b:block else $el:block } => {
        _matchand!( $( $p = $e ; { } )* ; { $b } { $el })
    };
    { !loop bool match && $($p:pat = $e:expr),* => $b:block $(else $el:block)? } => {
        _matchand!( $( $p = $e ; { } )* ; { $b ; true } { $($el ;)? false })
    };

    // base op
    { $x:expr $(;)? } => { $x };
    { || $x:expr $(;)? } => { $x };
    { && $x:expr $(;)? } => { $x };
    { = $x:expr $(;)? } => { $x };
    { || $x:expr ; $($op:tt $a:expr $(;$n:tt)?),* } => { $(_select_op!($x; $op $a $(;$n)?))||* };
    { && $x:expr ; $($op:tt $a:expr $(;$n:tt)?),* } => { $(_select_op!($x; $op $a $(;$n)?))&&* };
    { = $x:ident ; $($op:tt $a:expr $(;$n:tt)?),* } => { $($x = _select_op!($x; $op $a $(;$n)?));* ; };

    // inop
    { $fname:ident ; $($v:expr),* => in && $t:expr } => { $($t.$fname($v))&&* };
    { $fname:ident ; $($v:expr),* => in || $t:expr } => { $($t.$fname($v))||* };
    { $fname:ident ; $v:expr => in && $($t:expr),* } => { $($t.$fname($v))&&* };
    { $fname:ident ; $v:expr => in || $($t:expr),* } => { $($t.$fname($v))||* };

    { $($fname:ident ;)? => in && $t:expr  } => { false };
    { $($fname:ident ;)? => in || $t:expr  } => { false };
    { $($fname:ident ;)? $v:expr => in && } => { false };
    { $($fname:ident ;)? $v:expr => in || } => { false };

    { $($v:expr),* => in && $t:expr } => { $($t.contains($v))&&* };
    { $($v:expr),* => in || $t:expr } => { $($t.contains($v))||* };
    { $v:expr => in && $($t:expr),* } => { $($t.contains($v))&&* };
    { $v:expr => in || $($t:expr),* } => { $($t.contains($v))||* };
}

/// ## Usage
/// ```rust
/// let v = (1, 2);
/// let v2 = (3, 4);
/// using!((a, b) = v, (c, d) = v2; {
///   println!("{} {} {} {}", a, b, c, d)
/// })
/// ```
/// *equivalent to*
/// ```rust
/// let v = (1, 2);
/// let v2 = (3, 4);
/// {
///   let (a, b) = v;
///   let (c, d) = v2;
///   {
///     println!("{} {} {} {}", a, b, c, d)
///   }
/// }
/// ```
#[macro_export]
macro_rules! using {
    { $($p:pat = $v:expr),* ; $b:block } => {
        { $(let $p = $v ;)* $b }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let x = bop!(|| 4 ; == 2, > 3);
        assert!(x);

        let mut a = 1;
        bop!(= a ; + 1, - 2;!, + 3);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_let() {
        bop! { let a |u8 = 1, b = 2 }
        assert_eq!(a, 1);
        assert_eq!(b, 2);
    }

    #[test]
    fn test_match() {
        let a = Some(1);
        let b = Some(2);

        let _: i32 = bop!(match && Some(va) = a, Some(vb) = b => {
            println!("some {} {}", va, vb);
            1
        } else {
            2
        });

        let _: bool = bop!(bool match && Some(va) = a, Some(vb) = b => {
            println!("some {} {}", va, vb);
            1
        } else {
            2
        });

        let _: i32 = bop!(!loop match && Some(va) = a, Some(vb) = b => {
            println!("some {} {}", va, vb);
            1
        } else {
            2
        });

        let _: bool = bop!(!loop bool match && Some(va) = a, Some(vb) = b => {
            println!("some {} {}", va, vb);
            1
        } else {
            2
        });
    }

    #[test]
    fn test_in() {
        let r = 0..5;
        let c = bop!(&1, &2 => in && r);
        assert!(c);
    }

    #[test]
    fn test_using() {
        let v = (1, 2);
        let v2 = (3, 4);
        using!((a, b) = v, (c, d) = v2 ; {
            println!("{} {} {} {}", a, b, c, d)
        })
    }
}
