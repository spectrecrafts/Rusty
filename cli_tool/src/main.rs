use std::env;
use std::fs;
use std::process;

use cli_tool::Config;
fn main(){

    let args:Vec<String>=env::args().collect();
    let config= Config::new(args.clone()).unwrap_or_else(|err|{
    process::exit(1);
    });
    println!("{:?}",args);
    let filename= Config::new(args).expect("Could not read file");
    let  contents=fs::read_to_string(filename.filename);
    println!("{:?}",contents);
}   

