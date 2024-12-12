use std::io;

fn main() {
    ask the number
    println!("Guess the number");
    println!("Pls input your best guess:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect( "Failed to read line");

    println!("You guessed {}", guess)
}
