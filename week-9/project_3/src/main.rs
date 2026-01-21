fn main() {
   
    let commissioners = ["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Caliistus Ogbonna",
        "Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];


    let ministries = vec!["Internal Affairs","Justice","Defense","Power & Steel",
        "Petroleum"];

   
    let zones = ["South West","North East","South South","South West","South East"];

    println!("Merged EFCC Dataset:\n");
    println!("{:<5} / {:<30} / {:<20} | {}", 
             "S/N", "COMMISSIONER", "MINISTRY", "ZONE");
    println!("{}", "-".repeat(80));

    for index in 0..commissioners.len() {
        println!(
          "{:<5} / {:<30} / {:<20} / {}",
            index + 1,
            commissioners[index],
            ministries[index],
            zones[index]
);
}
}






