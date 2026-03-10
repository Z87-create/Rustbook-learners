//类型别名用来创建类型同义词
// type Killometers=i32;
// fn main(){
//     type Kilometer=i32;
//     let x:i32=5;
//     let y:Kilometer=5;
//     println!("x + y = {}",x+y);
// }
// let f:Box<dyn Fn() + send + 'static>=Box::new(|| println!("hi"));
// fn takes_long_type(f:Box<dyn Fn() + Send + 'static>){

// }
// fn returns_long_type()->Box<dyn Fn() + Send +'static>{

// }
//我们可以替换为trunk
// type Thunk=Box<dyn Fn() + Send + 'static>;
// let f:Thunk=Box::new(|| println!("hi"));
// fn talks_long_type(f:Thunk){

// }
// fn returns_long_type()->Thunk{

// }