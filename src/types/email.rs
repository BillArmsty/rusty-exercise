use validator::ValidateEmail;

#[cfg(test)]
mod tests {
    use claims::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};
    use rand::{rngs::StdRng, SeedableRng};

    use crate::types::email::Email;

    #[test]
    fn empty_string_is_invalid() {
        let email = "".to_string();

        assert_err!(Email::parse(email));
    }

    #[test]
    fn whitespace_only_email_is_invalid() {
        let email = " ".repeat(12);

        assert_err!(Email::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_invalid() {
        let email = "email.example.com".to_string();

        assert_err!(Email::parse(email));
    }

    #[test]
    fn email_missing_subject_is_invalid() {
        let email = "@example.com".to_string();

        assert_err!(Email::parse(email));
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));

            let email = SafeEmail().fake_with_rng(&mut rng);

            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_email_is_parsed_correctly(email: ValidEmailFixture) -> bool {
        Email::parse(email.0).is_ok()
    }
}

#[derive(Debug)]
pub struct Email(String);
impl Email {
    pub fn parse(email: String) -> Result<Self, String> {
        if email.validate_email() {
            Ok(Self(email))
        } else {
            Err(format!("{} is not a valid email address", email))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
