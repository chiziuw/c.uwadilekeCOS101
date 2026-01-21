use std::io;

// Data structure for one APS category
#[derive(Debug)]
struct ApsLevel {
    name: &'static str,
    office_admin: &'static str,
    academic: &'static str,
    lawyer: &'static str,
    teacher: &'static str,
}

fn main() {
    // Vector holding all APS classification rows
    let aps_levels = vec![
        ApsLevel {
            name: "APS 1-2",
            office_admin: "Intern",
            academic: "-",
            lawyer: "Paralegal",
            teacher: "Placement",
        },
        ApsLevel {
            name: "APS 3-5",
            office_admin: "Administrator",
            academic: "Research Assistant",
            lawyer: "Junior Associate",
            teacher: "Classroom Teacher",
        },
        ApsLevel {
            name: "APS 5-8",
            office_admin: "Senior Administrator",
            academic: "PhD Candidate",
            lawyer: "Associate",
            teacher: "Snr Teacher",
        },
        ApsLevel {
            name: "EL1 8-10",
            office_admin: "Office Manager",
            academic: "Post-Doc Researcher",
            lawyer: "Senior Associate 1-2",
            teacher: "Leading Teacher",
        },
        ApsLevel {
            name: "EL2 10-13",
            office_admin: "Director",
            academic: "Senior Lecturer",
            lawyer: "Senior Associate 3-4",
            teacher: "Deputy Principal",
        },
        ApsLevel {
            name: "SES",
            office_admin: "CEO",
            academic: "Dean",
            lawyer: "Partner",
            teacher: "Principal",
        },
    ];

    println!("=== APS LEVEL CHECKER ===");

    // Input: Profession
    println!("Enter profession (office_admin, academic, lawyer, teacher):");
    let mut profession = String::new();
    io::stdin().read_line(&mut profession).unwrap();
    let profession = profession.trim().to_lowercase();

    // Input: Role Title
    println!("Enter staff role/title:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim().to_lowercase();

    // Search for matching APS level
    let mut result: Option<&str> = None;

    for level in &aps_levels {
        let match_title = match profession.as_str() {
            "office_admin" => level.office_admin,
            "academic" => level.academic,
            "lawyer" => level.lawyer,
            "teacher" => level.teacher,
            _ => {
                println!("Invalid profession entered.");
                return;
            }
        }
        .to_lowercase();

        if match_title == title {
            result = Some(level.name);
            break;
        }
    }

    match result {
        Some(level) => println!("\nThe staff member belongs to: {level}"),
        None => println!("\nNo matching APS level found for the provided information."),
    }
}
