//! This module contains the relevant structures and enums relating to the data
//! of a `CCash` user.

use thiserror::Error;

#[derive(Error, Debug, Clone)]
/// Enum to describe the errors that could occur when trying to create a user
/// with an incorrect username.
pub enum CCashUsernameError {
    /// Error for if the username provided is too short to meet the
    /// requirements.
    #[error("CCashUserError: Name too short (needs to be atleast 3 characters)")]
    NameTooShort,
    /// Error for if the username provided is too long to meet the requirements.
    #[error("CCashUserError: Name too long (needs to be at most 16 characters)")]
    NameTooLong,
    /// Error for if the username contains illegal characters not allowed in the
    /// requirements.
    #[error("CCashUserError: Name contains invalid characters: {0}")]
    NameContainsInvalidCharacters(String),
}

/// User struct that can be used for authentication purposes.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq)]
pub struct CCashUser {
    #[serde(rename(serialize = "name"))]
    pub(crate) username: String,
    #[serde(rename(serialize = "pass"))]
    pub(crate) password: String,
}

impl CCashUser {
    /// Creates a new user where the username is checked against `CCash`'s
    /// requirements. This is the recommended way to create a `CCashUser` as
    /// it is guaranteed to not cause `CCash` to reject the username. `username`
    /// will automatically be made lowercase.
    ///
    /// # Errors
    ///
    /// - Returns `CCashUsernameError::NameTooShort` if given `username` is
    ///   shorter than 3 characters.
    /// - Returns `CCashUsernameError::NameTooLong` if given `username` is
    ///   longer than 16 characters.
    /// - Returns `CCashUsernameError::NameContainsInvalidCharacters` if given
    ///   `username` otherwise contains invalid characters such as spaces.
    pub fn new(username: &str, password: &str) -> Result<Self, CCashUsernameError> {
        let username = username.to_lowercase(); // usernames in `CCash` have to be lowercase. there's no point in iterating
                                                // through all the characters and checking, then erroring if a uppercase
                                                // character is found when an automatic conversion will
                                                // suffice.

        if username.len() < 3 {
            return Err(CCashUsernameError::NameTooShort);
        } else if username.len() > 16 {
            return Err(CCashUsernameError::NameTooLong);
        } else if username.contains(' ') {
            return Err(CCashUsernameError::NameContainsInvalidCharacters(
                "Name cannot contain spaces".into(),
            ));
        } else if !username
            .chars()
            .filter(|c| *c != '_')
            .all(char::is_alphanumeric)
        {
            return Err(CCashUsernameError::NameContainsInvalidCharacters(
                "Name cannot contain non-alphanumeric characters".into(),
            ));
        }

        Ok(Self {
            username,
            password: password.replace(' ', ""),
        })
    }

    /// Creates a new `CCashUser` with an unchecked username against `CCash`'s
    /// requirements. This method is not recommended as it could cause
    /// hard-to-diagnose or confusing errors from `CCash` for the user.
    #[must_use]
    pub fn new_unchecked(username: &str, password: &str) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Returns an immutable reference to the `CCashUser`'s username.
    #[must_use]
    pub fn get_username(&self) -> &str { &self.username }
    /// Returns an immutable reference to the `CCashUser`'s password.
    #[must_use]
    pub fn get_password(&self) -> &str { &self.password }

    pub(crate) fn update_password(&mut self, new_password: &str) {
        self.password = new_password.into();
    }
}
