use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let target = rand::thread_rng().gen_range(1..=100); // default is signed 32 bit number

    //println!("Guessing target is: {target}");

    loop {
    let mut guess = String::new(); // this needs to be inside the loop for multiple guesses
    println!("Guess a number:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        //let guess: u32 = guess.trim().parse().expect("Numbers only!"); // smth called shadowing going on here

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&target) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
