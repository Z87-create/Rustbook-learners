//第一个类型是Vec<T>，也被称为vector，vector允许我们在一个单独的数据结构中储存多个值，所有值在内存中彼此相互排列，vector只能储存相同类型的值
//新建vector,可以调用Vec::new函数
// let v:Vec<i32>=Vec::new();
//在更实际的代码中，一旦插入值rust就能推断出想要存放的类型，而且为了方便Rust提供了vec!宏，这个宏会根据我们提供的值来创建一个新的Vec
// let v=vec![1,2,3];
//更新vector,向里面添加元素需要调用push方法
// let mut v=Vec::new();
// v.push(5);
// v.push(6);
// v.push(7);
// v.push(8);
//丢弃vector时也会丢弃其所有的元素
// {
//     let v=vec![1,2,3,4];
    //处理变量v
// }//<-这里v离开作用域并被丢弃
//读取vector元素
fn main(){
let v=vec![1,2,3,4,5];
let third:&i32=&v[2];
println!("The third element is {}",third);
match v.get(2){
    Some(third)=>println!("The third element is {}",third),
    None=>println!("There is no third elemnt.")
}
}
//遍历vector元素而遍历
let v=vec![100,32,57];
for i in &v{
    println!("{}",i);
}
//可变引用
let mut v=vec![100,32,57];
for i in &mut v{
    *i+=50;
}
//通过枚举来储存多种类型
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}
let row=vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::float(10.12),
];