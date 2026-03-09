//互斥器一次只允许一个线程访问数据，线程首先需要通过获取互斥器的锁来表明其希望访问数据，它记录谁有数据的排他访问权，我们描述互斥器为通过所系统保护其数据
//互斥器的两个特性：1.在使用数据前尝试获取锁，2.处理完被互斥器所保护的数据之后，必须解锁数据，这样其他线程才能够获取锁
// use std::sync::Mutex;
// fn main(){
//     let m=Mutex::new(5);
//     {
//         let mut num=m.lock().unwrap();
//         *num=6;
//     }
//     println!("m = {:?}",m);
// }
//原子引用计数Arc<T>正是一个类似Rc<T>,并可以安全用于并发环境的类型，字母"a"代表原子性，所以这是一个原子引用类型
use std::sync::{Mutex,Arc};
use std::thread;
fn main(){
    let counter=Arc::new(Mutex::new(0));
    let mut handles=vec![];
    for_in0..10{
        let counter=Arc::clone(&counter);
        let handle=thread::spawn(move||{
            let mut num=counter.lock().unwrap();
            *num+=1;
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    println!("Result:{}",*counter.lock().unwrap());
}