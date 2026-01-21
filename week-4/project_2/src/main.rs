use std::io;

fn main() {

    let mut age_input = String::new();
    println!("Enter employee age");
    io::stdin().read_line(&mut age_input).expect("not a valid string");
    let age:f64 = age_input.trim().parse().expect("not a valid number");
    

   let experience_input = loop{
    let mut experience_input = String::new();
    println!("Is the employee experienced? (y/n)");
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let experience_input = experience_input.trim().to_lowercase();
    match experience_input.trim().parse::<char>(){
       Ok(char) => break char,
        Err(_) => println!("please enter 'y' or 'n'"),
    }
};

    if experience_input == 'y' && age >= 40.0 {
            println!("incentive = 1_560_000");
        }else if experience_input == 'y' && age >= 30.0 && age < 40.0{
            println!("incentive = 1_480_000");
        }else if experience_input == 'y' && age < 28.0 {
            println!("incentive = 1_300_000");
        }else if  experience_input ==  'y' && age >= 28.0 && age < 30.0 {
          println!("incentive = 1_300_000");
    }else if experience_input == 'n'{
         println!("incentive = 100_000");
}
}
