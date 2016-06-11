use std::f64;

fn sieve_of_eratosthenes(test: u64) -> &Vec<u64> {
	let test = test as f64;
	let upper_bound = test.sqrt().floor() as u64;
	let mut numbers: Vec<u64> = (2..upper_bound).collect();
	let mut p_test = vec![1; upper_bound as usize];
	for p in &numbers {
		let p = *p;
		let mut q = 2;
		while q*p < upper_bound {
			let index = (q*p - 2) as usize;
			p_test[index] = 0;
			q += 1;
		}
	}
	let mut primes = vec![];
	for p in numbers {
		let index = (p - 2) as usize;
		if p_test[index] == 1 {
			primes.push(p);
		}
	}
	&primes
}

fn largest_prime_of(test: u64) -> u64 {
	let mut primes = sieve_of_eratosthenes(test);
	primes.reverse();
	for prime in *primes {
		let prime = prime;
		if test % prime == 0 {
			return prime;
		}
	}
	return 0;
}

fn main() {
	let answer = largest_prime_of(600851475143);
	println!("{}", answer);
}