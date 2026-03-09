pub fn add_two(a:i32) -> i32 {
    a+2
}
pub fn greeting(name:&str)->String{
    format!("Hello {}!",name)
}
pub struct Guess{
    value:i32,
}
impl Guess{
    pub fn new(value:i32)->Guess{
        if value<1||value>100{
            panic!("Guess value must be between 1 and 100,got {}",value);
        }
        Guess{
            value
        }
    }
}


#[cfg(test)]//告诉rust只在执行 cargo test时才运行和编译代码，在cargo build时不这么做
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(200);
    }
    #[test]
    fn it_works()->Result<(),String>{
        if 2+2==4{
            Ok(())
        }else{
            Err(String::from("two plus two does not equal four"))
        }
    }

    // #[test]
    // fn exploration() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
    // #[test]
    // fn another(){
    //     panic!("Make this test fail");
    // }
    //使用assert!宏来检查结果
    #[derive(Debug)]
    struct Rectangle{
        width:u32,
        height:u32,
    }
    impl Rectangle{
        fn can_hold(&self,other:&Rectangle)->bool{
            self.width>other.width && self.height>other.height
        }
    }
    #[test]
    fn larger_can_hold_smaller(){
        let larger=Rectangle{width:8,height:7};
        let smaller=Rectangle{width:5,height:1};
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn it_adds_two(){
        assert_eq!(4,add_two(2));
    }
    #[test]
    fn greeting_contains_name(){
        let result=greeting("Carol");
        assert!(result.contains("Carol"),"Greeting did not contain name,value was `{}`",result);
    }
    #[test]
    fn add_two_and_two(){
        assert_eq!(4,add_two(2));
    }
    #[test]
    fn add_three_and_two(){
        assert_eq!(5,add_two(3));
    }
    #[test]
    fn one_hundred(){
        assert_eq!(102,add_two(100));
    }
}
//测试一般是并行运行的，如果不想并行运行应该输入这个指令 cargo testn-- --test-thread==1
//显示函数输出的指令是cargo test -- --show-output
//运行单个测试应该 cargo test add_two_and_two,对于想要忽视的测试上面应该加#[ignore]
//过滤运行多个测试应该 cargo test add_two
//需要运行过滤的测试可以使用cargo test -- --ignored

