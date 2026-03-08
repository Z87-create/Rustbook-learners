use std::env;
use std::process;
use minigrep::Config;
fn main(){
    let args:Vec<String>=env::args().collect();
    let config=Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments:{}",err);
        process::exit(1);
    });
    // println!("Problem parsing arguments:{}",err);
    // process::exit(1);
    // run(config);
    // let query=&args[1];
    // let filename=&args[2];
    // println!("{:?}",args);
    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);
    if let Err(e)=minigrep::run(config){
        eprintln!("Application error:{}",e);
        process::exit(1);
    }
    // if let Err(e)=run(Config){
    //     println!("Application error:{}",e);
    //     process::exit(1);
    // }
    // let contents=fs::read_to_string(filename).expect("Something went wrong reading the file");
    // println!("With text:\n{}",contents);
}
fn parse_config(args:&[String])->(&str,&str){
    let query=&args[1];
    let filename=&args[2];
    (query,filename)
}