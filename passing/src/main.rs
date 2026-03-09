use std::thread;
use std::sync::mpsc;
use std::time::Duration;
// fn main(){
//     let(tx,rx)=mpsc::channel();
//     thread::spawn(move||{
//         let val=String::from("hi");
//         tx.send(val).unwrap();
//     });
//     //在主线程中接受并打印内容hi,通道的接收端有两个接受方法:recv和try_recv，recv会阻塞主线程执行直到从通道中接受一个值，一旦发送一个值，recv会在一个Result<T,E>中返回它，try_recv不会阻塞，相反它立即返回一个Result<T,E>:Ok值包含可用的信息，Err值代表此时没有任何信息
//     let received=rx.recv().unwrap();
//     println!("Got: {}",received);
// }
fn main(){
    let (tx,rx)=mpsc::channel();
    //通过克隆发送者来创建多个生产者
    let tx1=tx.clone();
    thread::spawn(move||{
        let vals=vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move||{
        let vals=vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx{
        println!("Got:{}",received);
    }
}
