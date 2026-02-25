//用mod定义模块，指定模块名字为front_of_house，并用大括号包围模块主题，我们可以在模块里包含其它的模块
mod front_of_house{
    //模块内部的模块是私有化的private，如果内部想公有化，需要加pub前缀
   pub mod hosting{
        pub fn add_to_waitlist(){}
   }
    //     fn sent_at_table(){}
    // mod serving{
    //     fn take_order(){}
    //     fn serve_order(){}
    //     fn take_payment(){}
    // }
}
pub fn eat_at_restaurant(){
    //绝对路径
    crate::front_of_house::hosting::hosting::add_to_waitlist();
    //相对路径
    front_of_house::hosting::add_to_waitlist();
}
//super开头构建父模块开始的相对路径
fn serve_order(){}
mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }
    fn cook_order(){}
}
//我们还可以利用pub来设计公有的结构体和枚举，但是结构体的具体字段仍然是私有的
mod back_of_house{
    pub struct Breakfast{
        pub toast:String,
       seasonal_fruit:String, 
    }
    impl Breakfast{
        pub fn summer(toast:&str)->Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant(){
    //在夏天点一份黑麦面包作为早餐
    let mut meal=back_of_house::Breakfast::summer("Rye");
    //更改我们想要的面包
    meal.toast=String::from("wheat");
    println!("i'd like {} toast please",meal.roast);
    //如果取消下一行的注释，将会导致编译失败；我们不被允许看到更改随餐搭配的季节水果

}
//如果我们将枚举设置为共有，里面的所有元素自然会被视为公有，不用人为加pub
mod back_of_house{
    pub enum Appetizer{
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant(){
    let order1=back_of_house::Appetizer::Soup;
    let order2=back_of_house::Appetizer:Salad;
}