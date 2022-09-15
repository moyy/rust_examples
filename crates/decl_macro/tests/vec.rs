use decl_macro::my_vec;

// 展开后，变成
#[test]
fn test_vec() {
    // 展开后，变成
    // {
    //     let mut r = Vec::new();
    //     r.push(1);
    //     r.push(2);
    //     r.push(3);
    //     r
    // }
    let v = my_vec![1, 2, 3];
    assert_eq!(v, vec![1, 2, 3]);

    let v = my_vec![1, 2, 3,];
    assert_eq!(v, vec![1, 2, 3]);

    let v = my_vec!(1, 2, 3);
    assert_eq!(v, vec![1, 2, 3]);

    let v = my_vec! {1, 2, 3};
    assert_eq!(v, vec![1, 2, 3,]);

    let v = my_vec![1];
    assert_eq!(v, vec![1]);

    let v: Vec<f32> = my_vec![];
    assert_eq!(v, vec![]);
}
