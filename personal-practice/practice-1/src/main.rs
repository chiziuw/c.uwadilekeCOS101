use std::io;

fn main() {
    println!("\nAre you experienced? yes or no");
    let mut exp = String::new();
    io::stdin().read_line(&mut exp).expect("Failed to read input");
    let exp = exp.trim();
    let experienced = exp == "yes";

    println!("\nHow old are you?");
    let mut age = String::new();    
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let age:u32 = age.trim().parse().expect("Not a valid age");

    if experienced {
        if age >= 40 {
        println!("Your incentive is 1,560,000");
    }
        else if age < 40 && age >= 30 {
        println!("Your incentive is 1,480,000");
    }    
        else if age < 30 && age >= 20 {
        println!("Your incentive is 1,300,00");
        }
}
    else {
        println!("Your incentive is 100,000");
    }
}