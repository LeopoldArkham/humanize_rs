use super::humanize::Humanize;
use std::fmt::Write;

pub trait HumanizeNumbers {
    fn ord(&self) -> String;
    fn to_text(&self) -> String;
}

impl HumanizeNumbers for u8 {
    fn ord(&self) -> String {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
        let mut res = String::new();
        let _ = write!(res, "{}{}", self, suffix);
        res
    }

    fn to_text(&self) -> String {
    	
    }
}

impl HumanizeNumbers for i8 {
    fn ord(&self) -> String {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
        let mut res = String::new();
        let _ = write!(res, "{}{}", self, suffix);
        res
    }
}

impl HumanizeNumbers for u16 {
    fn ord(&self) -> String {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
        let mut res = String::new();
        let _ = write!(res, "{}{}", self, suffix);
        res
    }
}

impl HumanizeNumbers for i16 {
    fn ord(&self) -> String {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
        let mut res = String::new();
        let _ = write!(res, "{}{}", self, suffix);
        res
    }
}

impl HumanizeNumbers for u32 {
    fn ord(&self) -> String {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
        let mut res = String::new();
        let _ = write!(res, "{}{}", self, suffix);
        res
    }
}

impl HumanizeNumbers for i32 {
    fn ord(&self) -> String {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
        let mut res = String::new();
        let _ = write!(res, "{}{}", self, suffix);
        res
    }
}

impl HumanizeNumbers for usize {
    fn ord(&self) -> String {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
        let mut res = String::new();
        let _ = write!(res, "{}{}", self, suffix);
        res
    }
}

impl HumanizeNumbers for isize {
    fn ord(&self) -> String {
        let suffix = match self % 10 {
        	1 => "st",
        	2 => "nd",
        	3 => "rd",
        	_ => "th"
        };
        let mut res = String::new();
        let _ = write!(res, "{}{}", self, suffix);
        res
    }
}

#[test]
fn test_ordinals() {
	assert_eq!(101.ord(), "101st");
	assert_eq!(2.ord(), "2nd");
	assert_eq!(10093.ord(), "10093rd");
	assert_eq!((-159652).ord(), "-159652th");
	assert_eq!(0.ord(), "0th");
}