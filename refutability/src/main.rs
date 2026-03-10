//模式语法
//匹配字面量
fn main() {
    let x=1;
    match x{
        1=>println!("one"),
        2=>println!("two"),
        3=>println!("three"),
        _=>println!("anything"),
    }
}
匹配命名变量
fn main(){
    let x=Some(5);
    let y=10;
    match x{
        Some(50)=>println!("Got 50"),
        Some(y)=>println!("Matched,y = {:?}",y),
        _=>println!("Default case,x = {:?}",x),
    }
    println!("at the end: x = {:?},y = {:?}",x,y);
}
//多个模式，在match表达式中，可以使用|语法匹配多个模式，它代表或的模式
fn main(){
    let x=1;
    match x{
        1|2=>println!("one or two"),
        3=>println!("three"),
        _=>println!("anything"),
    }
}
//通过..=匹配值的范围
fn main(){
    let x=5;
    match x{
        1..5=>println!("one through five"),
        _=>println!("something else"),
    }
}
char类型值
fn main(){
    let x='c';
    match x{
        'a'..='j'=>println!("early ASCII letter"),
        'k'..='z'=>println!("late ASCII letter"),
        _=>println!("something else"),
    }
}
//解构并分解值
//解构结构体
struct Point{
    x:i32,
    y:i32,
}
fn main(){
    let p=Point{x:0,y:7};
    let Point{x:a,y:b}=p;
    assert_eq!(0,a);
    assert_eq!(7,b);
}
struct Point{
    x:i32,
    y:i32
}
fn main(){
    let p=Point{x:0,y:7};
    let Point{x,y}=p;
    assert_eq!(0,x);
    assert_eq!(7,y);
}
fn main(){
    let p=Point{x:0,y:7};
    match p{
        Point{x,y:0}=>println!("On the x axis at {}",x),
        Point{x;0,y}=>println!("On the y axis at {}",y),
        Point{x,y}=>println!("On neither axis:({},{})",x,y),
    }
}
//解构枚举
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
fn main(){
    let msg=Message::ChangeColor(0,160,255);
    match msg{
        Message::Quit=>{
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move{x,y}=>{
            println!("Move in the x direction {} and in the direction {}",x,y);
        }
        Message::Write(text)=>println!("Text message: {}",text),
        Message::ChangeColor(r,g,b)=>{
            println!("Change the color to red {},green {},and blue {}",r,g,b)
        }
    }
}
//解构嵌套的结构体和枚举
enum Color{
    Rgb(i32,i32,i32),
    Hsv(i32,i32,i32),
}
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(Color),
}
fn main(){
    let msg=Message::ChangeColor(Color::Hsv(0,160,255));
    match msg{
        Message::ChangeColor(Color::Rgb(r,g,b))=>{
            println!("Change the color to red {},green {},and blue {}",r,g,b)
        }
        Message::ChangeColor(Color::Hsv(h,s,v))=>{
            println!("Change the color to hue {},saturation {},and value {}",h,s,v)
        }
        _=>()
    }
}
//忽略模式中的值
//使用_忽略整个值
fn foo(_:i32,y:i32){
    println!("This code only uses the y parameter:{}",y);
}
fn main(){
    foo(3,4);
}
//使用嵌套的_忽略部分值
fn main(){
    let mut setting_value=Some(5);
    let new_setting_value=Some(10);
    match(setting_value,new_setting_value){
        (Some(_),Some(_))=>{
            println!("Can't overwrite an existing customized value");
        }
        _=>{
            setting_value=new_setting_value;
        }
    }
    println!("Setting is {:?}",setting_value);
}
fn main(){
    let numbers=(2,4,8,16,32);
    match numbers{
        (first,_,third,_,fifth)=>{
            println!("Some numbers: {},{},{}",first,third,fifth)
        },
    }
}
//通过在名字前以一个下划线开头来忽略未使用的变量
fn main(){
    let s=Some(String::from("Hello!"));
    if let Some(_)=s{
        println!("found a string");
    }
    println!("{:?}",s);
}
//用..忽略剩余值
struct Point{
    x:i32,
    y:i32,
    z:i32
}
fn main(){
    let origin=Point{x:0,y:0,z:0};
    match origin{
        Point{x,..}=>println!("x is {}",x),
    }
}
fn main(){
    let numbers=(2,4,8,16,32);
    match numbers{
        (first,..,last)=>{
            println!("Some numbers: {},{}",first,last);
        },
    }
}
//匹配守卫提供的额外条件
fn main(){
    let num=Some(4);
    match num{
        Some(x) if x<5=>println!("less than five:{}",x),
        Some(x)=>println!("{}",x),
        None=>(),
    }
}
// @绑定允许我们在创建一个存放值的变量的同时测试其值是否匹配模式
enum Message{
    Hello{id:i32},
}
fn main(){
    let msg=Message::Hello{id:5};
    match msg{
        Message::Hello{id:id_variable @ 3..7}=>{
            println!("Found an id in range:{}",id_variable)
        },
        Message::Hello{id:10..=12}=>{
            println!("Found an id in another range")
        },
        Message::Hello{id}=>{
            println!("Found some other id: {}",id)
        },
    }
}