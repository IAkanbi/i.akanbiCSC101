use std::io::Write;

fn main() {
    let s_name = vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh"];
    let m_number = vec!["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
    let dept = vec!["Accounting","Economics","Computer","Electrical","Mechanical"];
    let level = vec![300,100,200,200,100];

    let mut file = std::fs::File::create("student_data.txt").expect("create failed");
    writeln!(file, "{:<20} | {:<15} | {:<12} | {}", "Student Name", "Matric.Number", "Department", "Level")
        .expect("Failed to write header");

    for i in 0..s_name.len() {
        writeln!(file,"{:<20} | {:<15} | {:<12} | {}",s_name[i], m_number[i], dept[i], level[i]).expect("Failed to write data");
    }
    println!("Data written successfully.")
}
