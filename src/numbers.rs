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

fn stringify(res: &mut String, chunk: &Vec<&u64>) {
    match chunk.as_slice() {
        &[_, &1, _] => {
            *res += DIGITS[*chunk[0] as usize];
            *res += " hundred and ";
            *res += TO_20[*chunk[2] as usize];
        }
        &[_, &0, _] => {
            *res += DIGITS[*chunk[0] as usize];
            *res += " hundred ";
            if *chunk[2] != 0 {
                *res += "and ";
                *res += DIGITS[*chunk[2] as usize]
            }
        }
        &[_, _, _] => {
            *res += DIGITS[*chunk[0] as usize];
            *res += " hundred and ";
            *res += TENS[*chunk[1] as usize];
            if *chunk[2] != 0 {
                *res += "-";
                *res += DIGITS[*chunk[2] as usize]
            }
        }
        &[&1, _] => {
            *res += TO_20[*chunk[1] as usize];
        }
        &[_, _] => {
            *res += TENS[*chunk[0] as usize];
            if *chunk[1] != 0 {
                *res += "-";
                *res += DIGITS[*chunk[1] as usize]
            }
        }
        &[_] => {
            *res += DIGITS[*chunk[0] as usize];
        }
        _ => unreachable!(),
    }
}

pub fn to_text(_num: u64) -> String {
    if _num == 0 {
        return "zero".to_string();
    }

    let mut num = _num;
    let mut split_digits = Vec::new();
    let mut divider = 10;

    while num > 0 {
        let cur = num % divider;
        split_digits.push(cur / (divider / 10));
        num -= cur;
        println!("{}", cur);
        divider *= 10;
    }
    // TODO Change this to something less suicide-inducing
    let chunks = &split_digits.chunks(3)
        .collect::<Vec<_>>()
        .iter()
        .map(|e| e.iter().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let chunks = chunks.iter().rev().collect::<Vec<_>>();

    let mut result = String::new();
    let mut counter = chunks.len();
    for chunk in chunks.iter() {
        counter -= 1;
        stringify(&mut result, chunk);
        result += SCALE[counter as usize];
    }
    result
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
