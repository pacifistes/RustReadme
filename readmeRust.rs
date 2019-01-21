rustup update stable // update the rust version

cargo new filename

cargo build		// Compile
cargo check		// Compile but dont get executalbe (more faster than build)
cargo run		// Compile + Run
cargo update	// update the version of all the crates (3.14 -> 3.16); If need go 4.0 change in Cargo.toml directly

*******************************************************************

use std::io;
io::stdin().read_line(&mut guess).expect("Failed to read line"); // expect handle the Faillure if io::Result is Err

*******************************************************************

use rand::Rng;
rand = "0.3.14" // Add this crates below dependencies in Cargo.toml
let secret_number = rand::thread_rng().gen_range(1, 101);

********************************************************************

use std::cmp::Ordering;
let mut guess = String::new();

io::stdin().read_line(&mut guess)
	.expect("Failed to read line");

let guess: u32 = guess.trim().parse()
	.expect("Please type a number!");

println!("You guessed: {}", guess);

match guess.cmp(&secret_number) {
	Ordering::Less => println!("Too small!"),
	Ordering::Greater => println!("Too big!"),
	Ordering::Equal => println!("You win!"),
}

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
*******************************************************************
USE VARIABLE WITH SAME NAME AS default keyword
let r#fn
r#match();


