extern crate humanize;

use humanize::bool::{ToBool, Humanize};

fn main() {
	// &str to bool
    println!("{}", "yes".to_bool().unwrap());

    // String to bool
    let nope: String = String::from("off");
    println!("Value of nope is: {}", nope.to_bool().unwrap());

    // Bool to &str
    let sure: bool = true;
    println!("Value of b is: {}", sure.humanize());
}