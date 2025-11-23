use std::io;

// Define an Enum for the profession types to ensure type safety
enum Profession {
    OfficeAdministrator,
    Academic,
    Lawyer,
    Teacher,
    Unknown,
}

fn main() {
    println!("--- Federal Government of Nigeria APS Level Checker ---");

    // 1. Get the Profession
    println!("\nSelect your profession:");
    println!("1. Office Administrator");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");
    
    let mut input_prof = String::new();
    io::stdin().read_line(&mut input_prof).expect("Failed to read line");
    
    let profession = match input_prof.trim() {
        "1" => Profession::OfficeAdministrator,
        "2" => Profession::Academic,
        "3" => Profession::Lawyer,
        "4" => Profession::Teacher,
        _ => Profession::Unknown,
    };

    // Check if valid profession before proceeding
    if let Profession::Unknown = profession {
        println!("Invalid profession selected. Exiting.");
        return;
    }

    // 2. Get Years of Experience
    println!("\nEnter your years of experience (e.g., 4):");
    let mut input_years = String::new();
    io::stdin().read_line(&mut input_years).expect("Failed to read line");
    
    // Parse the string input into a number (u32)
    let years: u32 = match input_years.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    // 3. Determine Level and Title based on the table logic
    // We pass a reference (&) to profession to avoid moving ownership
    let result = check_level(&profession, years);

    println!("\n------------------------------------");
    println!("Result: {}", result);
    println!("------------------------------------");
}

fn check_level(prof: &Profession, years: u32) -> String {
    // The outer match handles the years (Ranges inferred from the table)
    match years {
        1..=2 => {
            let level = "APS 1-2";
            let title = match prof {
                Profession::OfficeAdministrator => "Intern",
                Profession::Academic => "N/A (No position available)", // The "-" in the table
                Profession::Lawyer => "Paralegal",
                Profession::Teacher => "Placement",
                _ => "Unknown",
            };
            format!("Level: {} | Position: {}", level, title)
        },
        3..=5 => {
            let level = "APS 3-5";
            let title = match prof {
                Profession::OfficeAdministrator => "Administrator",
                Profession::Academic => "Research Assistant",
                Profession::Lawyer => "Junior Associate",
                Profession::Teacher => "Classroom Teacher",
                _ => "Unknown",
            };
            format!("Level: {} | Position: {}", level, title)
        },
        6..=8 => { // Note: Table says 5-8, but 3-5 covers 5. I adjusted logic to 6-8 for no overlap.
            let level = "APS 5-8";
            let title = match prof {
                Profession::OfficeAdministrator => "Senior Administrator",
                Profession::Academic => "PhD Candidate",
                Profession::Lawyer => "Associate",
                Profession::Teacher => "Snr Teacher",
                _ => "Unknown",
            };
            format!("Level: {} | Position: {}", level, title)
        },
        9..=10 => {
             let level = "EL1 8-10";
             let title = match prof {
                Profession::OfficeAdministrator => "Office Manager",
                Profession::Academic => "Post-Doc Researcher",
                Profession::Lawyer => "Senior Associate 1-2",
                Profession::Teacher => "Leading Teacher",
                _ => "Unknown",
            };
            format!("Level: {} | Position: {}", level, title)
        },
        11..=13 => {
             let level = "EL2 10-13";
             let title = match prof {
                Profession::OfficeAdministrator => "Director",
                Profession::Academic => "Senior Lecturer",
                Profession::Lawyer => "Senior Associate 3-4",
                Profession::Teacher => "Deputy Principal",
                _ => "Unknown",
            };
            format!("Level: {} | Position: {}", level, title)
        },
        _ if years > 13 => {
             let level = "SES";
             let title = match prof {
                Profession::OfficeAdministrator => "CEO",
                Profession::Academic => "Dean",
                Profession::Lawyer => "Partner",
                Profession::Teacher => "Principal",
                _ => "Unknown",
            };
            format!("Level: {} | Position: {}", level, title)
        },
        _ => String::from("Experience too low for classification."),
    }
}