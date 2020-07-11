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
fn test_batch() {
    let x = bop!(|| 1 ; < ; 5, 6, 7, 0;!);
    assert!(x)
}

#[test]
fn test_let() {
    bop! { let a |u8 = 1, b = 2 }
    assert_eq!(a, 1);
    assert_eq!(b, 2);
}

#[allow(unused_variables)]
#[test]
fn test_match() {
    let a = Some(1);
    let b = Some(2);

    let _: i32 = bop!('a: match && Some(va) = a, Some(vb) = b => {
        #[cfg(feature = "std")] {
            println!("some {} {}", va, vb);
        }
        1
    } else {
        2
    });

    let _: bool = bop!(bool 'b: match && Some(va) = a, Some(vb) = b => {
        #[cfg(feature = "std")] {
            println!("some {} {}", va, vb);
        }
        1
    } else {
        2
    });

    let _: i32 = bop!(!loop match && Some(va) = a, Some(vb) = b => {
        #[cfg(feature = "std")] {
            println!("some {} {}", va, vb);
        }
        1
    } else {
        2
    });

    let _: bool = bop!(!loop bool match && Some(va) = a, Some(vb) = b => {
        #[cfg(feature = "std")] {
            println!("some {} {}", va, vb);
        }
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

#[allow(unused_variables)]
#[test]
fn test_using() {
    let v = (1, 2);
    let v2 = (3, 4);
    using!((a, b) = v, (c, d) = v2 ; {
        #[cfg(feature = "std")] {
            println!("{} {} {} {}", a, b, c, d)
        }
    })
}

#[test]
fn test_fn() {
    let v = 1;
    let v = effect(v, |v| assert_eq!(*v, 1));
    assert_eq!(v, 1);
    let mut v = using(v, |v| v + 1);
    assert_eq!(v, 2);
    using(&mut v, |v| *v = 3);
    assert_eq!(v, 3);
}
