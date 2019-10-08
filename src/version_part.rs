//! Version part module.
//!
//! A module that provides the `VersionPart` enum, with the specification of all available version
//! parts. Each version string is broken down into these version parts when being parsed to a
//! `Version`.

use std::cmp::Ordering;
// use regex::Regex;

#[derive(Debug, Copy, Clone)]
pub struct PEP440String {}

#[derive(Debug, Copy, Clone)]
pub enum VersionPart<'a> {
    Epoch(i16),
    Integer(i32),
    LexicographicString(&'a str),
    PEP440String(PEP440String),
    Bool(bool),
}

impl<'a> PartialOrd for VersionPart<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (VersionPart::Epoch(a), VersionPart::Epoch(b)) => a.partial_cmp(b),
            (VersionPart::Integer(a), VersionPart::Integer(b)) => a.partial_cmp(b),
            (VersionPart::LexicographicString(a), VersionPart::LexicographicString(b)) => a.partial_cmp(b),
            (VersionPart::Bool(a), VersionPart::Bool(b)) => a.partial_cmp(b),
            // Match simple position in the list, but reverse it because things at the top are higher
            _ => Some(match self {
                &VersionPart::Epoch(a) => 0,
                &VersionPart::Integer(a) => 1,
                &VersionPart::LexicographicString(a) => 2,
                &VersionPart::PEP440String(a) => 3,
                &VersionPart::Bool(a) => 4,
                _ => panic!()
            }.partial_cmp(
                match other {
                    &VersionPart::Epoch(a) => &0,
                    &VersionPart::Integer(a) => &1,
                    &VersionPart::LexicographicString(a) => &2,
                    &VersionPart::PEP440String(a) => &2,
                    &VersionPart::Bool(a) => &4,
                    _ => panic!()
                }
            ).unwrap().reverse())
        }
    }
}

impl<'a> PartialEq for VersionPart<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}


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
    use crate::version_part::{VersionPart};

    #[test]
    fn epoch_compare() {
        assert_eq!(VersionPart::Epoch(0), VersionPart::Epoch(0));
        assert!(VersionPart::Epoch(0) < VersionPart::Epoch(1));
        // epoch of any value trumps integer (priority)
        // assert!(VersionPart::Epoch(value: 0) > VersionPart::Integer(value: 1);
        // assert!(Version::Epoch{0} > Version::String{"abc"});
    }

    #[test]
    fn cross_type_compare() {
        assert!(VersionPart::Epoch(0) > VersionPart::Integer(1));
    }
}
