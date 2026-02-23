//和其他函数一样，构造函数在rust是可以传递参数的
fn main(){
    another_function(5);
    print_labeled_measurement(5,'h');
}
fn another_function(x:i32){
    println!("The value of x is:{}",x);
}
fn print_labeled_measurement(value:i32,uint_label:char){
    println!("The measurement is:{}{}",value,uint_label);
}