use super::humanize::Humanize;

static TO_TRUE: [&'static str; 9] = ["1", "yes", "y", "ok", "okay", "continue", "go", "on", "true"];

static TO_FALSE: [&'static str; 7] = ["0", "no", "n", "stop", "end", "off", "false"];

fn normalize(source: &str) -> String {
    source.trim().to_lowercase()
}

pub trait ToBool {
    fn to_bool(&self) -> Option<bool>;
}

impl ToBool for &'static str {
    fn to_bool(&self) -> Option<bool> {
        let s = &normalize(self);
        if TO_TRUE.iter().any(|x| x == s) {
            Some(true)
        } else if TO_FALSE.iter().any(|x| x == s) {
            Some(false)
        } else {
            None
        }
    }
}

impl ToBool for String {
    fn to_bool(&self) -> Option<bool> {
        let s = &normalize(self);
        if TO_TRUE.iter().any(|x| x == s) {
            Some(true)
        } else if TO_FALSE.iter().any(|x| x == s) {
            Some(false)
        } else {
            None
        }
    }
}

impl Humanize for bool {
    fn humanize(&self) -> &str {
        match *self {
            true => "yes",
            false => "no",
        }
    }
}

#[test]
fn str_to_bool() {
    assert_eq!("yes".to_bool(), Some(true));
    assert_eq!("stop".to_bool(), Some(false));
}
#[test]
fn str_to_bool_needs_normalizing() {
    assert_eq!("\tYES".to_bool(), Some(true));
    assert_eq!("Stop  ".to_bool(), Some(false));
}

#[test]
fn string_to_bool() {
    let s = String::from("ok");
    assert_eq!(s.to_bool(), Some(true));

    let s2 = String::from("stop");
    assert_eq!(s2.to_bool(), Some(false));
}

#[test]
fn string_to_bool_needs_normalizing() {
    let s = String::from("\nOK");
    assert_eq!(s.to_bool(), Some(true));

    let s2 = String::from(" Off");
    assert_eq!(s2.to_bool(), Some(false));
}

#[test]
fn bool_to_str() {
    assert_eq!(true.humanize(), "yes");
}
