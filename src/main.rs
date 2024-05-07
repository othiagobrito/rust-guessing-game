use std::io;

fn main() {
    println!("Try to guess the number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("Your guess was: {guess}");
}
