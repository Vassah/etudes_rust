fn next_fib(previous: (u64, u64)) -> (u64, u64) {
	(previous.1, previous.0 + previous.1)
}

fn even_fib_sum(upper_bound:u64)->u64 {
	let mut previous = (1,2);
	let mut acc = 2;
	loop {
		let next = next_fib(previous);
		let b = next.1;
	    if b > upper_bound {
	    	return acc;
	    } else if b % 2 == 0{
	    	acc += b;
	    }
	    previous = next;
	}
}

fn main() {
	let answer = even_fib_sum(4000000);
	println!("Answer: {}", answer);
}