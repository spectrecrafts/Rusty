//Suppose you have a function which returns either a value or null then what will be the type of that function ? We use options which is default enum provided by rust.

// structure of option enum
// enum Option<T>{
//     None,
//     Some(T)
// }

//The result enum is used for error handling. It allows you to return either okk or err. In javascript we can throw a custom error

enum CustomOption {
    Some(i32),
    None,
}

fn length_check(s: String) -> CustomOption {
    if s.chars().count() > 0 {
        return CustomOption::Some(s.chars().count() as i32);
    }
    return CustomOption::None;
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

//function to read the contents of a file. This is good example of a code that could return an error.
use std::fs; //library for file handling

fn main() {
    let my_string = String::from("raman");
    //pattern matching
    match find_first_a(my_string.clone()) {
        Some(index) => println!("The letter a is found at {}", index),
        None => println!("The letter 'a' could not be found in the string"),
    }

    //custom enum option
    match length_check(my_string.clone()) {
        CustomOption::Some(value) => println!("The length of the string is {}", value),
        CustomOption::None => println!("The string is not valid"),
    }

    //result enum
    let result: Result<String, std::io::Error> = fs::read_to_string("hello.txt");
    match result {
        Ok(data) => println!("{}", data),
        Err(_err) => println!("Error while reading the file"),
        //you can throw an error in rust using panic!("File could not be read")
    }
}
