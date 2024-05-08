use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);

    println!("Try to guess the number between 1 and 100:");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("You should type a number!");

    println!("Your guess was: {guess}");

    match guess.cmp(&random_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Winner Winner Chicken Dinner"),
    }

    println!("The number was: {random_number}");
}
