use std::io::Write;

fn main() {
    let names_of_comissioner = vec!["Aigbogun Alamba Dauda", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs", "   Justice", "Defense", "  Power & Steel", "   Petroleum"];
    let geopolitical_zone = vec!["South West", "        North East", "      South South", "   South West", "    South East"];



    let mut file = std::fs::File::create("Ministers.txt").expect("Create failed");

    let b = format!("S/N  |  Name of Commisioner  |   Ministry        |  Geopolitical Zone");
    file.write_all(b.as_bytes()).expect("write failed.");
    let mut count = 1;
    for i in 0..names_of_comissioner.len()
    {
        let a = format!("\n{}    {}      {}      {}",count, names_of_comissioner[i], ministry[i], geopolitical_zone[i]);
        count +=1;
        file.write_all(a.as_bytes()).expect("write failed.");
}
    print!("Data written to file.");
    } 


    

