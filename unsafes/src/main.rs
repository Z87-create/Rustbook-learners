//Rust还有一种语言，因为静态分析编译器是保守的，所以我们要用unsafe来进行处理
//解引用裸指针
//创建裸指针
let mut num=5;
let r1=&num as *count i32;
let r2=&mut num as *mut i32;
指向任何内存地址的裸指针
let address=0x012345usize;
let r=address as *count i32;
//在unsafe块中解引用裸指针
fn main(){
let mut num=5;
let r1=&num as *const i32;
let r2=&mut num as *mut i32;
unsafe{
    println!("r1 is:{}",*r1);
    println!("r2 is:{}",*r2);
}
}
//调用不安全函数和方法
unsafe fn dangerous(){}
unsafe{
    dangerous();
}
//创建不安全代码的安全抽象
fn main(){
    let mut v=vec![1,2,3,4,5,6];
    let r=&mut v[..];
    let(a,b)=r.split_at_mut(3);
    assert_eq!(a,&mut[1,2,3]);
    assert_eq!(b,&mut[4,5,6]);
}
//在split_at_mut函数的实现中使用不安全代码
use std::slice;
fn split_at_mut(slice:&mut[i32],mid:usize)->(&mut [i32],&mut[i32]){
    let len=slice.len();
    let ptr=slice.as_mut_ptr();
    assert_eq!(mid<=len);
    unsafe{
        (slice::from_raw_parts_mut(ptr,mid),
        slice::from_raw_parts_mut(ptr.add(mid),len-mid))
    }
}
通过任意内存地址创建slice
use std::slice;
let address=0x01234usize;
let r=address as *mut i32;
let slice:&[i32]=unsafe{
    slice::from_raw_parts_mut(r,10000)
};
//使用extern函数调用外部代码
unsafe extern "C"{
    fn abs(input:i32)->i32;
}
fn main(){
    unsafe{
        println!("Absolute value of -3 according to C:{}",abs(-3));
    }
}
//访问或修改可变静态变量
static HELLO_WORLD:&str="Hello,world!";
fn main(){
    println!("name is:{}",HELLO_WORLD);
}
static mut COUNTER: u32=0;
fn add_to_count(inc:u32){
    unsafe{
        COUNTER+=inc;
    }
}
fn main(){
    add_to_count(3);
    unsafe{
        println!("COUNTER: {}",COUNTER);
    }
}
//实现不安全trait
unsafe trait foo{

}
unsafe impl Foo for i32{

}