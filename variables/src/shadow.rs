//我们可以通过重复使用let来去覆盖变量
fn main(){
    let x=5;
    let x= x+1;
    {
        let x=x*2;
        println!("The value of x in the inner scope is:{}",x);
    }
    println!("The value of x is:{}",x);
}