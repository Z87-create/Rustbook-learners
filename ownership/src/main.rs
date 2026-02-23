fn main(){
    let s=String::from("hello");//s进入作用域
    takes_ownership(s);//s移动到函数里..
    let x=5;//x进入作用域
    make_copy(x);//x应该移动函数里，但i32是copy的，所以后面可继续使用
}//这里，x先移出了作用域，然后是s。但因为s的值已被移走，所以不会有特殊的操作
fn takes_ownership(some_string:String){//some_string进入作用域
    println!("{}",some_string);
}//这里，some_string移出作用域并调用`drop`方法，占用的内存被释放
fn make_copy(some_integer:i32){//some_integer进入作用域
    println!("{}",some_integer);
}//这里，some_integer移出作用域。不会有特殊操作