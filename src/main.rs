use std::io;

fn main() {
    let x = rand::random_range(0..10);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Something went wrong");

    let guess: i32 = guess
        .trim()
        .parse()
        .expect("Please enter a number");

    if guess == x {
        println!("Correct!");
    } else {
        println!("Wrong!");
    }
}