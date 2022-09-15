use decl_macro::make_public;

// 展开后，变成
// pub struct A {
//     pub a: i32,
//     pub b: f32,
//     pub c: u32,
// }
make_public!(
    struct A {
        a: i32,
        b: f32,
        pub c: u32,
    }
);