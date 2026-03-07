// use std::fs::File;
//使用match表达式处理可能会返回的Result成员
// fn main(){
//     let f = File::open("hello.txt");
//     let f = match f {
//         Ok(file) => file,  // 直接返回 file，不需要额外的 {}
//         Err(error) => {
//             panic!("Problem opening the file: {:?}", error)
//         }
//     };
// }
// use std::fs::File;
// use std::io::ErrorKind;
// fn main(){
//     let f=File::open("hello.txt");
//     let f=match f{
//         Ok(file)=>file,
//         Err(error)=>match error.kind(){
//             ErrorKind::NotFound=>match File::create("hello.txt"){
//                 Ok(fc)=>fc,
//                 Err(e)=>panic!("Problem creating the file:{:?}",e),
//             },
//             other_error=>panic!("Problem opening the file:{:?}",other_error),
//         },
//     };
// }
//失败时的panic缩写：unwrap和expect
// use std::fs::File;
// fn main(){
//     let f=File::open("hello.txt").unwrap();
// }
//传播错误：当编写一个需要先调用一些可能失败的操作的函数时，除了在这个函数处理错误外，还可以选择让调用者知道这个错误并决定如何处理
//使用match将错误返回给代码调用者
// use std::io;
// use std::io::Read;
// use std::io::File;
// fn read_username_from_file()->Result<String,io::Error>{
//     let f=File::open("hello.txt");
//     let mut f=match f{
//         Ok(file)=>file,
//         Err(e)=>return Err(e),
//     };
//     let mut s=String::new();
//     match f.read_to_string(&mut s){
//         Ok(_)=>Ok(s);
//         Err(e)=>Err(e),
//     }
// }
//传播错误的简写：？简写
// use std::io
// use std::io::Read;
// use std::fs::File;
// fn read_username_from_file()->Result<String,io::Error>{
//     let mut f=File::open("hello.txt")?;
//     let mut s=String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }
//进一步缩短,问号运算符之后的链式方法调用
// use std::io;
// use std::fs::File;
// use std::io::Read;
// fn read_username_from_file()->Result<String,io::Error>{
//     let mut s=String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }
//使用std::fs::read_to_string函数进一步缩短
// use std::io;
// use std::fs;
// fn read_username_from_file()->Result<String,io::Error>{
//     fs::read_to_string("hello.txt")
// }
//使用Result<T,E>类型的main函数
use std::error::Error;
use std::fs::File;
fn main()->Result<(),Box<dyn Error>>{
    let f=File::open("hello.txt")?;
    Ok(())
}
