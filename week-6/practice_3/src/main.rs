fn main() {
   let name1 = "Ayomide Adesokan";
    println!("My name is{}", name1);


    let name2 = name1.replace("Ayomide", "Adebare");
    println!("You cn also call me {}",name2 );
    let facility = "Faculty of Science of technology";

    let school = facility.replace("Faculty", "school");
    println!("I am a student of the {}", school);
}
