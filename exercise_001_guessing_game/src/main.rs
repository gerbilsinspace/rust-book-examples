use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
     
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess_count = 0;

    loop {
        println!("Guess a number between 1 and 100!");
        guess_count += 1;

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed the number {}. You guessed {} times.", guess, guess_count);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("To Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
