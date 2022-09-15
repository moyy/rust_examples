# 声明宏

目的：通过 容易使用的方式 来 消除 重复代码

例子：vec!, println!

## 0、背景

Rust 源码串 --词法分析--> Token[] --语法分析--> AST 和 Token-Tree（标志树） --宏展开--> ...
 
所有的 !宏 参数 是 非叶节点 的 标志树；(...) [...] {...}

## 1、定义 与 导出

+ 定义：通过 `macro_rules!`
+ 导出：加上 `#[macro_export]` 会导出到 crate 外部

## 匹配项 类型

|类型|解释|示例|
|--|--|--|
|ty|Rust 类型|u32、u33、String|
|ident|标识符；非 关键字的 字符串|x / long_identifier / SomeSortOfAStructType|
|expr|表达式|1、x + 1、if x == 4 { 1 } else { 2 }|
|stmt|语句|let x = 1|
|item|模块级的元素，包括：函数、use 声明及类型定义|use std::io / fn main() { println!("hello") } / const X:usize = 8;|
|meta|元项|#[foo]的foo / #[foo(bar)]的foo(bar)|
|pat|match 表达式中左侧 都是 模式|1 / "x" / t / Some(t) / 1...3|
|path|路径，用于 use 后面|foo / foo::bar|
|vis|可见性修饰符|pub / pub(crate)|
|lifetime|生命周期|'a / 'ctx / 'foo|
|literal|字面量 或 标识符|"foo" / bar / 34.3.f32|
|block|块|{...}|
|tt|词法树|bar / if x == 2 {3} else {4}|

## 重复模式

+ $( ... )*    匹配 0次 或 多次，没有分隔符
+ $( ... ),*   匹配 0次 或 多次，以逗号分隔
+ $( ... );*   匹配 0次 或 多次，以分号分隔

## 内置宏

|宏|解释|示例|
|--|--|--|
|file!()|当前文件名||
|line!()|当前行号||
|column!()|当前列号||
|stringify!(...)|将 ... 展开成字面量||
|cfg!(...)|展开成 布尔常量||
|env!("VAR_NAME")|编译时，取 环境变量的值，得到 字符串|let version = env!("CARGO_PKG_VERSION");|
|include!("file.rs")|展开 指定rust文件的内容||
|include_str!("file.txt")将 文件内容 变成 &‘static str 返回||
