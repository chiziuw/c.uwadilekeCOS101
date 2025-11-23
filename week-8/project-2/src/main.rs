use std::io;

// COMPOUND DATA TYPE 1: Struct
// A struct allows us to group different data types (String and u32)
// into a single custom type representing a "Candidate".
struct Candidate {
    name: String,
    experience: u32,
}

fn main() {
    println!("--- EY Nigeria Recruitment System ---");

    // COMPOUND DATA TYPE 2: Vector
    // A Vector (Vec) is a dynamic array that can grow in size.
    // We store our 'Candidate' structs inside this vector.
    let mut candidates: Vec<Candidate> = Vec::new();

    // 1. Input Loop
    loop {
        println!("\nEnter candidate name (or type 'done' to finish):");
        let mut name_input = String::new();
        io::stdin().read_line(&mut name_input).expect("Failed to read line");
        let name = name_input.trim().to_string();

        // Check exit condition
        if name.to_lowercase() == "done" {
            break;
        }

        println!("Enter years of experience for {}:", name);
        let mut exp_input = String::new();
        io::stdin().read_line(&mut exp_input).expect("Failed to read line");
        
        // Validate number input
        let experience: u32 = match exp_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number! Please re-enter this candidate.");
                continue;
            }
        };

        // Create a new instance of our struct
        let new_candidate = Candidate {
            name,
            experience,
        };

        // Push it into the vector
        candidates.push(new_candidate);
    }

    // 2. Logic to find the highest experience
    if candidates.len() > 0 {
        // We assume the first person is the highest to start
        let mut highest_candidate = &candidates[0];

        // Iterate through the vector to find the max
        for person in &candidates {
            if person.experience > highest_candidate.experience {
                highest_candidate = person;
            }
        }

        println!("\n=========================================");
        println!("HIRING DECISION:");
        println!("The candidate with the most experience is: {}", highest_candidate.name);
        println!("Years of Experience: {}", highest_candidate.experience);
        println!("=========================================");
    } else {
        println!("\nNo candidates were entered.");
    }
}