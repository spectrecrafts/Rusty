//Enums lets us enumerate over various types of a value

// enum Shape {
//     Rectangle,
//     Circle,
// }

//enums with values

enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn main() {
    print_area(Shape::Circle(4.0));
    print_area(Shape::Rectangle(4.0, 6.0));
}

fn print_area(shape: Shape) {
    //pattern matching syntax for an enum
    let area = match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle(a, b) => a * b,
    };
    println!("The area of the shape is {}", area)
}
