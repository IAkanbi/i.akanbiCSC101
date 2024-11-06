// Finding root of quadratic equation with Rust

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter coefficient of x squared (x^2): ");
    io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let a:f64 = input1.trim().parse().expect("Failed to input.");

    println!("Enter coefficient of x: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input.");
    let b:f64 = input2.trim().parse().expect("Failed to input.");

    println!("Enter value of c: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input.");
    let c:f64 = input3.trim().parse().expect("Failed to input.");

    let discriminant:f64 = (b * b) - 4.0 * a * c;

    if discriminant > 0.0{
        println!("There are two roots for this equation.");
        let root_1:f64 = (-b + discriminant.sqrt())/ (2.0 * a);
        let root_2:f64 = (-b - discriminant.sqrt())/ (2.0 * a);
        println!("The roots of this equation are {} and {}", root_1, root_2); 
    }
    else if discriminant < 0.0 {
        println!("There are no real roots.");
    }
    else if discriminant == 0.0 {
        println!("There is one real root.");
        let root:f64 = -b / 2.0 * a;
        println!("The root of the equation is {} ",root);
    }
}

