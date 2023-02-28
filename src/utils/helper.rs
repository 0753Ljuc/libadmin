use rocket_auth::User;

use crate::custom_error::{Error, ErrorType};

pub async fn check_auth(user: &Option<User>) -> Result<(), Error> {
    if cfg!(debug_assertions) {
        return Ok(());
    }
    if user.is_none() {
        return Err(Error::new(ErrorType::Unauthorized, None));
    }
    Ok(())
}
