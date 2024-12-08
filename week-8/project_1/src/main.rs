use std::io;

fn main() {
    let staff_job =  vec!["office administrator", "academic", "lawyer", "teacher"];
    let office_admin = vec!["Intern","Administrator","Senior Administrator","Office Manager", "Director", "CEO"];
    let academic = vec!["N/A", "Research Assistant", "PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
    
    println!("Welcome to the Public Service APS level checker developed by the Federal Government of Nigeria.");
    println!("This program will ask for your field and years of experience and respond with your position.");
    println!();
    println!("Which public servant do you classify under: (Office Administrator, Academic, Lawyer, Teacher)");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid String");
    let staff_answer = input1.trim().to_lowercase();

    if staff_job.iter().any(|job| job.to_lowercase() == staff_answer)
    {
        println!("How many years of experience do you have in this field?");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Invalid String");
        let staff_exp:u32 = input2.trim().parse().expect("Invalid Number");
        if staff_answer == "office administrator"{
            if (1..=2).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}",staff_exp, office_admin[0]);
            }
            else if (3..=5).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, office_admin[1]);
            }
            else if (6..=8).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, office_admin[2]);
            }
            else if (9..=10).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, office_admin[3]);
            }
            else if (11..=13).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, office_admin[4]);
            }
            else if staff_exp > 13{
                println!("Congratulations, with your {} years of experience, you qualify for the position of {}", staff_exp, office_admin[5]);
            }
        }
        else if staff_answer == "academic"{
            if (1..=2).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}",staff_exp, academic[0]);
            }
            else if (3..=5).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, academic[1]);
            }
            else if (6..=8).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, academic[2]);
            }
            else if (9..=10).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, academic[3]);
            }
            else if (11..=13).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, academic[4]);
            }
            else if staff_exp > 13{
                println!("Congratulations, with your {} years of experience, you qualify for the position of {}", staff_exp, academic[5]);
            }
        }
        else if staff_answer == "lawyer"{
            if (1..=2).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}",staff_exp, lawyer[0]);
            }
            else if (3..=5).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, lawyer[1]);
            }
            else if (6..=8).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, lawyer[2]);
            }
            else if (9..=10).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, lawyer[3]);
            }
            else if (11..=13).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, lawyer[4]);
            }
            else if staff_exp > 13{
                println!("Congratulations, with your {} years of experience, you qualify for the position of {}", staff_exp, lawyer[5]);
            }
        }
        else if staff_answer == "teacher"{
            if (1..=2).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}",staff_exp, teacher[0]);
            }
            else if (3..=5).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, teacher[1]);
            }
            else if (6..=8).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, teacher[2]);
            }
            else if (9..=10).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, teacher[3]);
            }
            else if (11..=13).contains(&staff_exp){
                println!("Congratulations, with your {} years of experience, your position is {}", staff_exp, teacher[4]);
            }
            else if staff_exp > 13{
                println!("Congratulations, with your {} years of experience, you qualify for the position of {}", staff_exp, teacher[5]);
            }
        }
    }
    else{
        println!("We do not have an open space for your profession. Please try another branch.");
    }

}
