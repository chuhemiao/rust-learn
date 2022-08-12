use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let rand_secret_number = rand::thread_rng().gen_range(1..=1000); //trait

    println!("The secret number is: {rand_secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //non number, string utf-8, :: associated function

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // error notice

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        println!("Please input your guess");

        match guess.cmp(&rand_secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
