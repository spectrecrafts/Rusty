fn create_string() {
    let a1: String = String::from("hello world");
    // let a2: String = a1; ownership of the string in the heap can only be with one variable and once we write this line the owner of the string becomes a2 and a1 is invalid and that's why it throws compile time errors. we can do
    //let a2:String= a1.clone() as that would create another memory allocation for the string with a2 as the owner.

    //We can completely do this. We can declare a2 in a different scope

    // print_str(a1);
    //The above line also does the same thing, it moves the ownership of the variable from a1 to s2 and that's why we see the error in the below print statement.
    //The alternative is
    print_str(a1.clone()); //PS : There are better ways to do this

    println!("{}", a1);
}

fn print_str(s2: String) {
    println!("{}", s2);
}
fn main() {
    create_string();
}

//refer the mermory_leak.cpp file to know about memory management in cpp
