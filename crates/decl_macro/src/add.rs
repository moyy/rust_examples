// 定义 声明宏 add
// 实现时，类似 match 的 思路，匹配模式
// 使用时，用  add!
// 加 这个 意味着 将宏导出
#[macro_export]
macro_rules! add {
    // 匹配到一个参数，直接返回
    ($a:expr) => {
        $a
    };

    // 匹配到 两个表达式参数
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

// 测试 类型 转换
#[macro_export]
macro_rules! add_as {
    // 匹配到 两个表达式参数，相加后 变为对应类型 返回
    ($a:expr, $b:expr, $typ:ty) => {
        $a as $typ + $b as $typ
    };
}

// 变长参数，TT muncher 惯用法
// 1个参数，2个参数，多个参数 则 递归
#[macro_export]
macro_rules! add_var {
    // 基本情况，1个参数
    ($a:expr) => {
           $a
    };
    
    // 基本情况，2个参数
    ($a:expr, $b:expr) => {
        $a + $b
    };

    // 多个参数，递归
    // 递归写法 $($名)
    ($a:expr, $($b:tt)*) => {
        $a + add_var!( $($b)* )
    }
}
