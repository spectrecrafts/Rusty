//Iterators
fn main() {
    //for (key,value) in vec{
    //  We are not iterating over a vector but rather an iterator in this loop
    //}
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
    //The v1 vector is still valid because the iter() borrows the values, the ownership is still with the v1 vector.
    println!("{:?}", v1);
}
