// 测试 属性宏

use proc_macro::my_custom_attribute;

// 通过 宏，将 S 变成 struct H
#[my_custom_attribute]
struct S {}

#[test]
fn test_custom_attribute() {
    // 这里直接 用 H 即可，已经没有了 S
    let _h = H {};

    // 这里 会编译报错，因为 经过 属性宏，没有了 S
    // let s = S {};
}

#[test]
fn test_trace_vars() {
    #[trace_vars(a)]
    fn do_something() {
        let a = 9;
        a = 6;
        a = 0;
    }
}
