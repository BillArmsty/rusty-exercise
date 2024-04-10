use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use secrecy::{ExposeSecret, Secret};

#[cfg(test)]
mod tests {
    use claims::assert_err;
    use rand::{distributions::Uniform, rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
    use secrecy::{ExposeSecret, Secret};

    use crate::types::Password;

    #[test]
    fn empty_string_is_invalid() {
        let password = "".to_string();

        assert_err!(Password::parse(password.into()));
    }

    #[test]
    fn password_less_than_12_characters_is_invalid() {
        let password = "a".repeat(11);

        assert_err!(Password::parse(password.into()));
    }

    #[test]
    fn password_greater_than_128_characters_is_invalid() {
        let password = "a".repeat(129);

        assert_err!(Password::parse(password.into()));
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub Secret<String>);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let rng = &mut StdRng::seed_from_u64(u64::arbitrary(g));

            let mut gen = |chars: &str| -> String {
                let chars = chars.chars().collect::<Vec<_>>();
                let uniform = Uniform::new(0, chars.len());

                let len = rng.gen_range(1..=chars.len());
                (0..=len)
                    .map(|_| {
                        let idx = rng.sample(uniform);
                        chars[idx] as char
                    })
                    .collect()
            };

            let upper = gen("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
            let lower = gen("abcdefghijklmnopqrstuvwxyz");
            let number = gen("0123456789");
            let special = gen("!#$%&*][(){}+-^@~");
            let mut password = format!("{upper}{lower}{number}{special}")
                .chars()
                .collect::<Vec<_>>();

            password.shuffle(rng);
            let password = password.into_iter().collect::<String>();

            Self(password.into())
        }
    }

    #[quickcheck_macros::quickcheck]
    fn password_with_valid_length_is_parsed_correctly(password: ValidPasswordFixture) -> bool {
        println!("password: {:?}", password.0.expose_secret());
        Password::parse(password.0).is_ok()
    }
}

#[derive(Debug)]
pub struct Password(Secret<String>);

impl Password {
    pub fn parse(password: Secret<String>) -> Result<Self, String> {
        let password = password.expose_secret();
        if password.len() < 12 {
            return Err("Password must be at least 12 characters".into());
        }
        if password.len() > 128 {
            return Err("Password must be less than 128 characters".into());
        }
        let mut has_uppercase = false;
        let mut has_lowercase = false;
        let mut has_number = false;
        let mut has_special = false;
        for c in password.chars() {
            if c.is_uppercase() {
                has_uppercase = true;
            }
            if c.is_lowercase() {
                has_lowercase = true;
            }
            if c.is_numeric() {
                has_number = true;
            }
            if !c.is_alphanumeric() {
                has_special = true;
            }
        }
        // password must contain at least one uppercase letter
        if !has_uppercase {
            return Err("Password must contain at least one uppercase letter".into());
        }
        // password must contain at least one lowercase letter
        if !has_lowercase {
            return Err("Password must contain at least one lowercase letter".into());
        }
        // password must contain at least one number
        if !has_number {
            return Err("Password must contain at least one number".into());
        }
        // password must contain at least one special character
        if !has_special {
            return Err("Password must contain at least one special character".into());
        }

        let salt = SaltString::generate(&mut rand::thread_rng());
        match Argon2::default().hash_password(password.as_bytes(), &salt) {
            Ok(hash) => Ok(Self(hash.to_string().into())),
            Err(_) => Err("Failed to hash password".into()),
        }
    }
}

impl AsRef<Secret<String>> for Password {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}
