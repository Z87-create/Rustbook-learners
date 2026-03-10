//使用自定义宏
use hello_macro::HelloMacro;
struct Pancakes;
#[derive(HelloMacro)]
impl HelloMacro for Pancakes{
    fn hello_macro(){
        println!("Hello,Macro! My name is Pancakes!");
    }
}
fn main(){
    Pancakes::hello_macro();
}