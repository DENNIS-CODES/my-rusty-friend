use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        println!("please input your guess.π€");

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read lineβ");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!π"),
            Ordering::Greater => println!("too big!π"),
            Ordering::Equal => {
                println!("you win!β");
                break;
            }
        }
    }
}
