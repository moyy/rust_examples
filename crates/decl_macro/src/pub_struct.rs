/// 匹配 struct 的定义
/// 作用：将 struct 成为 pub；已经 struct 的 所有成员改为 pub
#[macro_export]
macro_rules! make_public {
    (
        // meta 0个 或 多个 结构体 属性
        // 例子：#[derive(Debug)]
        // 例子：#[inline]
        $(#[$meta:meta])*
        // vis 是 （可能为空的）的 pub
        $vis:vis struct $struct_name:ident {
            $(
                // 0个 或 多个 成员属性 
                $(#[$field_meta:meta])*
                // field_vis 可能没有的 pub
                $field_vis:vis $field_name:ident: $field_type:ty
            ), *$(,)+
        }
    ) => {
        $(#[$meta])*
        pub struct $struct_name {
            $(
                $(#[$field_meta:meta])*
                pub $field_name: $field_type,
            )*
        }
    }
}