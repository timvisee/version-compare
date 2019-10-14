//! Version part module.
//!
//! A module that provides the `VersionPart` enum, with the specification of all available version
//! parts. Each version string is broken down into these version parts when being parsed to a
//! `Version`.

use std::cmp::Ordering;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
pub struct PEP440String<'a> {
    pre: i16,
    alpha: &'a str,
    post: i16,
}

impl<'a> PEP440String<'a> {
    fn new(input: &'a str) -> PEP440String {
        lazy_static! {
            static ref re: Regex = Regex::new("(\\d*)(post|dev|[a-zA-Z]+)(\\d*)").unwrap();
        }

        let caps = re.captures(input).unwrap();
        let pre: i16 = caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
        let alpha = caps.get(2).map_or("", |m| m.as_str());
        let post: i16 = caps.get(3).map_or(0, |m| m.as_str().parse().unwrap());

        PEP440String{pre, alpha, post }
    }
}

fn compare_pep440_str<'a>(left: &'a str, right: &'a str) -> Option<Ordering> {
    lazy_static! { static ref dev_re: Regex = Regex::new("dev").unwrap(); }
    lazy_static! { static ref post_re: Regex = Regex::new("post").unwrap(); }

    let is_dev = (dev_re.is_match(left), dev_re.is_match(right));
    let is_post = (post_re.is_match(left), post_re.is_match(right));

    let str_match = left.partial_cmp(right);
    match str_match {
        Some(Ordering::Equal) => Some(Ordering::Equal),
        _ => match is_dev {
            (false, true) => Some(Ordering::Greater),
            (true, false) => Some(Ordering::Less),
            _ => match is_post {
                (true, true) => Some(Ordering::Equal),
                (false, true) => Some(Ordering::Less),
                (true, false) => Some(Ordering::Greater),
                // this is the final fallback to lexicographic sorting, if neither
                //   dev nor post are in effect
                (false, false) => left.partial_cmp(right),
                _ => panic!(),
            }
        }
    }
}

impl<'a> PartialOrd for PEP440String<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.pre.partial_cmp(&other.pre) {
            Some(Ordering::Greater) => Some(Ordering::Greater),
            Some(Ordering::Less) => Some(Ordering::Less),
            Some(Ordering::Equal) => match compare_pep440_str(self.alpha, &other.alpha) {
                Some(Ordering::Equal) => self.post.partial_cmp(&other.post),
                Some(Ordering::Greater) => Some(Ordering::Greater),
                Some(Ordering::Less) => Some(Ordering::Less),
                _ => panic!()
            }
            _ => panic!()
        }
    }
}

impl<'a> PartialEq for PEP440String<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(&other).unwrap() == Ordering::Equal
    }
}


#[derive(Debug, Copy, Clone)]
pub enum VersionPart<'a> {
    Epoch(i16),
    Integer(i32),
    LexicographicString(&'a str),
    PEP440String(&'a PEP440String<'a>),
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
                    &VersionPart::PEP440String(a) => &3,
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
