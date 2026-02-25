enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin:Coin)->u8{
    match coin{
        //=>运算符将模式和将要运行的代码分开，这里的代码仅仅为1，每一个分支之间用,隔开
        Coin::Penny=>1,
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin:Quarter=>25,
    }
}
//绑定值的模式
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin:Coin)->u8{
    match coin{
        Coin::Penny=>1,
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quarter(state)=>{
            println!("State quarter from{:?}!",state);
            25
        }
    }
}