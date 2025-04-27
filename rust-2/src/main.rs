//struct implementation
struct User {
    first_name: String,
    last_name: String,
    age: u32,
}

struct Rect {
    height: u32,
    width: u32,
}
//functions can be implemented on structs, they behave like classes and the &self is similar to the this keyword
impl Rect {
    //We can define as many functions as we like inside impl
    fn area(&self) -> u32 {
        self.height * self.width //implicit return
    }

    // A function which does not have the self parameter is a static function and does not require any object and should be called via the struct name.
    fn debug() -> u32 {
        return 1;
    }
}

fn main() {
    let user1 = User {
        first_name: String::from("Sourav"),
        last_name: String::from("Kumar"),
        age: 24,
    };
    println!(
        "Name of the user is {} {} and his age is {}",
        user1.first_name, user1.last_name, user1.age
    );

    let rect1 = Rect {
        height: 12,
        width: 8,
    };
    println!("The area of your rectangle is {}", rect1.area());

    //calling the static function
    println!("debug returns {}", Rect::debug())
}
