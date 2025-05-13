
fn main() {
    println!("Guess the number please!");
    println!("Please input your guess.");
    let mut guess = String::new();

    let generated = rand::rng();

    // stdin().read_line(&mut guess).expect("Failed to read guess.");

    
    println!({generated.random::<char>});
}

