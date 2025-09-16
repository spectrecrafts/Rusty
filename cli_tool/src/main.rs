use std::env;
use std::fs;
fn main(){

    let args:Vec<String>=env::args().collect();
    let config= Config::new(args.clone());
    println!("{:?}",args);
    let  contents;
    match config{
        Ok(config)=> {
             contents= fs::read_to_string(config.filename).expect("Something went wrong reading the file"); println!("The query was {}",config.query); println!("{}",contents);},
        Err(res)=> println!("{}",res)
    }

}

struct Config{
    query:String,
    filename:String
}
impl Config{

    fn new(args:Vec<String>)->Result<Config,&'static str>{
        if args.len()<3 {
            return Err("not enough arguments");
        }
        else{
        let query= args[1].clone();
        let filename= args[2].clone();
       return Ok( Config {query,filename});
        }

    }
  
}
