
fn main() {
    // Using tuples (a simple Rust compound data type)
    let applicants = [
        ("Alice", 5),
        ("Bolu", 8),
        ("Chinedu", 12),
        ("Zainab", 7),
    ];

    // Start by assuming the first applicant has the highest experience
    let mut highest = applicants[0];

    // Loop through applicants to find the true highest
    for applicant in applicants {
        if applicant.1 > highest.1 {
            highest = applicant;
        }
    }

    println!("The person with the highest experience is:");
    println!("Name: {}", highest.0);
    println!("Years of Experience: {}", highest.1);
}

Class Project II – EY Nigeria Developer Experience Filter

