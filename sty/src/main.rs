//结构体可以每一个部分是不同类型，结构体需要命名各部分数据以便清楚的表明各部分的含义，定义结构体需要用struct关键字并为整个结构体提供一个名字，结构体的名字需要描述它所组合的数据的意义。在大括号中，定义每一部分数据的名字和类型，我们称为字段
//结构体定义
struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}
//创建User结构体的实例
fn main(){
    let user1=User{
        email:String::from("someone@example.com"),
        username:String::from("someusername123"),
        active:true,
        sign_in_count:1,
    };
}
//改变User实例的email字段的值
fn main(){
    let mut user1=User{
        email:String::from("someone@example.com"),
        username:String::from("someusername123"),
        active:true,
        sign_incount:1,
    };
    user1.email=String::from(anotheremail@example.com);
}
//build_user函数获取email和用户名并返回User的实例
fn build_user(email:String,username:String)->User{
    User{
        email:email,
        username:username,
        active:true,
        sign_in_count:1
    }
}
//变量和字段同名的初始化简写语法
fn build_user(email:String,username:String)->User{
    User{
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}
//使用结构体更新语法从其他实例创建实例
fn main(){
    let user2=User{
        active:user1.active,
        username:user1.username,
        email:String::from("another@example.com"),
        sign_in_count:user1.sign_in_count,
    }
}
//使用没有命名字段的元组结构来创建不同的类型
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
fn main(){
    let black=Color(0,0,0);
    let origin=Point(0,0,0);
}
//没有任何字段的类单元结构体
struct AlwaysEqual;
fn main(){
    let subject=AlwaysEqual;
}