fn main() {
    vector();
}

fn vector() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    println!("{:?}", vec); // We cannot really print structs automatically so we need to use "{:?}" syntax

    //Another way to initialize a vector
    // let numbers :Vec<i32> = vec![1, 2, 3]; vec is a macro do not confuse it with the variable name. We use the '!' sign. Also the type of vector is given through a generic

    even_values(&mut vec); //Using while loop
    let ans_vec = return_even(&vec); //Using for loop
    println!("{:?}", ans_vec);
}
fn return_even(vec: &Vec<i32>) -> Vec<&i32> {
    // for val in vec {
    //     if val % 2 == 0 {
    //         print!("{} ", val);
    //     }
    // }
    let mut new_vec = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }
    return new_vec;
}
fn even_values(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        }
        i += 1;
    }
    println!("{:?}", vec);
}
