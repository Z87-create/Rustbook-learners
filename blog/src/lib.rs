pub struct Post{
    state:Option<Box<dyn State>>,
    content:String,
}
impl Post{
    pub fn approve(&mut self){
        if let Some(s)=self.state.take(){
            self.state=Some(s.approve())
        }
    }
    pub fn new()->Post{
        Post{
            state:Some(Box::new(Draft{})),
            content:String::new(),
        }
    }
    pub fn request_review(&mut self){
        if let Some(s)=self.state.take(){
            self.state=Some(s.request_review())
        }
    }
    pub fn content(&self)->&str{
        self.state.as_ref().unwrap().content(self)
    }
}
    pub fn add_text(&mut self,text:&str){
        self.content.push_str(text);
    }
    pub fn content(&self)->&str{
        
    }
trait State{
    fn request_review(self:Box<Self>)->Box<dyn State>;
    fn approve(self:Box<self>)->Box<dyn State>;
    fn content<'a>(&self,post:&'a Post)->&'a str{
        ""
    }
}
struct Draft{}
impl State for Draft{
    fn request_review(self:Box<Self>)->Box<dyn State>{
        self
    }
}
struct PendingReview{}
impl State for PendingReview{
    fn approve(self:Box<Self>)->Box<dyn State>{
        Box::new(Published{})
    }
}
struct Published{}
impl State for Published{
    fn request_review(self:Box<Self>)->Box<dyn State>{
        self
    }
    fn approve(self:Box<Self>)->Box<dyn State>{
        self
    }
}
struct Published{}
impl State for Published{
    fn content<'a>(&self,post:&'a Post)->&'a str{
        &post.content
    }
}