fn main() {
    //通过解引用运算符追踪指针的值
    let x=5;
    let y=&x;
    assert_eq!(5,x);
    assert_eq!(5,*y);
    //像引用一样使用Box<T>
    let x=5;
    let y=Box::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y);
    //自定义智能指针
    struct MyBox<T>(T);
    impl<T> MyBox<T>{
        fn new(x:T)->MyBox<T>{
            MyBox(x)
        }
    }
}
//通过实现Deref trait将某类型像引用一样处理
use std::ops::Deref;
impl<T> Deref for MyBox<T>{
    type Target=T;
    fn deref(&self)->&T{
        &self.0
    }
}
