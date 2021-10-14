//! Version compare module, with useful static comparison methods.

use crate::version::Version;
use crate::Cmp;

/// Compare two version number strings to each other.
///
/// This compares version `a` to version `b`, and returns whether version `a` is greater, less
/// or equal to version `b`.
///
/// The two given version numbers must be valid, or an error will be returned.
///
/// One of the following ok results may be returned:
///
/// * `Cmp::Eq`
/// * `Cmp::Lt`
/// * `Cmp::Gt`
///
/// # Examples
///
/// ```
/// use version_compare::{Cmp, compare};
///
/// // Compare version numbers
/// assert_eq!(compare("1.2.3", "1.2.3"), Ok(Cmp::Eq));
/// assert_eq!(compare("1.2.3", "1.2.4"), Ok(Cmp::Lt));
/// assert_eq!(compare("1", "0.1"), Ok(Cmp::Gt));
/// ```
pub fn compare<A, B>(a: A, b: B) -> Result<Cmp, ()>
where
    A: AsRef<str>,
    B: AsRef<str>,
{
    let a = Version::from(a.as_ref());
    let b = Version::from(b.as_ref());

    // Both version numbers must have been parsed
    if a.is_none() || b.is_none() {
        return Err(());
    }

    // Compare and return the result
    Ok(a.unwrap().compare(b.unwrap()))
}

/// Compare two version number strings to each other and test against the given comparison
/// `operator`.
///
/// The two given version numbers must be valid, or an error will be returned.
///
/// # Examples
///
/// ```
/// use version_compare::{Cmp, compare_to};
///
/// // Compare version numbers
/// assert!(compare_to("1.2.3", "1.2.3", Cmp::Eq).unwrap());
/// assert!(compare_to("1.2.3", "1.2.3", Cmp::Le).unwrap());
/// assert!(compare_to("1.2.3", "1.2.4", Cmp::Lt).unwrap());
/// assert!(compare_to("1", "0.1", Cmp::Gt).unwrap());
/// assert!(compare_to("1", "0.1", Cmp::Ge).unwrap());
/// ```
pub fn compare_to<A, B>(a: A, b: B, operator: Cmp) -> Result<bool, ()>
where
    A: AsRef<str>,
    B: AsRef<str>,
{
    let a = Version::from(a.as_ref());
    let b = Version::from(b.as_ref());

    // Both version numbers must have been parsed
    if a.is_none() || b.is_none() {
        return Err(());
    }

    // Compare and return the result
    Ok(a.unwrap().compare_to(&b.unwrap(), operator))
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use crate::test::{COMBIS, COMBIS_ERROR};
    use crate::Cmp;

    #[test]
    fn compare() {
        // Compare each version in the version set
        for entry in COMBIS {
            assert_eq!(
                super::compare(entry.0, entry.1),
                Ok(entry.2),
                "Testing that {} is {} {}",
                entry.0,
                entry.2.sign(),
                entry.1,
            );
        }

        // Compare each error version in the version set
        for entry in COMBIS_ERROR {
            let result = super::compare(entry.0, entry.1);

            if result.is_ok() {
                assert!(result != Ok(entry.2));
            }
        }
    }

    #[test]
    fn compare_to() {
        // Compare each version in the version set
        for entry in COMBIS {
            // Test
            assert!(super::compare_to(entry.0, entry.1, entry.2).unwrap());

            // Make sure the inverse operator is not correct
            assert_eq!(
                super::compare_to(entry.0, entry.1, entry.2.invert()).unwrap(),
                false
            );
        }

        // Compare each error version in the version set
        for entry in COMBIS_ERROR {
            let result = super::compare_to(entry.0, entry.1, entry.2);

            if result.is_ok() {
                assert!(!result.unwrap())
            }
        }

        // Assert an exceptional case, compare to not equal
        assert!(super::compare_to("1.2.3", "1.2", Cmp::Ne).unwrap());
    }
}
