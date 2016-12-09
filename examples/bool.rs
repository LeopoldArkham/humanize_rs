extern crate humanize;

use humanize::ToBool;

fn main() {
    // &str to bool
    println!("{}", "yes".to_bool().unwrap());

    // String to bool
    let nope: String = String::from("off");
    println!("Value of nope is: {}", nope.to_bool().unwrap());
}
