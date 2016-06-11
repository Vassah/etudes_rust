extern crate rand;

use std::io;
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
           .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
            Err(_) => continue,
        };

        if secret_number == guess {
             println!("Correct! The number was {}", secret_number);
	     break;
        } else {
             println!("Nope. The number was {}, you guessed {}", 
                     secret_number,
                     guess);
        }
    }
}
