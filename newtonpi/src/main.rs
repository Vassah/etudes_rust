use std::io::BufRead;

fn even(n: u64) -> bool {
	n % 2 == 0
}
// Obviously if we actually gave a shit about performance
// then newton_pi and leibniz_pi would be refactored to be
// internal to a single function computing both at the same
// time. But this implementation isn't even numerical stable
// at a certain point so it's not really worth quibbling i
// suppose.

fn newton_pi(iterations: u64) -> f64 {
	let mut x_n: f64 = 3.;
	let mut error_term: f64;
	for i in 1..iterations {
		error_term = x_n.sin()/x_n.cos();
		x_n -= error_term;
	}
	x_n
}

fn leibniz_pi(iterations: u64) -> f64 {
	let mut x_n: f64 = 0.;
	let mut delta: f64;
	for i in 0..iterations {
		delta = 1. / ((2*i + 1) as f64);
		if even(i) {
			x_n += delta;
		} else {
			x_n -= delta;
		} 
	}
	4.*x_n
}

fn main() {
	println!("How many iterations?");
	let stdin = std::io::stdin();
	let iterations = stdin.lock()
						  .lines()
						  .next()
						  .unwrap()
						  .unwrap()
						  .parse::<u64>()
						  .unwrap();
	let pi_newt = newton_pi(iterations);
	let pi_leibniz = leibniz_pi(iterations);
	let difference = pi_newt - pi_leibniz;
	println!("Newton on {0}: {1}", iterations, pi_newt);
	println!("Leibniz on {0}: {1}", iterations, pi_leibniz);
	println!("Difference: {0}", difference);
}