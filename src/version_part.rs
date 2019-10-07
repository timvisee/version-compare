//! Version part module.
//!
//! A module that provides the `VersionPart` enum, with the specification of all available version
//! parts. Each version string is broken down into these version parts when being parsed to a
//! `Version`.

use std::cmp::Ordering;
// use regex::Regex;

// unifying trait so that they can be used together in the enum
#[enum_dispatch]
pub trait VersionTrait<T: PartialOrd + PartialEq> {
    // separate comparison of priority from value because priority will always match type,
    //    but value will not
}

#[derive(Debug, Clone, Copy)]
pub struct Epoch<T: PartialOrd + PartialEq> {
    pub value: T
}

impl<T: PartialOrd + PartialEq> VersionTrait<T> for Epoch<T> {
}
impl<T: PartialOrd + PartialEq> PartialOrd for Epoch<T> {
    fn partial_cmp(&self, other: &Epoch<T>) -> Option<Ordering> {self.value.partial_cmp(&other.value)}
}
impl<T: PartialOrd + PartialEq> PartialEq for Epoch<T> {
    fn eq(&self, other: &Epoch<T>) -> bool {self.partial_cmp(&other) == Some(Ordering::Equal)}
}

macro_rules! stratum {
    
}

#[enum_dispatch(VersionTrait)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum VersionPart<T: PartialOrd + PartialEq> {
    Epoch(i32),
    Integer(i32),
    // LexicographicString(LexicographicString<T>)
}

// impl PartialOrd<Enum> for VersionPart


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
    use crate::version_part::{VersionPart, Epoch};

    #[test]
    fn epoch_compare() {
        assert_eq!(Epoch{value: 0}, Epoch{value: 0});
        assert_eq!(Epoch{value: 0} < Epoch{value: 1});
        // epoch of any value trumps integer (priority)
        // assert!(VersionPart::Epoch(value: 0) > VersionPart::Integer(value: 1);
        // assert!(Version::Epoch{0} > Version::String{"abc"});
    }

    #[test]
    fn cross_type_compare() {
        assert!(VersionPart::Epoch(Epoch{value: 0}) > VersionPart::Integer(Epoch{value: 1}));
    }
}
