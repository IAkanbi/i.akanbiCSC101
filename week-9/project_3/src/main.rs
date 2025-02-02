use std::io::Write;

fn main() {
    let id = vec![1,2,3,4,5];
    let name = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    // Why is my surname here? 
    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let geo = vec!["South West","North East","South South","South West","South East"];

    let mut file = std::fs::File::create("EFCC_data.txt").expect("create failed");
    writeln!(file,"{} | {:<30} | {:<20} | {:<18}","S/N","Name of Commissioner","Ministry","Geopolitical Zone").expect("Failed to write header");
    writeln!(file,"------------------------------------------------------------------------------------------------------------------").expect("failed to write");

    for i in 0..id.len() {
        writeln!(file,"{}   | {:<30} | {:<20} | {:<18}",id[i],name[i],ministry[i],geo[i]).expect("Failed to write data");
    }
    println!("Data written to file.");
}
