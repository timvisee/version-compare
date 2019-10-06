//! Version part module.
//!
//! A module that provides the `VersionPart` enum, with the specification of all available version
//! parts. Each version string is broken down into these version parts when being parsed to a
//! `Version`.

use std::cmp::Ordering;
// use std::fmt;
use regex::Regex;

pub trait VersionPart<'a>{
    fn part(&self);
}

struct EpochPart<'a>(i32);
struct NumberPart<'a>(i32);
struct StringPart<'a>(&'a str);
struct PEP440StringPart<'a>(&'a str);

struct NumberComparator<'a> {
    priority: i8,
    value: i32
}

struct StringComparator<'a> {
    priority: i8,
    value: &'a str
}

struct PEP440StringComparator<'a> {
    priority: i8,
    value: &'a str,
}

impl<'a> PEP440StringComparator<'a> {
    fn pep440_order(&self, other: &'a str) -> Ordering {
        lazy_static! {
            static ref dev_re: Regex = Regex::new("(\\d*)(dev)(\\d*)").unwrap();
        }
        lazy_static! {
            static ref post_re: Regex = Regex::new("(\\d*)(post)(\\d*)").unwrap();
        }

        dev_re.is_match(self.value);
        Ordering::Equal
    }
}

impl<'a> PartialEq for NumberComparator<'a> {
    fn eq(&self, other: impl VersionPart<'a>) -> bool {
        self.priority.eq(other.priority) && self.value.eq(other.value)
    }
}

impl<'a> PartialEq for StringComparator<'a> {
    fn eq(&self, other: impl VersionPart<'a>) -> bool {
        self.priority.eq(other.priority) && self.value.eq(other.value)
    }
}

impl<'a> PartialEq for PEP440StringComparator<'a> {
    fn eq(&self, other: impl VersionPart<'a>) -> bool {
        self.priority.eq(other.priority) && self.value.eq(other.value)
    }
}

impl<'a> PartialOrd for NumberComparator<'a> {
    fn partial_cmp(&self, other: impl VersionPart<'a>) -> Option<Ordering> {
        match self.priority.partial_cmp(other.priority) {
            // Priority of self is lower (more important) that priority of other
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Greater) => Some(Ordering::Less),
            Some(Ordering::Equal) => self.value.partial_cmp(other.value),
            _ => panic!()
        }
    }
}

impl<'a> PartialOrd for StringComparator<'a> {
    fn partial_cmp(&self, other: impl VersionPart<'a>) -> Option<Ordering> {
        match self.priority.partial_cmp(other.priority) {
            // Priority of self is lower (more important) that priority of other
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Greater) => Some(Ordering::Less),
            Some(Ordering::Equal) => self.value.partial_cmp(other.value),
            _ => panic!()
        }
    }
}

impl<'a> PartialOrd for PEP440StringComparator<'a> {
    fn partial_cmp(&self, other: impl VersionPart<'a>) -> Option<Ordering> {
        match self.priority.partial_cmp(other.priority) {
            // Priority of self is lower (more important) that priority of other
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Greater) => Some(Ordering::Less),
            Some(Ordering::Equal) => PEP440StringComparator::pep440_order(self, other.value),
            _ => panic!()
        }
    }
}

impl<'a> VersionPart<'a> for EpochPart<'a> {
    fn part(&self) -> NumberComparator {NumberComparator{priority: 0, value: self.0}}
}

impl<'a> VersionPart<'a> for NumberPart<'a> {
    fn part(&self) -> NumberComparator {NumberComparator{priority: 1, value: self.0}}
}

impl<'a> VersionPart<'a> for StringPart<'a> {
    fn part(&self) -> StringComparator {StringComparator{ priority: 2, value: self.0 }}
}

impl<'a> VersionPart<'a> for PEP440StringPart<'a> {
    fn part(&self) -> PEP440StringComparator {PEP440StringComparator{priority: 2, value: self.0}}
}

// Simplified comparison idea from
// https://internals.rust-lang.org/t/simplifying-custom-comparison-and-hashing/5108
impl<'a, T> PartialEq for T
    where
        T: VersionPart<'a>
{
    fn eq(&self, other: &T) -> bool { self.part().eq(&other.part()) }
}

impl<'a, T> PartialOrd for T
    where
        T: VersionPart<'a>
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.part().partial_cmp(&other.part())
    }
}


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
    use crate::version_part::{EpochPart, StringPart, NumberPart};

    #[test]
    fn test_epoch_compare() {
        assert_eq!(EpochPart(0), EpochPart(0));
        assert!(EpochPart(0) < EpochPart(1));
        assert!(EpochPart(0) > NumberPart(1));
        assert!(EpochPart(0) > StringPart("abc"));
    }
}
