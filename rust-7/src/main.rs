//Borrowing references

fn create_string() {
    let mut s1: String = String::from("Hello");
    //Using the reference of the variable instead of the variable itself
    print_string(&s1); //s1 is the owner
    let s2: &String = &s1;
    println!("{}", *s2); //we can either write *s2 or just s2, both are the same *s2 is dereferencing
    println!("{}", s1); //s1 is the owner
    print_mut(&mut s1) // borrow and mutate i.e borrow and also make changes

    //At any time you can either have on mutable reference or any number of immutable references

    //let s2= mut &s1;
    //let s3= &s1; This code gives error as s2 is mutable reference of s1 and it causes memory conflicts
}

fn print_string(s2: &String) {
    println!("{}", s2) //s2 owns the value here
}

fn print_mut(s2: &mut String) {
    s2.push_str(" World");
    println!("{}", s2);
}

fn main() {
    create_string();
}
