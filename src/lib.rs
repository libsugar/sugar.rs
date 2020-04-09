macro_rules! _matchand {
    { ; $b:block $($el:block)? } => { $b };
    { $p:pat = $e:expr; { } $($pp:pat = $ee:expr; { })+ ; $b:block $($el:block)?} => {
        if let $p = $e { _matchand!($($pp = $ee ; {})* ; $b $($el)?) } $(else $el)?
    };
    { $p:pat = $e:expr; { } ; $b:block $($el:block)? } => { if let $p = $e $b $(else $el)? };
}
#[macro_export]
macro_rules! bop {
    {} => { };
    { let $($p:pat , $(: $t:ty)? $(= $e:expr)?;)*} => { $(let $p $(: $t)? $(= $e)?;)* };
    { match && $($p:pat = $e:expr),* => $b:block $(else $el:block)? } => {
        _matchand!( $( $p = $e ; { } )* ; { $b ; true } { $($el ;)? false });
    };
    { -> $t:ty ; match && $($p:pat = $e:expr),* => $b:block else $el:block } => {
        { let retv: $t;  { retv = _matchand!( $( $p = $e ; { } )* ; { $b; } { $el; }) }; retv }
    };
    { || -> $t:ty ; match && $($p:pat = $e:expr),* => $b:block else $el:block } => {
        || -> $t { _matchand!( $( $p = $e ; { } )* ; { return $b; }) ; return $el; } ()
    };
    { move || -> $t:ty ; match && $($p:pat = $e:expr),* => $b:block else $el:block } => {
        move || -> $t { _matchand!( $( $p = $e ; { } )* ; { return $b; }) ; return $el; } ()
    };
    { $x:expr $(=>$(:)?)? } => { $x };
    { $x:expr $(=>$($t:tt$(:)?)?)? } => { $x };
    { $x:expr => || : $($op:tt $a:expr),* } => { $($x $op $a)||* };
    { $x:expr => && : $($op:tt $a:expr),* } => { $($x $op $a)&&* };
    { $x:ident => = : $($op:tt $a:expr),* } => { $($x = $x $op $a);* };
}

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
        let x = bop!(4 => || : == 2, > 3);
        assert!(x);

        let mut a = 1;
        bop!(a => = : + 1, + 2, + 3);
        assert_eq!(a, 7);
    }

    #[test]
    fn test_let() {
        bop! {let a, = 1; b, = 2;}
        assert_eq!(a, 1);
        assert_eq!(b, 2);
    }

    #[test]
    fn test_match() {
        let a = Some(1);
        let b = Some(2);
        bop!(|| -> () ; match && Some(va) = a, Some(vb) = b => {
            println!("some {:?} {:?}", va, vb)
        } else {

        });
    }

    #[test]
    fn test_using() {
        let v = (1, 2);
        using!((a, b) = v ; {
            println!("some {:?} {:?}", a, b)
        })
    }
}
