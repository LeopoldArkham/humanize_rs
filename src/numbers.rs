#[allow(unused_imports)]
use super::humanize::Humanize;


trait HumanizeNumbers {
    fn ord(&self) -> &str;
}

impl HumanizeNumbers for u8 {
    fn ord(&self) -> &str {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
    }
}

impl HumanizeNumbers for i8 {
    fn ord(&self) -> &str {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
    }
}

impl HumanizeNumbers for u16 {
    fn ord(&self) -> &str {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
    }
}

impl HumanizeNumbers for i16 {
    fn ord(&self) -> &str {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
    }
}

impl HumanizeNumbers for u32 {
    fn ord(&self) -> &str {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
    }
}

impl HumanizeNumbers for i32 {
    fn ord(&self) -> &str {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
    }
}

#[test]
fn test_ordinals() {
	assert_eq!(101.ord(), "st");
	assert_eq!(2.ord(), "nd");
	assert_eq!(10093.ord(), "rd");
	assert_eq!((-159652).ord(), "th");
	assert_eq!(0.ord(), "th");
}