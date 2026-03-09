/// Adds one to the number given.
/// #Examples //表示创建一个以Example为标题的部分
/// ```
/// let arg=5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6,answer);
/// ```
//可以运行cargo doc来生成这个文档注释的HTML文档。这个命令行由Rust分发的工具rustdoc并将生成的HTML文档放入target/doc目录下。可以使用cargo doc --open会构建文档并在浏览器中打开它。
pub fn add_one(x:i32)->i32{
    x+1
}
//注释包含项的结构，//!为包含注释的项添加文档
//! # My Crate
//!
//! `my_crate is a collection of utilities to make performing certain
//! calculations more environment.
/// Adds one to the number given.
// --snip--
// 使用pub use导出合适的共有API
//！ # Art
//!
//! A library for modeling artistic concepts
pub mod kinds{
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor{
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor{
        Orange,
        Green,
        Purple,
    }
}

pub mod utils{
    use crate::kinds::*;

    /// Combines two primary colors in equal amouunts to create
    /// a secondary color.
    pub fn mix(c1:PrimaryColor,c2:PrimaryColor)->SecondaryColor{
        
    }
}