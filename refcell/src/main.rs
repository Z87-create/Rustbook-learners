//内部可变性是Rust中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据,
#[derive(Debug)]
enum List{
    Cons(Rc<RefCell<i32>>,Rc<List>),
    Nil,
}
use crate::List::{Cons,Nil};
use std::rc::Rc;
use std::cell::RefCell;
fn main(){
    let value=Rc::new(RefCell::new(5));
    let a=Rc::new(Cons(Rc::clone(&value),Rc::new(Nil)));
    let b:List=Cons(Rc::new(RefCell::new(6)),Rc::clone(&a));
    let c:List=Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    *value.borrow_mut()+=10;
    println!("a after = {:?}",a);
    println!("b after = {:?}",b);
    println!("c after = {:?}",c);
}