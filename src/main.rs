use rand::Rng;
use std::cmp::Ordering;
use std::io; //Standard library (input/output)

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) //this returns an io::Result (enums(Ok or Err))
            .expect("Failed to read line!"); // Error handling if io::Result == Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
