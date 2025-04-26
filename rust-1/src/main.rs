fn main() {
    println!("Hello, world!");
    let n: i32 = 5;

    println!("{}", is_even(n));
    println!("{}", is_even(20))
}

//write a function is_even to return true if a number is even and false if it is odd
fn is_even(num: i32) -> bool {
    return num % 2 == 0;
}
