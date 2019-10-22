use std::cmp::Ordering;
use std::fmt;
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
            static ref RE: Regex = Regex::new(r"^(\d*)([a-zA-Z]*)(\d*)").unwrap();
        }

        let caps = RE.captures(input).unwrap();
        let pre: i16 = caps.get(1).map_or(0, |m| match m.as_str().is_empty() {
            true => 0,
            false => m.as_str().parse().unwrap()
        });
        let alpha = caps.get(2).map_or("", |m| m.as_str());
        let post: i16 = caps.get(3).map_or(0, |m| match m.as_str().is_empty() {
            true => 0,
            false => m.as_str().parse().unwrap()
        });

        PEP440String{ pre, alpha, post }
    }

    pub fn empty() -> PEP440String<'a> {
        PEP440String {pre: 0, alpha: "", post: 0}
    }
}

fn compare_pep440_str<'a>(left: &'a str, right: &'a str) -> Option<Ordering> {
    lazy_static! { static ref DEV_RE: Regex = Regex::new("dev").unwrap(); }
    lazy_static! { static ref POST_RE: Regex = Regex::new("post").unwrap(); }

    let is_dev = (DEV_RE.is_match(left), DEV_RE.is_match(right));
    let is_post = (POST_RE.is_match(left), POST_RE.is_match(right));

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

impl<'a> fmt::Display for PEP440String<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.pre, self.alpha, self.post)
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::PEP440String;

    #[test]
    fn compare_implict_leading_zero() {
        assert_eq!(PEP440String::new("0dev"), PEP440String::new("dev"));
        // epoch of any value trumps integer (priority)
        // assert!(VersionPart::Epoch(value: 0) > VersionPart::Integer(value: 1);
        // assert!(Version::Epoch{0} > Version::String{"abc"});
    }
}