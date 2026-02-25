//定义枚举,枚举应用于大量数据精确分类，比如说IP地址分类，只有IPV4或者IPV6两种类型，所以V4和V6是这个枚举的成员
enum IpAddKind{
    v4,
    v6
}
//创建枚举值，确保IpAddrKind类型捆绑的是v4,v6这种
let four=IpAddKind::v4;
let six=IpAddKind::v6;
//定义一个函数来获取任何IpAddKind
fn route(ip_type:IpAddKind){}
//使用任意成员调用这个函数
route(IpAddKind::v4);
route(IpAddKind::v6);
//将枚举成员
struct IpAddr{
    kind:IpAddKind,
    address:String,
}
let home=IpAddr{
    kind:IpAddKind::v4,
    address:String::from("127.0.0.1"),
};
let loopback=IpAddr{
    kind:IpAddKind::v6,
    address:String::from("::1"),
};
//简介写法:使用枚举将数据直接放进每一个枚举成员，这样我们都不用额外用结构体了
enum IppAddr{
    v4(String),
    v6(String),
}
let home=IpAddr::v4(String::from("127.0.0.1"));
let loopback=IpAddr::v6(String::from("::1"));
//使用枚举处理不同类型和数量和数据，这就是枚举比起结构体更优越的地方
enum IpAddr{
    v4(u8,u8,u8,u8),
    v6(String),
}
let home=IpAddr::v4(127,0,0,1);
let loopback=IpAddr::v6(String::from("::1"));
//内嵌多种多样类型的枚举
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
//枚举同样可以像结构体一样定义方法，定义一个名为call方法
impl Message{
    fn call(&self){
        //这里定义方法体
    }
    let m=Message::Write(String::from("hello"));
    m.call();
}