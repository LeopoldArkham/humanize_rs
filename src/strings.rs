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
    /// let s = "hello, world";
    ///
    /// assert_eq!(s.capitalize(), "Hello, world");
    /// ```
    fn capitalize(self) -> String;
}

impl<'a> Strings for &'a str {
    fn capitalize(self) -> String {
        let first = self.chars().nth(0).expect("Empty string");
        first.to_uppercase().collect::<String>() + &self.chars().skip(1).collect::<String>()
    }
}

#[test]
fn test_capitalize() {
    assert_eq("hi", capitalize(), "Hi");
    assert_eq!("6".capitalize(), "6");
}