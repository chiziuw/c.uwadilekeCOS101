fn main() {
    let p: f64 = 520000000.0; // define principal value

    let r: f64 = 10.0; // define rate value

    let t: f64 = 5.0; // define time value


    let a = p * (1.0 + (r / 100.0)).powf(t); // state formula
    let ci = a - p; // define main formula now

    println!("The compound interest is: ₦{:.1}", ci); // final answer
}