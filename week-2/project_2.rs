fn main() {
	let t = 450_000.00;
	let m = 1_500_000.00;
	let h = 750_000.00;
	let d = 2_850_000.00;
	let a = 250_000.00;

	let s = (2.0 * t) + m + (3.0 * h) + (3.0 * d) + a;
	println!("SUM = {}",s );
	let ave = s/10.0; 
	println!("Average = {}",ave );
}