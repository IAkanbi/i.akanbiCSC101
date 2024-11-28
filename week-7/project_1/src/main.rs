use std::io;

fn main() {
    let mut shape = String::new();
    let mut height = String::new();
    let mut base_1 = String::new();
    let mut base_2 = String::new();
    let mut base = String::new();
    let mut altitude = String::new();
    let mut diagonal_1 = String::new();
    let mut diagonal_2 = String::new();
    let mut length = String::new();
    let mut radius = String::new();

    println!("Welcome! This program will assist you in making some calculations\nfor a select few shapes.");
    println!("Please select which shape you will be calculating:");
    println!();
    println!("1 ---- Area of Trapezium");
    println!("2 ---- Area of Rhombus");
    println!("3 ---- Area of Parallelogram");
    println!("4 ---- Area of Cube");
    println!("5 ---- Volume of Cylinder");
    io::stdin().read_line(&mut shape).expect("Invalid String");
    let shape = shape.trim();

    if shape == "1" {
        println!("Please enter the height: ");
        io::stdin().read_line(&mut height).expect("Invalid String");
        let height:f32 = height.trim().parse().expect("Invalid Number");


        println!("Please enter base 1: ");
        io::stdin().read_line(&mut base_1).expect("Invalid String");
        let base_1:f32 = base_1.trim().parse().expect("Invalid Number");


        println!("Please enter base 2: ");
        io::stdin().read_line(&mut base_2).expect("Invalid String");
        let base_2:f32 = base_2.trim().parse().expect("Invalid Number");

        trapezium(height,base_1,base_2)
    }
    else if shape == "2" {
        println!("Please enter value for diagonal 1: ");
        io::stdin().read_line(&mut diagonal_1).expect("Invalid String");
        let diagonal_1:f32 = diagonal_1.trim().parse().expect("Invalid Number");

        println!("Please enter value for diagonal 2: ");
        io::stdin().read_line(&mut diagonal_2).expect("Invalid String");
        let diagonal_2:f32 = diagonal_2.trim().parse().expect("Invalid Number");

        rhombus(diagonal_1,diagonal_2)
    }
    else if shape == "3" {
        println!("Please enter the value for your base: ");
        io::stdin().read_line(&mut base).expect("Invalid String");
        let base:f32 = base.trim().parse().expect("Invalid Number");

        println!("Please enter the value for your altitude: ");
        io::stdin().read_line(&mut altitude).expect("Invalid String");
        let altitude:f32 = altitude.trim().parse().expect("Invalid Number");

        parallelogram(base, altitude);
    }
    else if shape == "4"{
        println!("Please enter the length of the cube: ");
        io::stdin().read_line(&mut length).expect("Invalid String");
        let length:f32 = length.trim().parse().expect("Invalid Number");

        cube(length);

    }
    else if shape == "5"{
        println!("Please enter the radius of the Cylinder: ");
        io::stdin().read_line(&mut radius).expect("Invalid String");
        let radius:f32 = radius.trim().parse().expect("Invalid Number");

        println!("Please enter the height: ");
        io::stdin().read_line(&mut height).expect("Invalid String");
        let height:f32 = height.trim().parse().expect("Invalid Number");

        cylinder(radius,height);
    }
    else{
        println!("Selection not available.");
    }

}

fn trapezium(height:f32, base_1:f32, base_2:f32) {
    let area:f32 = (height/2.0) * (base_1 + base_2);
    println!("The Area of your trapezium is {}",area);
}

fn rhombus(diagonal_1:f32, diagonal_2:f32){
    let area:f32 = 0.5 * diagonal_1 * diagonal_2;
    println!("The area of your rhombus is {}", area);
}
fn parallelogram(base:f32, altitude:f32){
    let area:f32 = base * altitude;
    println!("The area of your Parallelogram is {}", area);
}
fn cube(length:f32){
    let area:f32 = 6.0 * length.powf(2.0);
    println!("The area of your Cube is {}",area);
}
fn cylinder(radius:f32, height:f32){
    let volume:f32 = (22.0/7.0) * radius.powf(2.0) * height;
    println!("The volume of your Cylinder is {} ", volume);
}