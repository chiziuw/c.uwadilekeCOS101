use std::io;

fn main() {
    

    println!("===FOOD MENU===");
    println!("P = Poundo yam and Edikaiko soup = 3200");
    println!("F = Fried rice & chicken = 3000");
    println!("A = Amala and Ewedu soup = 2000");
    println!("E = Eba and Egusi soup = 2000");
    println!("W = White rice & stew = 2500");


//     let food_input = loop {
//     let mut input1 = String::new();
//     println!("Please enter food code");
//     io::stdin().read_line(&mut input1).expect("not a valid input");
//     let input1 = input1.trim().to_uppercase();   
//     match input1.trim().parse::<char>(){
//         Ok(char) => break char,
//         Err(_) => println!("please enter in the food CODE"),
//     }
// };



  let food_input = loop {
        let mut input = String::new();
        println!("Please enter food code:");
        io::stdin().read_line(&mut input).expect("Invalid input");

        match input.trim().to_uppercase().chars().next() {
            Some('P' | 'F' | 'A' | 'E' | 'W') => break input.chars().next().unwrap(),
            _ => {println!("Please enter a valid food code");
            continue; }
        }
    };

    if food_input == 'P' {
        println!("order for Poundo yam and Edikaiko soup = 3200 received");
    }
    else if food_input == 'f'{
        println!("order for Fried rice & chicken = 3000 received");
    }
    else if food_input == 'A'  {
        println!("order for Amala and Ewedu soup = 2000 received");
    }
    else if food_input == 'E' {
        println!("order for Eba and Egusi soup = 2000 received");
    }
    else if food_input == 'W' {
         println!("order for White rice & stew = 2500 received");
    }
    else {
        println!("NOT A VALID  FOOD CODE");
}
}
 


