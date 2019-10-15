use crate::version_part::VersionPart;

/// Split the given version string, in it's version parts.
/// TODO: Move this method to some sort of helper class, maybe as part of `VersionPart`.
pub fn default_parser(
    version: &str,
) -> Option<Vec<VersionPart>> {
    // Split the version string, and create a vector to put the parts in
    // TODO: split at specific separators instead
    let split = version.split(|c| !char::is_alphanumeric(c));
    let mut parts = Vec::new();

    // Flag to determine whether this version number contains any number part
    let mut has_number = false;

    // Loop over the parts, and parse them
    for part in split {
        // Skip empty parts
        if part.is_empty() {
            continue;
        }

        // Try to parse the value as an number
        match part.parse::<i32>() {
            Ok(number) => {
                // Push the number part to the vector, and set the has number flag
                parts.push(VersionPart::Integer(number));
                has_number = true;
            }
            Err(_) => {
                // Push the text part to the vector
                parts.push(VersionPart::LexicographicString(part));
            }
        }
    }

    // The version must contain a number part, if any part was parsed
    if !has_number && !parts.is_empty() {
        return None;
    }

    // Return the list of parts
    Some(parts)
}