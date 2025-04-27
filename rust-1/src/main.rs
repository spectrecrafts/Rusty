fn main() {
    println!("Hello, world!");
    let n: i32 = 5;
    println!("{}", is_even(n));
    println!("{}", fibonacci(5));

    //Taking string as an input
    let s = String::from("Hello world");
    println!("{}", get_length(&s))
}

//write a function is_even to return true if a number is even and false if it is odd
fn is_even(num: i32) -> bool {
    return num % 2 == 0;
}

//write a function to give the fibonacci series number
fn fibonacci(num: i32) -> i32 {
    // let mut first = 0;
    // let mut second = 1;

    let (mut first, mut second) = (0, 1);
    if num == 0 {
        return 0;
    }
    if num == 1 {
        return 1;
    }
    for _ in 1..num - 2 {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}

fn get_length(s: &str) -> usize {
    s.chars().count() //implicit return type this function automatically returns the statement
}
