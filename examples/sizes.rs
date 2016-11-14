extern crate humanize;
use humanize::file_sizes::*;

fn main() {
	println!("{}", file_size(5456, file_size_opts::DECIMAL));
	println!("{}", file_size(5456, file_size_opts::BINARY));
	println!("{}", file_size(1024, file_size_opts::BINARY));
	println!("{}", file_size(1023_654_123_654, file_size_opts::BINARY));
	println!("{}", file_size(1023_654_123_654, file_size_opts::DECIMAL));
}