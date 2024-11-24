use std::io;

fn main() {
    let mut order = String::new();
    let mut quantity = String::new();
    let poundo:f32 = 3200.0;
    let f_rice:f32 = 3000.0;
    let amala:f32 = 2500.0;
    let eba:f32 = 2000.0;
    let w_rice:f32 = 2500.0;

    println!("Welcome to Naija Classics.");
    println!("Today's menu will be:
P - Poundo Yam/Edinkaiko Soup = N3,200
F - Fried Rice & Chicken = N3,000
A - Amala & Ewedu Soup = N2,500
E - Eba & Egusi Soup = N2,000
W - White Rice & Stew = N2,500");

    println!("What would you like to order? (P, F, A, E, W.)");
    io::stdin().read_line(&mut order).expect("Invalid String");
    let order = order.trim().to_uppercase();

    println!("How many plates would you like?");
    io::stdin().read_line(&mut quantity).expect("Invalid String");
    let quantity:f32 = quantity.trim().parse().expect("Invalid Number");

    if order == "P" {
        let amount:f32 = poundo * quantity;
        if amount > 10_000.0{
            let discount:f32 = 0.05 * amount;
            let final_price:f32 = amount - discount;
            println!("You've spent over N10,000. You have earned a 5% discount. Your total is {}", final_price);
        } else {
            println!("You're final total is {}", amount);
        }
    } else if order == "F" {
        let amount:f32 = f_rice * quantity;
        if amount > 10_000.0{
            let discount:f32 = 0.05 * amount;
            let final_price:f32 = amount - discount;
            println!("You've spent over N10,000. You have earned a 5% discount. Your total is {}", final_price);
        } else{
            println!("Your final total is {}",amount);
        }
    } else if order == "A"{
        let amount:f32 = amala * quantity;
        if amount > 10_000.0{
            let discount:f32 = 0.05 * amount;
            let final_price:f32 = amount - discount;
            println!("You've spent over N10,000. You have earned a 5% discount. Your total is {}", final_price);
        } else{
            println!("Your final total is {}",amount);
        }
    } else if order == "E"{
        let amount:f32 = eba * quantity;
        if amount > 10_000.0{
            let discount:f32 = 0.05 * amount;
            let final_price:f32 = amount - discount;
            println!("You've spent over N10,000. You have earned a 5% discount. Your total is {}", final_price);
        } else{
            println!("Your final total is {}",amount);
        }
    } else if order == "W" {
        let amount:f32 = w_rice * quantity;
        if amount > 10_000.0{
            let discount:f32 = 0.05 * amount;
            let final_price:f32 = amount - discount;
            println!("You've spent over N10,000. You have earned a 5% discount. Your total is {}", final_price);
        } else{
            println!("Your final total is {}",amount);
        }
    } else{
        println!("We do not have this food in our establishment. Please try somewhere else, thank you.");
    }

}
