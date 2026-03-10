pub trait Draw{
    fn draw(&self);
}
pub struct Screen{
    pub components:Vec<Box<dyn Draw>>,
}
impl Screen{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}
//代替实现用泛型和trait bound
pub struct Screen<T:Draw>{
    pub components:Vec<T>,
}
impl<T> Screen<T>
where T:Draw{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}
//实现trait
pub struct Button{
    pub width:u32,
    pub height:u32,
    pub label:String,
}
impl Draw for Button{
    fn draw(&self){
        //实际绘制按钮代码
    }
}