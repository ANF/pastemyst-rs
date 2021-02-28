/// This macro can assist prevention of
/// converting an str to a String and
/// calling that method each time.
///
/// It can be used for other purposes
/// if you wish to, but it's designed
/// to work with any struct field that
/// takes in a String type. Not necessarily
/// recommened to be used however you can
/// if you wish to but it's not compulsory.
///
/// You can also use other macros like
/// methods like `String::from` or even
/// `to_string()` if you wish to.
///
/// This method calls the `String::from` method.
#[macro_export]
macro_rules! str {
    {$value:expr} => (String::from($value));
}

pub mod time;

pub mod data;

pub mod user;

pub mod paste;
