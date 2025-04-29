use std::collections::HashMap;

fn main() {
    let mut users: HashMap<String, i32> = HashMap::new();
    users.insert(String::from("Sourav"), 23);
    users.insert(String::from("Riya"), 21);

    let user1 = users.get("Sourav"); //The get function returns an option
                                     // println!("{}", user1.unwrap());
    match user1 {
        Some(value) => println!("{}", value),
        None => println!("User not found"),
    }

    //input for the get_hashed funtion
    let pair: Vec<(String, i32)> =
        vec![(String::from("harkirat"), 21), (String::from("raman"), 20)];
    let hashed_output = get_hashed(&pair);
    println!("{:?}", hashed_output);
}

//Write a function that takes vector of tuples (each containing a key and a value) and returns a hashmap where the keys are the unique keys from the input tuples and the values are vectors of all corresponding values associated with the key
fn get_hashed(vec: &Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut output_hash: HashMap<String, i32> = HashMap::new();

    for (key, value) in vec {
        output_hash.insert(key.to_string(), *value);
    }
    return output_hash;
}
