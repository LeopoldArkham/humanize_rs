use std::fmt::Write;

static DIGITS: [&'static str; 10] = ["pad", "one", "two", "three", "four", "five", "six", "seven",
                                     "eight", "nine"];

static TO_20: [&'static str; 10] = ["ten",
                                    "eleven",
                                    "twelve",
                                    "thirteen",
                                    "fourteen",
                                    "fifteen",
                                    "sixteen",
                                    "seventeen",
                                    "eighteen",
                                    "nineteen"];

static TENS: [&'static str; 10] = ["padding", "ten", "twenty", "thirty", "fourty", "fifty",
                                   "sixty", "seventy", "eighty", "ninety"];

static SCALE: [&'static str; 9] = ["",
                                   " thousand ",
                                   " million ",
                                   " billion ",
                                   " trillion ",
                                   " quadrillion ",
                                   " quintillion ",
                                   " hexillion ",
                                   " heptillion "];

pub trait HumanizeNumbers {
    fn ord(&self) -> String;
    // fn to_text(&self) -> String;
}

fn stringify(res: &mut String, chunk: Vec<u64>) {
    match chunk.len() {
    	3 => {
    		match chunk[1] {
    			1 => {
    				*res += DIGITS[chunk[0] as usize];
            		*res += " hundred and ";
            		*res += TO_20[chunk[2] as usize];
    			}
    			0 => {
    				*res += DIGITS[chunk[0] as usize];
            		*res += " hundred";
            		if chunk[2] != 0 {
                		*res += " and ";
                		*res += DIGITS[chunk[2] as usize]
            		}
    			}
    			_ => {
    				if chunk[0] != 0 {
	    				*res += DIGITS[chunk[0] as usize];
	            		*res += " hundred and ";
	            	}
	            	*res += TENS[chunk[1] as usize];
            		if chunk[2] != 0 {
                		*res += "-";
                		*res += DIGITS[chunk[2] as usize]
            		}
    			}
    		}
    	}
    	2 => {
    		match chunk[0] {
    			1 => {
    				*res += TO_20[chunk[1] as usize]
    			}
    			_ => {
    				*res += TENS[chunk[0] as usize];
            		if chunk[1] != 0 {
                		*res += "-";
                		*res += DIGITS[chunk[1] as usize]
            		}
        		}
    		}
    	}
    	1 => {
    		*res += DIGITS[chunk[0] as usize];
    	}
    	_ => unreachable!{}
    }
}

pub fn to_text(_num: u64) -> String {
    if _num == 0 {
        return "zero".to_string();
    }

    let mut num = _num;
    let mut split_digits = Vec::new();
    // let mut divider: u64 = 10;

    while num > 0 {
        // let cur = num % 10;
        split_digits.insert(0, num % 10);
        num /= 10
        // divider *= 10;
    }
    	
	let (first, remainder) = split_digits.split_at(split_digits.len() % 3);
	let chunks = first.chunks(3).chain(remainder.chunks(3)).map(|x| x.to_vec()).collect::<Vec<_>>();

	let mut res = String::new();
	let mut scale_idx = chunks.len();

	for c in chunks {
		stringify(&mut res, c);
		scale_idx -= 1;
		res += SCALE[scale_idx];
	}

	res

}

impl HumanizeNumbers for u8 {
    fn ord(&self) -> String {
        let suffix = match self % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        let mut res = String::new();
        let _ = write!(res, "{}{}", self, suffix);
        res
    }
}

impl HumanizeNumbers for i8 {
    fn ord(&self) -> String {
        let suffix = match self % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
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
            _ => "th",
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
            _ => "th",
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
            _ => "th",
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
            _ => "th",
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
            _ => "th",
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
            _ => "th",
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
