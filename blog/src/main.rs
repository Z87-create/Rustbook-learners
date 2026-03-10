//状态模式是一个面向对象的设计模式，该模式的关键在于一个值有某些内部状态，体现为一系列的状态对象，同时值的行为随着其内部状态而改变
use blog::Post;
fn main(){
    let mut post=Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("",post.content());
    post.request_review();
    assert_eq!("",post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today",post.content());
}