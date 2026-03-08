#[test]
fn iterator_demonstration(){
    let v1=vec![1,2,3];
    let mut v1_iter();
    assert_eq!(v1_iter.next(),Some(&1));
    assert_eq!(v1_iter.next(),Some(&2));
    assert_eq!(v1_iter.next(),Some(&3));
    assert_eq!(v1.iter().next(),None);
}
//消费迭代器的方法，消费适配器一个消费适配器的方法就是sum方法
#[test]
fn iterator_sum(){
    let v1=vec![1,2,3];
    let v1_iter=v1.iter();
    let total:i32=v1_iter.sum();
    assert_eq!(total,6);
}
//使用闭包适配环境，展示一个filter迭代器适配器和捕获环境的闭包的常规案例，获取的是一个返回布尔值的闭包，返回true的闭包其值包含在filter提供的新迭代器中，返回false，其值不会包含在结果迭代器中
#[derive(PartialEq,Debug)]
struct Shoe{
    size:u32,
    style:String,
}
fn shoes_in_my_size(shoes:Vec<Shoe>,shoe_size:u32)->Vec<Shoe>{
    shoes.into_iter()
    .filter(|s| s.size==shoe_size)
    .collect()
}
#[test]
fn filters_by_size(){
    let shoes=vec![
        Shoe{size:10,style:String::from("sneaker")},
        Shoe{size:13,style:String::from("sandal")},
        Shoe{size:10,style:String::from("boot")},
    ];
    let in_my_size=shones_in_my_size(shoes,10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe{size:10,style:String::from("sneaker")},
            Shoe{size:10,style:String::from("boot")},
        ]
    );
}
//使用自定义迭代器
struct Counter{
    count:u32,
}
impl Counter{
    fn new()->Couter{
        Counter{count:0}
    }
}
impl Iterator for Counter{
    type Item=u32;
    fn next(&mut self)->Option<Self::Item>{
        self.count+=1;
        if self.count<6{
            Some(self.count)
        }else{
            None
        }
    }
}
//使用Counter迭代器的next方法
#[test]
fn calling_next_directly(){
    let mut counter=Counter::new();
    assert_eq!(counter.next(),Some(1));
    assert_eq!(counter.next(),Some(2));
    assert_eq!(counter.next(),Some(3));
    assert_eq!(counter.next(),Some(4));
    assert_eq!(counter.next(),Some(5));
    assert_eq!(counter.next(),None);
}
//使用自定义迭代器中其他Iterator trait方法
#[test]
fn using_iterator_trait_methods(){
    let sum:u32=Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a,b)|a*b)
    .filter(|x| x%3==0)
    .sum();
    assert_eq!(10,sum);
}