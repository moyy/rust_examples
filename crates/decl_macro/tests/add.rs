use decl_macro::{add, add_as, add_var};

#[test]
fn test_add_1() {
    let s = add!(2);
    assert_eq!(s, 2);
}

#[test]
fn test_add_2() {
    let s = add!(2, 3);
    assert_eq!(s, 5);
}

#[test]
fn test_add_as() {
    let s = add_as!(8, 2, u8);
    assert_eq!(s, 10u8);
}

// 测试 变长参数；
// tt惯用法
#[test]
fn add_var() {
    let s = add_var!(1);
    assert_eq!(s, 1);

    let s = add_var!(1u64, 2u64);
    assert_eq!(s, 3u64);

    let s = add_var!(1.0f32, 2.0f32);
    assert_eq!(s, 3.0f32);

    let s = add_var!(1, 2, 3, 4, 5);
    assert_eq!(s, 15);
}
