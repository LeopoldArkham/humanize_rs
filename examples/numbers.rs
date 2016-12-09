extern crate humanize;
use humanize::HumanizeNumbers;

fn main() {
    let scoreboard = ["Bob", "Victor", "Richard", "John", "Lisa"];

    for (i, name) in scoreboard.iter().enumerate() {
        println!("{}: {}", (i + 1).ord(), name);
    }

    let num: i64 = -6;
    println!("{}: {}", num, num.to_text());

    println!("{}", 0.intcomma());
}