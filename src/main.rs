use std::io::stdin;


fn main() {
    println!("Guess the number please!");
    println!("Please input your guess.");
    let mut guess = String::new();


    stdin()
    .read_line(&mut guess)
    .expect("Failed to read guess.");

    
    println!("You Guesses {guess}");

    // This is only an added comment to check Git reactions
}

