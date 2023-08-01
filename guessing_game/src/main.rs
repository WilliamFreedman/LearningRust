use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let to_guess = rand::thread_rng().gen_range(1..=100);

    println!("Test display {}", to_guess);

    println!("Guess the number!");

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() 
            { //trims and parses guess, matches with result enum to assign
                Ok(num) => num,
                Err(_) => {
                        println!("Invalid guess entered: {}Try again.",guess);
                        continue;
                        },
            };

        match guess.cmp(&to_guess) { //.cmp returns variants of the Ordering enum, the match then compares them
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => 
                {
                    println!("You win!");
                    break;
                }
        }

        

    }

    
}
