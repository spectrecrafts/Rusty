pub struct Config{
    pub query:String,
    pub filename:String
}
impl Config{

   pub  fn new(args:Vec<String>)->Result<Config,&'static str>{
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
