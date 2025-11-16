use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Guess the number!");
    let mut rng = rand::rng();
    let secret_num = rng.random_range(0..=100);
    println!("The secret number is: {}", secret_num);

    loop {
        println!("=> Input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
