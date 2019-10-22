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
            }
            Err(_) => {
                // Push the text part to the vector
                parts.push(VersionPart::LexicographicString(part));
            }
        }
    }

    if parts.is_empty() && version.is_empty() {
        parts.push(VersionPart::Empty);
    }

    // Return the list of parts
    Some(parts)
}