use std::io::Read;
use std::io;

fn main() {

    loop{
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();
        println!("Welcome to the Globacom Ltd Database Management App.");
        println!("This program will show you the structure of a table/databse depending on your level of access.");

        println!("Determining Access Level.");
        println!("-----------------------------------------");
        println!("Please select your position from the list below:");
        println!("1: Administrator\n2: Project Manager\n3: Employee\n4: Customer\n5: Vendor");

        io::stdin().read_line(&mut input1).expect("Failed to read input.");
        let user_access: u32 = input1.trim().parse().expect("Failed to input");

        match user_access {
            1 => {
                println!("Database Administrator Access Confirmed. Kindly find the database structure below:");
                administrator();
            }

            2 => {
                println!("Project Manager Access Confirmed. Kindly find below the data structure of the Project Table");
                project();
            }

            3 => {
                println!("Employee Access Confirmed. Kindly find below the data structure of the Staff Table");
                staff();
            }

            4 => {
                println!("Customer Access Confirmed. Kindly find below the data structure of the Customer Table");
                customer();
            }

            5 => {
                println!("Vendor Access Confirmed. Kindly find below the data stucture of the Data Plan Table");
                dataplan();
            }

            _ => {
                println!("Invalid selection.");
                continue;
            }

        };
        println!("Would you like to check another data structure? (y/n)");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let choice = input2.trim().to_lowercase();

        if choice != "y" {
            println!("Goodbye!");
            break;
        }
    }

}
    

fn administrator() {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn project() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn staff() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn customer() {
    let mut file = std::fs::File::open("customer_table_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn dataplan() {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}