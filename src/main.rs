mod greetings;
//use greetings::default_greeting;
//use greetings::default_greeting2;
//use greetings::*;

mod greetings;
use greetings::{default_greeting, default_greeting2, french, spanish};

fn main() {
    println!("Hello, world!");
    println!("{}", default_greeting());
    println!("{}", default_greeting2());
    println!(
        "My greeting is '{}' and the second is '{}'",
        default_greeting(),
        default_greeting2()
    );
}
