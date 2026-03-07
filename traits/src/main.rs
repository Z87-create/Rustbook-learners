// fn largest<T:Partial0rd+Copy>(list:&[T])->T{
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
//     let result=largest(&number_list);
//     println!("The largest number is {}",result);

//     let char_list=vec!['y','m','a','q'];
//     let result=largest(&char_list);
//     println!("The largest char is {}",result);
// }
//使用trait bound有条件实现方法
use std::fmt::Display;
struct Pair<T>{
    x:T,
    y:T,
}
impl<T> Pair<T>{
    fn new(x:T,y:T)->Self{
        Self{
            x,
            y,
        }
    }
}
impl<T:Display + PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x>=self.y{
            println!("The largest member is x={}",self.x);
        }else{
            println!("The largest member is y={}",self.y);
        }
    }
}