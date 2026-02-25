fn main(){
    //宏调用是一个表达式，我们用来创建新作用域的大括号(代码块){}也是一个表达式
    let y={
        let x=3;
        x+1
    };
    println!("The value of y is:{}",y)
}