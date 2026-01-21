use std::io;

// Utility function to read a floating-point number from user
fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().parse::<f64>().expect("Please enter a valid number")
}

// --- Calculation Functions ---

fn trapezium_area() -> f64 {
    let height = read_input("Enter height:");
    let base1 = read_input("Enter base1:");
    let base2 = read_input("Enter base2:");
    height / 2.0 * (base1 + base2)
}

fn rhombus_area() -> f64 {
    let d1 = read_input("Enter diagonal 1:");
    let d2 = read_input("Enter diagonal 2:");
    0.5 * d1 * d2
}

fn parallelogram_area () -> f64 {
    let base = read_input("Enter base:");
    let altitude = read_input("Enter altitude:");
    base * altitude
}

fn cube_area() -> f64 {
    let side = read_input("Enter length of the side:");
    6.0 * side.powi(2)
}

fn cylinder_volume() -> f64 {
    let radius = read_input("Enter radius:");
    let height = read_input("Enter height:");
    std::f64::consts::PI * radius.powi(2) * height
}

// --- Main Program ---

fn main() {
    println!("Select the calculation you want to perform:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choice");

    let choice = choice.trim();

    let result = match choice {
        "1" => trapezium_area(),
        "2" => rhombus_area(),
        "3" => parallelogram_area(),
        "4" => cube_area(),
        "5" => cylinder_volume(),
        _ => {
            println!("Invalid selection.");
            return;
        }
    };

    println!("Result = {}", result);
}
