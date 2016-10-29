mod bool {
    static TO_TRUE: [&'static str; 9] = ["1", "yes", "y", "ok", "okay", "continue", "go", "on",
                                         "true"];

    static TO_FALSE: [&'static str; 6] = ["0", "no", "n", "stop", "end", "false"];

    pub trait ToBool {
        fn to_bool(&self) -> Option<bool>;
    }

    impl ToBool for &'static str {
        fn to_bool(&self) -> Option<bool> {
            if TO_TRUE.iter().any(|x| x == self) {
                Some(true)
            } else if TO_FALSE.iter().any(|x| x == self) {
                Some(false)
            } else {
                None
            }
        }
    }

    impl ToBool for String {
        fn to_bool(&self) -> Option<bool> {
            if TO_TRUE.iter().any(|x| x == &self) {
                Some(true)
            } else if TO_FALSE.iter().any(|x| x == &self) {
                Some(false)
            } else {
                None
            }
        }
    }

    pub trait Humanize {
        fn humanize(&self) -> &str;
    }

    impl Humanize for bool {
        fn humanize(&self) -> &str {
            match *self {
                true => "true",
                false => "false",
            }
        }
    }

    #[test]
    fn str_to_bool() {
        assert_eq!("yes".to_bool(), Some(true));
        assert_eq!("stop".to_bool(), Some(false));
    }

    #[test]
    fn string_to_bool() {
        let s = String::from("ok");
        assert_eq!(s.to_bool(), Some(true));
    }

    #[test]
    fn bool_to_str() {
        assert_eq!(true.humanize(), "true");
    }
}
