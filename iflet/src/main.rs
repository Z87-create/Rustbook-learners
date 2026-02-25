//if let语法让我们以一种不那么冗长的方式结合if和let，来处理只匹配一个模式的值而忽略其他模式的情况
//以下这个代码它处理的是只希望当前的值为3的情况
let some_u8_value=Some(0u8);
match some_u8_value{
    Some(3)=>println!("three"),
    //满足rust代码穷尽性的要求
    _=>(),
}
//if let的更简单写法,if let是match的一个语法糖，它当值匹配某一模式执行代码而忽略其他所有值，它的工作模式和match相同，表达式对应的是match的一个分支
if let Some(3)=some_u8_value{
    println!("three")l;
}
//前面的案例优化,可以在if let增加一个else,else块中的代码
let mut count=0;
match coin{
    Coin::Quarter(state)=>println!("State quarter from{:?}!",state),
    _=>count+=1
}
let mut count=0;
if let Coin::Quarter(state)=coin{
    println!("State quarter from {:?}!",state);
}else{
    count+=1;
}
