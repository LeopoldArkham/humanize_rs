/// This trait exopses all manipulations on string types
pub trait Strings {
    /// Removes leading whitespace until a string is found, and returns
    /// that string capitalized.
    ///
    /// # Panics
    /// Panics when called on an empty string, or a string containing only whitespace.
    ///
    /// # Examples
    /// ```
    /// use humanize::strings::Strings;
    /// let s = "hello, world";
    ///
    /// assert_eq!(s.capitalize(), "Hello, world");
    /// ```
    fn capitalize(self) -> String;
    /// Capitalizes all words in a String.
    /// 
    /// # Panics
    /// Panic when called on an empty string, or a string containing only whitespace.
    ///
    /// # Examples
    /// ```
    /// use humanize::strings::Strings;
    /// let title = "foo and bar: a comparative study";
    ///
    /// assert_eq!("Foo And Bar: A Comparative Study ", title.title_case());
    /// ```
    fn title_case(self) -> String;
}

impl<'a> Strings for &'a str {
    fn capitalize(self) -> String {
        let first = self.chars().nth(0).expect("Empty string");
        first.to_uppercase().collect::<String>() + &self.chars().skip(1).collect::<String>()
    }

    fn title_case(self) -> String {
        // TODO: Validate input
        let mut res = String::with_capacity(self.len());
        for word in self.split_whitespace() {
            res += &(word.capitalize() + " "); // TODO: No space on last word
        }
        res
    }
}

#[test]
fn test_capitalize() {
    assert_eq!("hi".capitalize(), "Hi");
    assert_eq!("6".capitalize(), "6");
}

#[test]
fn test_title_case() {
    assert_eq!("this is a title".title_case(), "This Is A Title ");
}