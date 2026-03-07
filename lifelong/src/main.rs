//函数中的泛型生命周期
// fn main(){
//     let string1=String::from("abcd");
//     {
//     let string2=String::from("xyz");
//     let result=longest(string1.as_str(),string2.as_str());
//     println!("The longest string is {}",result);
//     }

// }
//结构体定义中的生命周期标注
// struct ImportantExcerpt<'a>{
//     part:&'a str,
// }
// fn main(){
//     let novel=String::from("Call me Ishmael. Some years ag0...");
//     let first_sentence=novel.split('.').next().expect("Could not find a '.');
//     let i=ImportantExcerpt{part:first_sentence};
// }