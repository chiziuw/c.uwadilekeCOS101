fn main() {
    let principal: f64 = 520_000_000.0; // set principal value
    let rate: f64 = 10.0; // set rate value
    let time: f64 = 5.0; // set time value

    let amount = principal * (1.0 + (rate / 100.0)).powf(time); // state formula
    let compound_interest = amount - principal; // state main formula now lol

    println!("The compound interest is: ₦{:.2}", compound_interest); // final answer
}