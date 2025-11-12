use std::io;
// import the constant value for PI
use std::f64::consts::PI;

fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let value: f64 = input.trim().parse().expect("Invalid input. Please enter a number.");
    return value;
}

// --- Calculation Functions ---
fn area_trapezium() {
    println!("\n--- Area of Trapezium ---");
    let height = get_input("Enter height:");
    let base1 = get_input("Enter base 1:");
    let base2 = get_input("Enter base 2:");
    
    let area = (height / 2.0) * (base1 + base2);
    println!("Result: The area of the trapezium is: {:.2}", area);
}

fn area_rhombus() {
    println!("\n--- Area of Rhombus ---");
    let d1 = get_input("Enter diagonal 1:");
    let d2 = get_input("Enter diagonal 2:");
    
    let area = 0.5 * d1 * d2;
    println!("Result: The area of the rhombus is: {:.2}", area);
}

fn area_parallelogram() {
    println!("\n--- Area of Parallelogram ---");
    let base = get_input("Enter base:");
    let altitude = get_input("Enter altitude:");
    
    let area = base * altitude;
    println!("Result: The area of the parallelogram is: {:.2}", area);
}

fn area_cube() {
    println!("\n--- Area of Cube ---");
    let length = get_input("Enter the length of one side:");
    
    // side^2 is just side * side
    let area = 6.0 * length * length;
    println!("Result: The surface area of the cube is: {:.2}", area);
}

fn volume_cylinder() {
    println!("\n--- Volume of Cylinder ---");
    let radius = get_input("Enter radius:");
    let height = get_input("Enter height:");

    let volume = PI * radius * radius * height;
    println!("Result: The volume of the cylinder is: {:.2}", volume);
}


// --- Main Function (The Menu) ---

fn main() {
    loop {
        println!("\n========= MTH 101 Calculator =========");
        println!("Select a calculation:");
        println!("  1. Area of Trapezium");
        println!("  2. Area of Rhombus");
        println!("  3. Area of Parallelogram");
        println!("  4. Area of Cube");
        println!("  5. Volume of Cylinder");
        println!("  0. Exit");
        println!("========================================");
        print!("Enter your choice (0-5): ");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number between 0 and 5.");
                continue;
            }
        };

        if choice == 1 {
            area_trapezium();
        } else if choice == 2 {
            area_rhombus();
        } else if choice == 3 {
            area_parallelogram();
        } else if choice == 4 {
            area_cube();
        } else if choice == 5 {
            volume_cylinder();
        } else if choice == 0 {
            println!("Goodbye!");
            break;
        } else {
            println!("Invalid choice. Please select from 0-5.");
        }
    }
}