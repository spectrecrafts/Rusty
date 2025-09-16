#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6
}
//We can also add data into our enums by specifying the type of values using parenthesis like done below
enum IpAddrKind1{
    V4(String),
    //We can also pass four uint variables like v4(u8,u8,u8,u8).
    V6(String)
}

//Enums can have a wide variety of types. 
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)

}
//We could define separate structs for these datatypes but these would be independant and not related to each other so we declare them as enums. We can write impl functions as in a struct.
impl Message{
    fn _some_function(){
        println!("Let's get rusty")
    }
}

struct IpAddr{
    Kind:IpAddrKind,
    Address:String
}

fn main() {
   
   let _localhost= IpAddr{
    Kind  :IpAddrKind::V4,
    Address:String::from("127.0.0.0")
   };

   //Now if we want to assign value using enum which accepts value, we need to do it like this.
   let _localhost_1= IpAddrKind1::V4(String::from("127.0.0.0"));

   hello();
   enum_matching();
}

//option enum
fn hello(){
    // enum Option<T>{
    //     Some(T),
    //     None
    // }
    //Whenever we know that a value can be null, we wrap it inside the option enum.
    //Examples of using option enum
    let some_number= Option::Some(4);
    let some_string= Option::Some(String::from("Hello world"));
    //As none does not  have any type, we need to manually annotate the type of the none value.
    let absent_number: Option<i32>=Option::None;


    //Let's understand the difference between an normal integer and an integer from option enum.
    let custom_integer= 5;
    let option_integer: Option<i32> = Option::Some(5);


    
    // let sum= custom_integer+option_integer;
    
    //The above line when uncommented gives error as 
    
    //lets suppose, we want to use a default value if the optional integer is not available. then we can use
    let sum= custom_integer+option_integer.unwrap_or(0); 
    println!("The sum of an optional integer and a custom integer is {}",sum);
}


enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn enum_matching(){
    let coin= Coin::Nickel;
    let answer= match coin{
        Coin::Penny=>1,
        Coin::Nickel=>50,
        Coin::Quarter=>100,
        Coin::Dime=>200
    };
    println!("{}",answer)

}