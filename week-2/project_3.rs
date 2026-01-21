fn main() {
	let p:f64 = 510_000.0;
	let r:f64 = 5.0;
    let t:f64 = 3.0;

    let a = p * (1.0 - (r/100.0)) * t;
    println!("Ammount = {}", a );
    let c = a - p;
    println!("CI = {}", c );
}