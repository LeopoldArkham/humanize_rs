use std::sync::Mutex;

static DEFAULT_TO_TRUE: [&'static str; 2] = ["1", "ok"];
static DEFAULT_TO_FALSE: [&'static str; 1] = ["0"];

static TO_TRUE: [&'static str; 7] = ["yes", "y", "okay", "continue", "go", "on", "true"];
static TO_FALSE: [&'static str; 6] = ["no", "n", "stop", "end", "off", "false"];

static TO_TRUE_FR: [&'static str; 5] = ["oui", "o", "d'accord", "vrai", "vraie"];
static TO_FALSE_FR: [&'static str; 3] = ["non", "faux", "n"];

lazy_static! {
    static ref ENABLED_LANG: Mutex<Vec<Lang>> = Mutex::new(Vec::new());
    static ref TO_TRUE_LIST: Mutex<Vec<&'static str>> = Mutex::new(vec!["1", "ok"]);
    static ref TO_FALSE_LIST: Mutex<Vec<&'static str>> = Mutex::new(vec!["0"]);
}

#[derive(Debug, PartialEq)]
pub enum Lang {
    English,
    French,
    Spanish,
    Italian,
}

pub fn enable_lang(lang: Lang) {
    {
        let mut lock = ENABLED_LANG.lock().unwrap();
        // Check if lang wasn't already added
        if lock.iter().any(|e| *e == lang) {
            return;
        }
        lock.push(lang);
    }
    gen_list();
}

fn gen_list() {
    let lang = ENABLED_LANG.lock().unwrap();
    let mut to_true_list = TO_TRUE_LIST.lock().unwrap();
    let mut to_false_list = TO_FALSE_LIST.lock().unwrap();
    to_true_list.extend(DEFAULT_TO_TRUE.iter());
    to_false_list.extend(DEFAULT_TO_FALSE.iter());
    for l in lang.iter() {
        match *l {
            Lang::English => {
                to_true_list.extend(TO_TRUE.iter());
                to_false_list.extend(TO_FALSE.iter());
            }
            Lang::French => {
                to_true_list.extend(TO_TRUE_FR.iter());
                to_false_list.extend(TO_FALSE_FR.iter());
            }
            _ => {}
        }
    }
}

fn to_true_list() -> Vec<&'static str> {
    TO_TRUE_LIST.lock().unwrap().clone()
}

fn to_false_list() -> Vec<&'static str> {
    TO_FALSE_LIST.lock().unwrap().clone()
}

fn normalize(source: &str) -> String {
    source.trim().to_lowercase()
}

pub trait ToBool {
    fn to_bool(&self) -> Option<bool>;
}

impl ToBool for &'static str {
    fn to_bool(&self) -> Option<bool> {
        let s = &normalize(self);
        if to_true_list().iter().any(|x| x == s) {
            Some(true)
        } else if to_false_list().iter().any(|x| x == s) {
            Some(false)
        } else {
            None
        }
    }
}

impl ToBool for String {
    fn to_bool(&self) -> Option<bool> {
        let s = &normalize(self);
        if to_true_list().iter().any(|x| x == s) {
            Some(true)
        } else if to_false_list().iter().any(|x| x == s) {
            Some(false)
        } else {
            None
        }
    }
}


#[test]
fn str_to_bool() {
    enable_lang(Lang::English);
    assert_eq!("yes".to_bool(), Some(true));
    assert_eq!("stop".to_bool(), Some(false));
}

#[test]
fn str_to_bool_needs_normalizing() {
    enable_lang(Lang::English);
    assert_eq!("\tYES".to_bool(), Some(true));
    assert_eq!("Stop  ".to_bool(), Some(false));
}

#[test]
fn string_to_bool() {
    enable_lang(Lang::English);
    let s = String::from("ok");
    assert_eq!(s.to_bool(), Some(true));

    let s2 = String::from("stop");
    assert_eq!(s2.to_bool(), Some(false));
}

#[test]
fn string_to_bool_needs_normalizing() {
    enable_lang(Lang::English);
    let s = String::from("\nOK");
    assert_eq!(s.to_bool(), Some(true));

    let s2 = String::from(" Off");
    assert_eq!(s2.to_bool(), Some(false));
}

#[test]
fn str_to_bool_multiple_lang() {
    enable_lang(Lang::English);
    enable_lang(Lang::French);
    assert_eq!("yes".to_bool(), Some(true));
    assert_eq!("oui".to_bool(), Some(true));
    assert_eq!("no".to_bool(), Some(false));
    assert_eq!("non".to_bool(), Some(false));
}

#[test]
fn bool_to_str() {
    //    assert_eq!(true.humanize(), "yes");
}
