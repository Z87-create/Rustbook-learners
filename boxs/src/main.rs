enum list{
    Cons(i32,Box<List>),
    Nil,
}
use crate::List::{Cons,Nil};
fn main(){
    //使用Box<T>指向堆上的数据
    let b=Box::new(5);
    println!("b = {}",b);
    //使用cons list来储存列表1,2,3
    let list=Cons(1,Cons(2,Cons(3,Nil)));
    //Cons实现
    // fn main(){
    // let list=Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
    //}
    
}