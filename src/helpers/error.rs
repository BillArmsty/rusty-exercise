

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();

    while let Some(cause) = current {
        writeln!(f, "Caused by: \n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}

#[derive(Debug)]
pub enum AuthError {
    IncorrectPassword,
    NoEmailSet,
    NoNameSet,
    NoPasswordSet,
    EnvironmentError(dotenvy::Error),
    Argon2Error(argon2::password_hash::Error),
    DatabaseError(diesel::result::Error),
    ConversionError(std::string::FromUtf8Error),
}

impl From<argon2::password_hash::Error> for AuthError {
    fn from(e: argon2::password_hash::Error) -> Self {
        AuthError::Argon2Error(e)
    }
}

impl From<diesel::result::Error> for AuthError {
    fn from(e: diesel::result::Error) -> Self {
        AuthError::DatabaseError(e)
    }
}

impl From<std::string::FromUtf8Error> for AuthError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        AuthError::ConversionError(e)
    }
}

impl From<dotenvy::Error> for AuthError {
    fn from(e: dotenvy::Error) -> Self {
        AuthError::EnvironmentError(e)
    }
}

