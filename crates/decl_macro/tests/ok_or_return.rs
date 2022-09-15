use decl_macro::ok_or_return;

#[test]
fn test_ok_or_return() {
    assert_eq!(Err("e1".to_owned()), test_impl());
}

fn test_impl() -> Result<(), String> {

    let a = Ok(3); 
    assert_eq!(3, ok_or_return!(a));

    let r = ok_or_return!(Ok((1, 4)));
    assert_eq!((1, 4), r);

    assert_eq!((1, 4), ok_or_return!(some_work(1, 4)));

    ok_or_return!(Err("e1".to_owned()));
    
    Ok(())
}

fn some_work(i: i64, j: i64) -> Result<(i64, i64), String> {
    if i + j > 2 {
        Ok((i, j))
    } else {
        Err("error".to_owned())
    }
}
