use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess. A Number between 1-100");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let int_guess: &mut u32 = &mut guess.trim().parse().unwrap();

    let mut rng = rand::thread_rng();

    let random_number: u32 = rng.gen_range(0..100);

    if random_number == *int_guess {
        println!("You did it! My number was also {random_number}");
    } else {
        println!("But my number was {random_number} you're a failure!");
    }

}
