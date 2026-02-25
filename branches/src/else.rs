//6即使可以被2整除也不会输出，因为rust只会执行第一个条件为真的代码，并且它找到第一个后，甚至不会检查之后的条件了
fn main(){
    let number=6;
    if number%4==0{
        println!("number is divisible by 4");
    }else if number%3==0{
        println!("number is divisible by 3");
    }else if number%2==0{
        println!("number is divisible by 2");
    }else{
        println!("number is not divisible by 4,3,or 2");
    }
}