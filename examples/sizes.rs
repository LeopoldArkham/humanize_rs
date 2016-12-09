extern crate humanize;

use humanize::sizes::*;

fn main() {
    println!("{}", 1549863.file_size(file_size_opts::DECIMAL).unwrap());
}
