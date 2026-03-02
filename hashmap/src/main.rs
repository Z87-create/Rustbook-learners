//HashMap<K,V>类型储存了一个键类型K和一个值类型V的映射关系
//新建一个哈希map，可以使用new创建一个空的HashMap,并使用insert添加元素
use std::collections::HashMap;

let mut scores=HashMap::new();
scores.insert(String::from("Blue"),10);
scores.insert(String::from("Yellow"),50);
//也可以使用collect方法从一个包含键值对的元组数组创建一个HashMap
let teams=vec![String::from("Blue"),String::from("Yellow")];
let initial_scores=vec![10,50];
let scores:HashMap<_,_>=teams.into_iter().zip(initial_scores.into_iter()).collect();
//哈希map和所有权
let field_name=String::from("favorite color");
let field_value=String::from("Blue");
let mut map=HashMap:new();
map.insert(field_name,field_value);//当insert方法被调用时，field_name和field_value的值被移动到哈希map中，因此它们不再有效
//访问哈希map中的值
let mut scores=HashMap::new();
scores.insert(String::from("Blue"),10);
scores.insert(String::from("Yellow"),50);
let team_name=String::from("Blue");
let score=scores.get(&team_name);
//用vector类似的方式遍历哈希map也是for循环
for(key,value)in &scores{
    println!("{},{}",key,value);
}
//更新哈希map
//覆盖一个值，如果我们插入一个已经存在的键，那么原来的值会被覆盖掉
let mut scores=hashMap::new();
scores.insert(String::from("Blue"),10);
scores.insert(String::from("Blue"),25);//如果我们插入一个已经存在的键
println!("{:?}",scores);
//只在键没有对应值时插入,entry函数返回值是一个枚举，Entry,代表的是一个可能存在也可能不存在的基础上，看是否关联一个值，如果没有就插入值是50
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);
println!("{:?}",scores);
//根据旧值更新一个值
let text="hello world wonderful world";
let mut map=HashMap::new();
for word in text.split_whitespace(){
    let count=map.entry(word).or_insert(0);
    *count+=1;
}
println!("{:?}",map);
//HashMap默认使用一种叫做SipHash的哈希函数来抵御DoS攻击，使用这种算法会比

