use std::cmp::Ordering;
use std::io;
//use std::process;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!!!");
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is {secret}");

    loop {
        println!("Guess a number : ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid input!!!\n");
                continue;
            }
        };
    
        match num.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
             },
        }
    }
}
