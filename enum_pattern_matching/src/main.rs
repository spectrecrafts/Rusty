#[derive(Debug)]
enum ip_addr_kind{
    v4,
    v6
}
//We can also add data into our enums by specifying the type of values using parenthesis like done below
enum ip_addr_kind_1{
    v4(String),
    //We can also pass four uint variables like v4(u8,u8,u8,u8).
    v6(String)
}

//Enums can have a wide variety of types. 
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)

}

struct ip_addr{
    kind:ip_addr_kind,
    address:String
}

fn main() {
   
   let localhost= ip_addr{
    kind:ip_addr_kind::v4,
    address:String::from("127.0.0.0")
   };

   //Now if we want to assign value using enum which accepts value, we need to do it like this.
   let localhost_1= ip_addr_kind_1::v4(String::from("127.0.0.0"));

}
fn route(ipKind:ip_addr_kind){
    print!("{:?}",ipKind);
}
