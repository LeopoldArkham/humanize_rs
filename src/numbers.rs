extern crate num_traits;

#[allow(unused_imports)]
use super::humanize::Humanize;
use self::num_traits::PrimInt;


trait HumanizeNumbers {
    fn ord(&self) -> &str;
}

impl<I> HumanizeNumbers for I
{
    fn ord(&self) -> &str {
    	let r = self % 10;
        match r {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        }
    }
}

#[test]
fn test_ordinals() {
	assert_eq!(5.ord(), "test");
}