//提取函数减少重复
// fn largest(list:&[i32])->i32{
//     let mut largest=list[0];
//     for &item in list{
//         if item>largest{
//             largest=item;
//         }
//     }
//     largest
// }
// fn main(){
//     let number_list=vec![34,50,25,100,65];
//     let result=largest(&number_list);
//     println!("The largest number is {}",result);
//     let number_list=vec![102,34,6000,89,54,2,43,8];
//     let result=largest(&number_list);
//     println!("The largest number is {}",result);
// }
//在函数定义中使用泛型
// fn largest_i32(list:&[i32])->i32{
//     let mut largest=list[0];
//     for &item in list.iter(){
//         if item>largest{
//             largest=item;
//         }
//     }
//     largest
// }
// fn largest_char(list:&[char])->char{
//     let mut largest=list[0];
//     for &item in list.iter(){
//         if item>largest{
//             largest=item;
//         }
//     }
//     largest
// }
// fn main(){
//     let number_list=vec![34,50,25,100,65];
//     let result=largest_i32(&number_list);
//     println!("The largest number is {}",result);
//     let char_list=vec!['y','m','a','q'];
//     let result=largest_char(&char_list);
//     println!("The largest char is {}",result);
// }
//结构体定义中的泛型
// struct Point<T>{
//     x:T,
//     y:T,
// }
// fn main(){
//     let integer=Point{x:5,y:10};
//     let float=Point(x:1.0,y:4.0);
// }
//结构体中可以是不同类型的
// struct Point<T,U>{
//     x:T,
//     y:U,
// }
// fn main(){
//     let both_integer=Point{x:5,y:10};
//     let both_float=Point{x:1.0,y:4.0};
//     let integer_and_float=Point{x:5,y:4.0};
// }
//枚举定义中的泛型
// enum Option<T>{
//     Some(T),
//     None,
// }
//枚举中也可以有多个泛型
// enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }
//方法定义中的泛型
// struct Point<T>{
//     x:T,
//     y:T,
// }
// impl<T> Point<T>{
//     fn x(&self)->&T{
//         &self.x
//     }
// }
// fn main(){
//     let p=Point{x:5,y:10};
//     println!("p.x={}",p.x());
// }
//方法体使用了与结构体中不同的泛型
// struct Point<T,U>{
//     x:T,
//     y:U,
// }
// impl<T,U> Point<T,U>{
//     fn mixup<V,W>(self,other:Point<V,W>)->Point<T,W>{
//         Point{
//             x:self.x,
//             y:other.y,
//         }
//     }
// }
// fn main(){
//     let p1=Point{x:5,y:10.4};
//     let p2=Point{x:"Hello",y:'c'};
//     let p3=p1.mixup(p2);
//     println!("p3.x={},p3.y={}",p3.x,p3.y);
// }
//泛型代码的性能
enum Option_i32{
    Some(i32),
    None,
}
enum Option_f64{
    Some(f64),
    None,
}
fn main(){
    let integer=Option_i32::Some(5);
    let float=Option_f64::Some(5.0);
}