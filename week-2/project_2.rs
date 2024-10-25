fn main(){
	let t:f64 = 2.0 * 450_000.00;
	let m:f64 = 1.0 * 1_500_000.00;
	let hp:f64 = 3.0 * 750_000.00;
	let d:f64 = 3.0 * 2_850_000.00;
	let a:f64 = 1.0 * 250_000.00;

	let sum = t + m + hp + d + a;
	println!("The sum of the sales record is {} ", sum);
	let average = sum/10.0;
	println!("The average of the sales record is {} ", average);
}
