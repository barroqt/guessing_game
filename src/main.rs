use std::io;
use rand::Rng;
use std::cmp::Ordering;

// Step 1 : Read the input
// Step 2 : Add RNG for mystery number
// Step 3 : Compare input with mystery number
// Step 4 : Convert input from string to integer
// Step 5 : Loop the program and break when win
// Step 6 : Ignore non numbers
// Step 7 : Remove mystery number from term

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess) // Type io::Result => Ok or Err
        .expect("Failed to read line"); // takes the return value from "Ok" and returns it (number of bytes in what the user entered into standard input.)

        let guess: u32 = guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
