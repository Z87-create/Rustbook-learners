//io是rust的输入输出库，io库来自于标准库，标准库也被称为std
use rand::Rng;
use std::cmp::Ordering;
use std::io;
//main函数是程序的入口点
fn main(){
    println!("Guess the number!");
    let secret_number=rand::thread_rng().gen_range(1..101);
    loop{
    println!("Please input your guesss.");
    //在rust中直接let赋值变量的最终结果是不可变的，需要let后加mut确保数据是可变的
    let mut guess=String::new();
    //::new这个语法表明new是String类型的一个关联函数，关联函数是实现一种特定类型的函数，在这个例子是String
    //从io库中调用stdin函数,这将允许我们处理用户输入，代表终端输入句柄
    io::stdin()
    //调用read_line方法，从标准句柄获取用户输入，从标准输入获取的任何内容都追加到一个字符串中
    .read_line(&mut guess)
    //.expect起到的是一个result
    .expect("Failed to read line");
    let guess:u32=match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };
    println!("You guessed:{}",guess);

    match guess.cmp(&secret_number){
        Ordering::Less=>println!("Too small!"),
        Ordering::Greater=>println!("Too big!"),
        Ordering::Equal=>{
            println!("You win!");
            break;
        }
    }
}
}