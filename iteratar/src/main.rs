//迭代器负责遍历序列中的每一项和决定序列何时结束的逻辑，迭代器是惰性的
//创建一个迭代器
fn main(){
let v1=vec![1,2,3];
let v1_iter=v1.iter();
//我们可以循环遍历使用迭代器
for val in v1_iter{
    println!("Got:{}",val)
}
}
