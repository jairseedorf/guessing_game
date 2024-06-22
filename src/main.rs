use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Welcome to Guesser 🕹️");
    println!("Guess a number between 1 to 999");

    let secret_number = rand::thread_rng().gen_range(1..999);

    println!("Secret Number: {}", secret_number);

    loop {
        println!("{}", "What's your guess?".yellow());
        
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Ooops, we broke the game");
    
        let guess: u32 = match guess.trim().parse() {
           Ok(num) => num,
           Err(_) => continue
        };
    
    println!("Your guess: {}", guess);
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}", "Too small 😩".red()),
        Ordering::Greater => println!("{}", "Too big 😹".red()),
        Ordering::Equal => {
            println!("{}", "You win 💃🎉".green());
            break;
        }
    }
   }
}
