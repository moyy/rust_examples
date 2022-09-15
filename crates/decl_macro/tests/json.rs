use decl_macro::{json, Json};

#[test]
fn test_null() {
    let null = json!(null);
    assert_eq!(null, Json::Null);
}

#[test]
fn test_bool() {
    let j = json!(true);
    assert_eq!(j, Json::Boolean(true));

    let j = json!(false);
    assert_eq!(j, Json::Boolean(false));
}

#[test]
fn test_number() {
    let j = json!(34);
    assert_eq!(j, Json::Number(34.0));

    let j = json!(60u32);
    assert_eq!(j, Json::Number(60.0));

    let j = json!(25.25f32);
    assert_eq!(j, Json::Number(25.25));
}

#[test]
fn test_string() {
    let j = json!("abc");
    assert_eq!(j, Json::String("abc".to_string()));

    let s = "abc".to_string();
    let j = json!(s);
    assert_eq!(j, Json::String("abc".to_string()));
}

#[test]
fn test_array() {
    let j = json!([1, 2, 3]);
    assert_eq!(
        j,
        Json::Array(vec![
            Json::Number(1.0),
            Json::Number(2.0),
            Json::Number(3.0)
        ])
    );

    let j = json!([1, true, null]);
    assert_eq!(
        j,
        Json::Array(vec![Json::Number(1.0), Json::Boolean(true), Json::Null])
    );
}

#[test]
fn test_object() {
    let j = json!({"a": 123, "b": true});
    assert_eq!(
        j,
        Json::Object(Box::new(
            vec![
                ("a".to_string(), Json::Number(123.0)),
                ("b".to_string(), Json::Boolean(true))
            ]
            .into_iter()
            .collect()
        ))
    );
}

#[test]
fn test_json() {
    let students = json!([
        {
            "name": "Jim Blandy",
            "class_of": 1926,
            "major": "Tibetan throat singing"
        },
        {
            "name": "Jason Orendorff",
            "class_of": 1702,
            "major": "Knots"
        }
    ]);

    println!("student = {:?}", students);
}
