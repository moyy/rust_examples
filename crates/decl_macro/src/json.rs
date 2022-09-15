use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

/// 作用 json 简写
#[macro_export]
macro_rules! json {
    // null 直接转
    (null) => {
        Json::Null
    };

    // 数组
    ([$($element:tt) ,*]) => {
        // rust 宏递归 最大深度是 64次
        // 需要时，可以指定 #![recursion_limit = "256"]
        Json::Array(vec![ $( json!($element) ),* ])
    };

    // 对象
    ({ $($key:tt : $value:tt),* }) => {
        // 注：vec![(K, V)] --> HashMap<K, V>
        Json::Object(Box::new(vec![
            $( ($key.to_string(), json!($value)) ),*
        ].into_iter().collect()))
    };

    // 单一元素：Number, String, or Boolean
    ($other:tt) => {
        // 用 from 技巧
        Json::from($other)
    };
}

// bool -> json
impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

// string -> json
impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

// str -> json
impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}

// 数字 --> json
macro_rules! impl_from_num_for_json {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}
impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64);