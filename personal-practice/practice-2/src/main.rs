use std::io;

fn main() {
    println!("\nPlease enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("\nPlease enter test score 1:");
    let mut score1 = String::new();
    io::stdin().read_line(&mut score1).expect("Failed to read input");
    let score1:f32 = score1.trim().parse().expect("Not a valid score");

    println!("\nPlease enter test score 2:");
    let mut score2 = String::new();
    io::stdin().read_line(&mut score2).expect("Failed to read input");
    let score2:f32 = score2.trim().parse().expect("Not a valid score");

    println!("\nPlease enter test score 3:");
    let mut score3 = String::new();
    io::stdin().read_line(&mut score3).expect("Failed to read input");
    let score3:f32 = score3.trim().parse().expect("Not a valid score");

    let avg = (score1 + score2 + score3) / 3.0;
let grade:char;

        if avg >= 70.00 && avg <= 100.00 {
             grade =  'A';               
        }
        else if avg >= 60.00 && avg < 70.00 {
             grade =  'B';
        }
        else if avg >= 50.00 && avg < 60.00 {
             grade =  'C';
        }
        else if avg >= 45.00 && avg < 50.00 {
             grade =  'D';
        }
        else if avg >= 0.00 && avg < 45.00 {
             grade =  'F';
        }

        else {
            println!("Your grade is an anomaly and doesn't exist.");
            return;
        }
    
    println!("\n{} your grade is {}", name, grade);
}