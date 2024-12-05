use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(0..10);

    println!("Guess the number between 0 and 10!");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("You guessed correct");
                break;
            }
            Ordering::Greater => println!("You guessed higher"),
            Ordering::Less => println!("You guessed lower"),
        }
    }
}
