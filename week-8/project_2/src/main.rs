use std::io;

fn main() {
    let mut interviews: Vec<(String,u32)> = Vec::new(); //Empty experience vector

    let mut input1 = String::new();
    println!("Welcome to the Ernst & Young (EY) interview experience checker.");
    println!("This program will check the candidates you enter and return the one with the most experience as a software developer.");
    println!("How many developers are you interviewing?");
    io::stdin().read_line(&mut input1).expect("Invalid String");
    let interview_num:u32 = input1.trim().parse().expect("Invalid Number"); // number of elements in Vector

    for count in 0..interview_num {
        println!("Candidate {}", count+1);
        println!();
        let mut input2 = String::new();
        println!("Candidate Name: ");
        io::stdin().read_line(&mut input2).expect("Invalid String");
        let name:String = input2.trim().parse().expect("Invalid Input");

        println!("Years of experience:");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Invalid String");
        let exp:u32 = input3.trim().parse().expect("Invalid Number");
        interviews.push((name,exp));
    }

    if let Some((name, exp)) = interviews.iter().max_by_key(|(_, exp)| exp){
        println!("The candidate with the most experience is {} with {} years of experience", name,exp);
    }
    else{
        println!("No candidates were entered");
    }

}
