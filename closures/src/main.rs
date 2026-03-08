//Rust的闭包是可以保存进变量或作为参数传递给其他函数的匿名函数
use std::thread;
use std::time::Duration;
// fn simulated_expensive_calculation(intensity:u32)->u32{
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

struct Cacher<T>
        where T:Fn(u32)->u32
        {
            calculation:T,
            value:Option<u32>,
        }
        impl<T> Cacher<T>
        where T:Fn(u32)->u32
        {
            fn new(calculation:T)->Cacher<T>{
                Cacher{
                    calculation,
                    value:None,
                }
            }
            fn value(&mut self,arg:u32)->u32{
                match self.value{
                    Some(v)=>v,
                    None=>{
                        let v=(self.calculation)(arg);
                        self.value=Some(v);
                        v
                    },
                }
            }
//程序的业务逻辑，它根据输入并调用simulated_expensive_calculation函数打印健身计划
fn generate_workout(intensity:u32,random_number:u32){
    // let expensive_result=simulated_calculation(intensity);
    //使用闭包来代替函数调用
    // let expensive_closure=|num:u32|{
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // }
    //使用带有泛型和Fn trait的闭包
    let mut expensive_result=Cacher::new(|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
        }
    if intensity<25{
        println!("Today,do {} pushups!",expensive_result);
    };
    println!("Next,do {} situps!",expensive_result);
}else{
    if random==3{
        println!("Take a break day! Remember to stay hydrated!");
    }else{
        println!("Today,run for {} minutes!",expensive_result);
    }
}
//main函数包含了用于generate_workout函数的模拟用户输入和模拟随机数输入
fn main(){
    let simulated_user_specified_value=10;
    let simulated_random_number=7;
    generate_workout(simulated_user_specified_value,simulated_random_number);
}