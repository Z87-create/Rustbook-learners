//使用spawn创建新线程
use std::thread;
// use std::time::Duration;
// fn main(){
//    let handle = thread::spawn(||{
//         for i in 1..100{
//             println!("hi number {} from the spawned thread!",i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//      handle.join().unwrap();
//     for i in 1..5{
//         println!("hi number {} from the main thread!",i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     //使用join等待所有线程结束
//     // handle.join().unwrap();
// }
fn main(){
    let v=vec![1,2,3];
    let handle=thread::spawn(move || {
        println!("Here's a vector: {:?}",v);
    });
    handle.join().unwrap();
}