#![feature(plugin)]
#![plugin(interpolate_idents)]

macro_rules! define_foo {
    ($x:ident) => ( interpolate_idents! {
        fn [foo_ $x _1]() -> u32 { 1 }

        struct [Foo $x] { [$x _30]: u32 }
        impl [Foo $x] {
            pub fn new() -> [Foo $x] {
                [Foo $x] { [$x _30]: 30 }
            }
        }
    } )
}

define_foo!(bar);

#[test]
fn test_macro() {
    assert_eq!(foo_bar_1(), 1);
    assert_eq!(Foobar::new().bar_30, 30);
}

macro_rules! define_brackets {
    () => ( interpolate_idents! {
        fn brackets(data: &[i32; 1]) -> Vec<i32> {
            let mut b: Vec<i32> = vec![];
            let c: Vec<i32> = vec![1, 2, 3];
            let d: Vec<i32> = vec![1; 25];
            b.push(c[1]);
            b.push(d[1]);
            b.push(data[0]);
            b
        }
    } )
}

define_brackets!();

#[test]
fn test_brackets() {
    let data = [1; 1];
    assert_eq!(brackets(&data), vec![2, 1, 1]);
}
