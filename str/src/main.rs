//新建字符串,创建一个空字符串
let mut s = String::new();
//向字符串里装载数据
let data="initial contents";
let s=data.to_string();
//该方法也可直接用于字符串字面量
let s="initial contents".to_string();
//String::from函数来从函数来从字符串字面量创建String,下面的案例等同于使用to_String
let s=String::from("initial contents");
//使用push_str和push附加字符串
let mut s=String::from("foo");
s.push_str("bar");
println!("{}",s);
//push_str不会获取参数的所有权
//push方法被定义为获取一个单独的字符作为参数，并附加到String中
let mut s=String::from("lo");
s.push('l');
println!("{}",s);
//使用+运算符或format!宏来连接字符串
let s1=String::from("Hello,");
let s2=String::from("world!");
let s3=s1+&s2;//s1被移动了，不能再使用
println!("{}",s3);
//使用format!宏来连接字符串
let s1=String::from("tic");
let s2=String::from("toc");
let s3=String::from("toe");
let s=format!("{}-{}-{}",s1,s2,s3);
println!("{}",s);
//内部表现
let len=String::from("Hala").len();
println!("len:{}",len);