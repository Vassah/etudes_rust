fn gradient_descent<F>(gradient: F, iterations: u64, step_size: f64) -> [f64; 2]
	where F : Fn([f64;2]) -> [f64; 2] {
		let mut guess: [f64;2] = [1., -1.];
		let mut derivative = [0., 0.];
		for i in 1..iterations {
			derivative = gradient(guess);
			guess = [guess[0] - step_size*derivative[0], guess[1] - step_size*derivative[1]];
		}
		return guess;
}

fn main() {
	//let func = |x: [f64; 2]| x[0].sin() - x[0].cos()*x[1].cos();
	//let gradient = |x: [f64; 2]| [-x[0].cos() + x[0].sin()*x[1].cos(), x[0].cos() * x[1].sin()];
	let gradient = |x: [f64; 2]| [-x[0].sin(), x[1].cos()];
	let maximum_at = gradient_descent(gradient, 10000, 0.5);
	println!("[{0},{1}]", maximum_at[0], maximum_at[1]);
}