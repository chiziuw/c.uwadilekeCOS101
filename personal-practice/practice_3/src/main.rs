use std::io;

fn main() {
     loop {          
    println!("\nWelcome to your Compound Interest Savings Calculator!");
    println!("\nPlease select 'y' to add a new user and 'n' to stop the calculator");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
    if input == "n" {
     break
    } else if input == "y" {
          println!("\nLet's get started!");
     } else {
          println!("\nYou must select 'y' or 'n'");
     }

    println!("\nPlease enter your deposit amount:");
    let mut p = String::new();
    io::stdin().read_line(&mut p).expect("Failed to read input");
    let p:f64 = p.trim().parse().expect("Not a valid value");

    println!("\nPlease enter the interest rate:");
    let mut r = String::new();
    io::stdin().read_line(&mut r).expect("Failed to read input");
    let r:f64 = r.trim().parse().expect("Not a valid score");

    println!("\nPlease enter the duration period:");
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read input");
    let t:f64 = t.trim().parse().expect("Not a valid score");

    let a:f64 = p * (1.0 + (r / 100.0)).powf(t);
    let ci:f64 = a - p;

println!("\nYour amount is {:.2} and your compound interest is {:.2}",a, ci);
     }
}