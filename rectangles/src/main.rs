//计算原始长方形数据的案例，area传递两个相关度不高的元素，长方形的长和宽本身是相互联系的
//fn main(){
//    let width1=30;
//    let height1=50;
//    println!("The area of the rectangle is {} square pixels.",area(width1,height1));

//}
//fn area(width:u32,height:u32)->u32{
//    width*height
//}
//使用元组进行代码重构,传递两个参数变为一个参数，元组并没有给出元素的名称，我们不得不用索引理解这个部分
// fn main(){
//     let rect1=(30,50);
//     println!("The area of the rectangle is {} square pixels.",area(rect1));
// }
// fn area(dimensions:(u32,u32))->u32{
//     dimensions.0*dimensions.1
// }
//使用结构体重构，赋予更多意义
// struct Rectangle{
//     width:u32,
//     height:u32,
// }
// fn main(){
//     let rect1=Rectangle{
//         width:30,
//         height:50,
//     };
//     println!("The area of rectangle is {} square pixels.",area(&rect1));
// }
// fn area(rectangle:&Rectangle)->u32{
//     rectangle.width * rectangle.height
// }
// #[derive(Debug)] //增加外部属性打印调试信息的功能
// struct Rectangle{
//     width:u32,
//     height:u32,
// }
// fn main(){
//     let rect1=Rectangle{
//         width:30,
//         height:50,
//     };
//     println!("rect1 is {:?}",rect1);
// }
//调用dbg！宏打印到标准错误控制流
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}
fn main(){
    let scale=2;
    let rect1=Rectangle{
        width:dbg!(30*scale),
        height:50,
    };
    dbg!(&rect1);
}