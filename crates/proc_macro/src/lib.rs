//! 过程宏
//!
//! + 接收 TokenStream，返回 TokenStream
//! + 过程宏 必须 是 lib crate；
//! + 过程宏 crate，不能导出 除过程宏 外的 东西；
//! + 过程宏 不能在 定义crate 自身中使用。
//!
//! ## 分类：三种
//!
//! + 属性宏 #[inline], ...
//! + 派生宏 #[derive(Trait, ...)]
//! + 函数式宏
//!
//! ## Cargo.toml
//!
//! [lib] 写  proc-macro = true 表示 该库 定义 过程宏
//!
//! [dependencies] 写上 syn 和 quote 用于 解析 语法树
//!
//! + syn 用于 解析 和 操作 TokenStream
//! + quote 用于 将 Rust 代码 变成 TokenStream
//! 

use std::collections::HashSet;

use proc_macro::{TokenStream, Ident};
use quote::quote;
use syn::{parse_macro_input, ItemFn, parse::{Parse, ParseStream}, Token, punctuated::Punctuated};

// 声明 这就是 过程宏
// _metadata 宏调用 参数
// _input 宏 绑定的 rust 代码的 词法树
#[proc_macro_attribute]
pub fn my_custom_attribute(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    // 返回 TokenStrea，这里 直接返回 结构体
    // quote 将 rust 代码 变成 TokenStream
    TokenStream::from(quote! {struct H{}})
}

// 属性宏：跟踪变量
#[proc_macro_attribute]
pub fn trace_vars(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    // 解析 rust 函数
    let input_fn = parse_macro_input!(input as ItemFn);
    
    TokenStream::from(quote!{fn dummy(){}})
}

struct Args{
    vars:HashSet<Ident>
}

impl Parse for Args{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // parses a,b,c, or a,b,c where a,b and c are Indent
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}