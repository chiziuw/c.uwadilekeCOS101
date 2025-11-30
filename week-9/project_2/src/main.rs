use std::io::Write;

fn main() {
    let v1 = vec![("Oluchi Mordi    |", "ACC10211111 |", "Accounting  |", 300  )];
    let v2 = vec![("Adams Aliyu     |", "ECO10110101 |", "Economics   |", 100)];
    let v3 = vec![("Shania Bolade   |", "CSC10328828 |", "Computer    |", 200)];
    let v4 = vec![("Adekunle Gold   |", "EEE11020202 |", "Electrical  |", 200)];
    let v5 = vec![("Blanca Edemoh   |", "MEE10202001 |", "Mechanical  |", 100)];

    let table = format!("               PAU SMIS \nStudent Name       |   Matric. Number |   Department  |   Level
    \n{:?} \n{:?} \n{:?} \n{:?} \n{:?} ", v1,v2,v3,v4,v5);

    let mut file = std::fs::File::create("students.txt").expect("Create failed");
    file.write_all(table.as_bytes()).expect("write failed");
    println!("Data written to file.");
}
