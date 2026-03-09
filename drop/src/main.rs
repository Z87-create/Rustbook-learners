//通过std::mem::drop提早丢弃值
use std::mem::drop;
struct CustomSmartPointer{
    data:String,
}
impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data `{}`!",self.data);
    }
}
fn main(){
    // let c=CustomSmartPointer{data:String::from("my stuff")};
    // let d=CustomSmartPointer{data:String::from("other stuff")};
    // println!("CustomSmartPointers created.");
    let c=CustomSmartPointer{data:String::from("some data")};
    println!("CustomSmarterPointer created.");
    drop(c);
    println!("CustomSmarterPointer dropped before the end of main.");
}
