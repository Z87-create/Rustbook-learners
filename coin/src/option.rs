fn plus_one(x:Option<i32>)->Option<i32>{
    match x{
        None=>none,
        Some(i)=>Some(i+1),
    }
}
//在match匹配过程中与调用的构造函数分支相互匹配
let five=Some(5);
let six=plus_one(five);
let none=plus_one(none);