//! Version part module.
//!
//! A module that provides the `VersionPart` enum, with the specification of all available version
//! parts. Each version string is broken down into these version parts when being parsed to a
//! `Version`.

use std::cmp::Ordering;
// use regex::Regex;

pub trait VersionPart<T> {
    // separate comparison of priority from value because priority will always match type,
    //    but value will not
    fn compare_priority(&self, other: i8) -> Option<Ordering>;
    fn compare_value(&self, other: T) -> Option<Ordering>;
}

// cover most types that Rust knows how to compare already
#[derive(Debug, Clone, Copy)]
pub struct DefaultPart<T: Clone> {
    pub priority: i8,
    pub value: T,
}

impl<T: PartialOrd + Clone> VersionPart<T> for DefaultPart<T>
{
    fn compare_priority(&self, other: i8) -> Option<Ordering> {
        self.priority.partial_cmp(&other)
    }
    fn compare_value(&self, other: T) -> Option<Ordering> {
        self.value.partial_cmp(&other)
    }
}

impl<T, U> PartialEq<DefaultPart<U>> for DefaultPart<T>
    where
        T: From<U>,
        T: PartialOrd + Clone,
        U: PartialOrd + Clone,
{
    fn eq(&self, other: &DefaultPart<U>) -> bool {
        (self.compare_priority(other.priority) == Some(Ordering::Equal)) &&
            (self.compare_value(other.value.clone().into()) == Some(Ordering::Equal))
    }
}

// Provide default implementation for comparison -
impl<T, U> PartialOrd<DefaultPart<U>> for DefaultPart<T>
    where
        T: From<U>,
        T: PartialOrd + Clone,
        U: PartialOrd + Clone,
{
    fn partial_cmp(&self, other: &DefaultPart<U>) -> Option<Ordering> {
        match self.compare_priority(other.priority) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Greater) => Some(Ordering::Less),
            Some(Ordering::Equal) => self.compare_value(other.value.clone().into()),
            _ => panic!(),
        }
    }
}

pub struct LexicographicStringType {
    pub value: &str
}

impl PartialOrd for LexicographicStringType {
    
}

// // Name handles just to make it easier to instantiate things with default priority
pub fn get_epoch_part(v: i32) -> DefaultPart<i32> {DefaultPart{priority: 0, value: v }}
pub fn get_integer_part(v: i32) -> DefaultPart<i32> {DefaultPart{priority: 1, value: v }}
pub fn get_unsigned_integer_part(v: u32) -> DefaultPart<u32> {DefaultPart{priority: 1, value: v }}
pub fn get_lexicographic_string_part(v: &str) -> DefaultPart<&str> {DefaultPart{priority: 2, value: v }}

// struct PEP440StringComparator<'a> {
//     priority: i8,
//     value: &'a str,
// }

// impl<'a> PEP440StringComparator<'a> {
//     fn pep440_order(&self, other: &'a str) -> Ordering {
//         lazy_static! {
//             static ref dev_re: Regex = Regex::new("(\\d*)(dev)(\\d*)").unwrap();
//         }
//         lazy_static! {
//             static ref post_re: Regex = Regex::new("(\\d*)(post)(\\d*)").unwrap();
//         }

//         dev_re.is_match(self.value);
//         Ordering::Equal
//     }
// }

// Simplified comparison idea from
// https://internals.rust-lang.org/t/simplifying-custom-comparison-and-hashing/5108


//impl<'a> fmt::Display for VersionPart<'a> {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match self {
//            VersionPart::Number(n) => write!(f, "{}", n),
//            VersionPart::Text(t) => write!(f, "{}", t),
//        }
//    }
//}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use crate::version_part::{get_epoch_part,
                              get_integer_part,
                              get_lexicographic_string_part};

    #[test]
    fn epoch_compare() {
        assert_eq!(get_epoch_part(0), get_epoch_part(0));
        assert!(get_epoch_part(0) < get_epoch_part(1));
        // epoch of any value trumps integer (priority)
        assert!(get_epoch_part(0) > get_integer_part(1));
        assert!(get_epoch_part(0) > get_lexicographic_string_part("abc"));
    }
}
