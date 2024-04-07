pub mod auth;
pub mod schema;
use diesel::pg::PgConnection;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use std::error::Error;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

   
}

fn current_user(conn: &mut PgConnection) -> Result<auth::User, Box<dyn Error>> {
    match auth::current_user_from_env(conn) {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err("No user found with the given username".into()),
        Err(e) => Err(convert_auth_error(e)),
    }
}

fn register_user(conn: &mut PgConnection) -> Result<(), Box<dyn Error>> {
    use auth::AuthError as Auth;
    use diesel::result::DatabaseErrorKind::UniqueViolation;
    use diesel::result::Error::DatabaseError;

    match auth::register_user_from_env(conn) {
        Ok(_) => Ok(()),
        Err(Auth::DatabaseError(DatabaseError(UniqueViolation, _))) => {
            Err("A user with that name already exists".into())
        }
        Err(e) => Err(convert_auth_error(e)),
    }
}

fn convert_auth_error(err: auth::AuthError) -> Box<dyn Error> {
    use auth::AuthError::*;

    match err {
        IncorrectPassword => "The password given does not match our records".into(),
        NoNameSet => {"No name given. You need to set NAME enviroment variable ".into()},
        NoEmailSet => {
            "No email given. You need to set the EMAIL environment variable.".into()
        }
        NoPasswordSet => {
            "No password given. You need to set the PASSWORD environment variable.".into()
        }
        EnvironmentError(e) => e.into(),
        Argon2Error(e) => e.to_string().into(),
        DatabaseError(e) => e.into(),
    }
}