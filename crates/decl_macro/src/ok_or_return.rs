
// 作用：类似于 ?
#[macro_export]
macro_rules! ok_or_return {
    // 补一个 专门针对变量的匹配
    // 例子 let a = Ok(3); ok_or_return!(a);
    ($a:ident) => {
        match $a {
            Ok(value) => value,
            Err(err) => {
                return Err(err);
            }
        }
    };

    // 匹配 标识符(语法树)
    // 圆括号内 的 可以 是 任何东西
    // ok_or_return!(f()) 能通过，a = f, $($b)* = 空
    // ok_or_return!(f(1, 2, 3)) 能通过，a = f, $($b)* = 1, 2, 3
    // ok_or_return!(a) 不能通过，因为 没有 圆括号
    ($a:ident($($b:tt)*)) => {
        match $a($($b)*) {
            Ok(value) => value,
            Err(err) => {
                return Err(err);
            }
        }
    };
}