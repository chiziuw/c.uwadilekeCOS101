use std::io::Write;

fn main() {
    let v1 = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let v2 = vec!["Legend", "Turbo King", "Williams"];
    let v3 = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    let menu = format!("\nLager = {:?}
        \nStout = {:?}
        \nNon-Alcoholic = {:?}", v1, v2, v3);


    let mut file = std::fs::File::create("Drink Categories.txt").expect("Create failed");
    file.write_all(menu.as_bytes()).expect("Write failed");
    println!("Data written to file.");
}