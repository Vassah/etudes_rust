fn sum_of_threes_and_fives(upper_bound: u32) -> u32 {
	let mut acc = 0;
	for i in 1..upper_bound {
		if i % 3 == 0 || i % 5 == 0 {
			acc += i;
		}
	}
	let acc = acc;
	acc
}

fn main() {
	let answer = sum_of_threes_and_fives(1000);
	println!("Answer: {}", answer);
}