fn main(){
    let n=10;
    println!("前{}个费波纳奇数:",n);
    for i in 0..n{
        let result=fib(i);
        print!("{}",result);
    }
    println!();
}
fn fib(n:i32)->f64{
    if n==0{
        return 0.0;
    }else if n==1{
        return 1.0;
    }
    fib(n-1)+fib(n-2)
}