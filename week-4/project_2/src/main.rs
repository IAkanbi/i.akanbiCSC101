use std::io;

fn main() {
    let mut exp = String::new();
    let mut age = String::new();

    println!("Please state your level of experience (experienced or inexperienced): ");
    io::stdin().read_line(&mut exp).expect("Failed to read input.");
    let exp = exp.trim().to_lowercase();

    println!("Enter your age: ");
    io::stdin().read_line(&mut age).expect("Invalid String.");
    let age:i64 = age.trim().parse().expect("Not a valid number");

    if exp == "experienced" && age >= 40 {
        println!("Your annual incentive is 1,560,000 naira.");
    }
    else if exp == "experienced" && age >=30 && age <40 {
        println!("Your annual incentive is 1,480,000 naira.");
    }
    else if exp == "experienced" && age < 28 {
        println!("Your annual incentive is 1,300,000 naira.");
    }
    else if exp == "inexperienced" {
        println!("Your annual incentive is 100,000 naira.");
    }
}
