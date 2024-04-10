use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod tests {

    use claims::{assert_err, assert_ok};

    use crate::types::name::Name;

    #[test]
    fn empty_string_is_invalid() {
        let name = "".to_string();

        assert_err!(Name::parse(name));
    }

    #[test]
    fn whitespace_only_name_is_invalid() {
        let name = " ".repeat(12);

        assert_err!(Name::parse(name));
    }

    #[test]
    fn a_256_grapheme_name_is_valid() {
        let name = "a".repeat(256);

        assert_ok!(Name::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_invalid() {
        let name = "a".repeat(257);

        assert_err!(Name::parse(name));
    }

    #[test]
    fn a_name_with_invalid_characters_is_invalid() {
        for c in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = format!("{}{}", "a".repeat(12), c);

            assert_err!(Name::parse(name));
        }
    }
    #[test]
    fn a_name_with_valid_characters_is_valid() {
        let name = "valid name".to_string();

        assert_ok!(Name::parse(name));
    }
}

#[derive(Debug)]
pub struct Name(String);

impl Name {
    /// A valid name is not empty, is not longer than 256 characters and does not contain any
    /// of the following characters: `['/', '(', ')', '"', '<', '>', '\\', '{', '}']`
    pub fn parse(name: String) -> Result<Self, String> {
        // trim and check if empty
        let is_empty = name.trim().is_empty();

        let is_too_long = name.graphemes(true).count() > 256;

        // check if name contains any invalid characters
        let invalid_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_invalid_characters = name.chars().any(|c| invalid_characters.contains(&c));

        if is_empty || is_too_long || contains_invalid_characters {
            Err(format!("{} is not a valid name", name))
        } else {
            Ok(Self(name))
        }
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}