//else后的变量只能是一个类型，不能前后的类型是不一样的
fn main(){
    let  condition=true;
    let number=if condition{5}else{6};
    println!("The value of number is:{}",number);
}