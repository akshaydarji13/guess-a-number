use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, User!");
    
    let secret_number = rand::thread_rng().gen_range(1,101);

    let mut count = 0;

    loop{
        println!("Please input your guess.");
    
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                count = count + 1;
                println!("Too small!");
            },
            Ordering::Greater => {
                count = count + 1;
                println!("Too big!");
            },
            Ordering::Equal => {
                count = count + 1;
                println!("You win!");
                println!("Number of Guesses: {}", count);
                break;
            }
        }
    }
}
