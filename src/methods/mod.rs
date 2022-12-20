//! This module contains all the non-admin functions that can be mapped to an
//! endpoint provided by the [`CCash`](https://github.com/EntireTwix/CCash) API.
//! All admin functions can be found within the [`admin`] module.

pub mod admin;

use crate::{
    request::request, CCashError, CCashResponse, CCashSession, CCashUser, TransactionLog,
    TransactionLogV2,
};
use reqwest::Method;
use velcro::hash_map;

/// Returns the balance of the [`user`](CCashUser).
///
/// # Errors
///
/// Will return `CCashError` if the request fails or if the response from
/// `CCash` cannot be parsed as a valid `u32`.
pub async fn get_balance(
    session: &CCashSession,
    user: &CCashUser,
) -> Result<u32, CCashError> {
    let url = format!(
        "{}/v1/user/balance?name={}",
        &session.session_url, &user.username
    );

    let r = request::<()>(Method::GET, session, &url, Some(user), None).await?;
    if let Ok(v) = r.convert_message::<u32>() {
        Ok(v)
    } else {
        Err(r.into())
    }
}

/// Returns the transaction logs for a given [`user`](CCashUser). This function
/// requires a correct password.
///
/// # Errors
///
/// Will return a `CCashError` if the request fails or if the data returned by
/// `CCash` cannot be parsed into a valid `Vec<TransactionLog>`.
#[deprecated(since = "2.0.0", note = "Please use `get_log_v2` where possible")]
pub async fn get_log(
    session: &CCashSession,
    user: &CCashUser,
) -> Result<Vec<TransactionLog>, CCashError> {
    let url = format!("{}/v1/user/log", &session.session_url);

    let r = request::<()>(Method::GET, session, &url, Some(user), None).await?;
    if let Ok(v) = r.convert_message::<Vec<TransactionLog>>() {
        Ok(v)
    } else {
        Err(r.into())
    }
}

/// Returns the transaction logs for a given [`user`](CCashUser). This function
/// requires a correct password.
///
/// # Errors
///
/// Will return a `CCashError` if the request fails or if the data returned by
/// `CCash` cannot be parsed into a valid `Vec<`[`TransactionLogV2`]`>`.
pub async fn get_log_v2(
    session: &CCashSession,
    user: &CCashUser,
) -> Result<Vec<TransactionLogV2>, CCashError> {
    let url = format!("{}/v2/user/log", &session.session_url);

    let r = request::<()>(Method::GET, session, &url, Some(user), None).await?;
    if let Ok(v) = r.convert_message::<Vec<TransactionLogV2>>() {
        Ok(v)
    } else {
        Err(r.into())
    }
}

/// Returns a `bool` about whether or not the the user with a given
/// [`user`](CCashUser) exists. This function does not require a password.
///
/// # Errors
///
/// Will return a `CCashError` if the request fails or if the `CCash` instance
/// returns an error code as long as the error code isn't a 401 and as long as
/// the `interpret_endpoint_errors_as_false` is disabled.
pub async fn contains_user(
    session: &CCashSession,
    user: &CCashUser,
) -> Result<bool, CCashError> {
    let url = format!(
        "{}/v1/user/exists?name={}",
        &session.session_url, &user.username
    );

    let r = request::<()>(Method::GET, session, &url, Some(user), None).await?;
    match r {
        CCashResponse::Success { .. } => Ok(true),
        #[cfg(feature = "interpret_endpoint_errors_as_false")]
        CCashResponse::Error { .. } => Ok(false),
        #[cfg(not(feature = "interpret_endpoint_errors_as_false"))]
        CCashResponse::Error { code: 401, .. } => Ok(false),
        #[cfg(not(feature = "interpret_endpoint_errors_as_false"))]
        CCashResponse::Error { .. } => Err(r.into()),
    }
}

/// Returns a `bool` about whether or not the `password` for a
/// given [`user`](CCashUser) is correct.
///
/// # Errors
///
/// Will return a `CCashError` if the request fails or if the `CCash` instance
/// returns an error code when verifing the password as long as the
/// `interpret_endpoint_errors_as_false` feature is disabled.
pub async fn verify_password(
    session: &CCashSession,
    user: &CCashUser,
) -> Result<bool, CCashError> {
    let url = format!("{}/v1/user/verify_password", &session.session_url);

    let r = request::<()>(Method::POST, session, &url, Some(user), None).await?;
    match r {
        CCashResponse::Success { .. } => Ok(true),
        #[cfg(feature = "interpret_endpoint_errors_as_false")]
        CCashResponse::Error { .. } => Ok(false),
        #[cfg(not(feature = "interpret_endpoint_errors_as_false"))]
        CCashResponse::Error { code: 401, .. } => Ok(false),
        #[cfg(not(feature = "interpret_endpoint_errors_as_false"))]
        CCashResponse::Error { .. } => Err(r.into()),
    }
}

/// Returns `true` about if a password change was successful for the given
/// [`user`](CCashUser). This function modifies `user` to use the `new_password`
/// instead of the previous password in the case of a successful password
/// change.
///
/// # Errors
///
/// Will return a `CCashError` if the request fails or if `CCash` instance
/// returns an error code when changing the password for the `user`, as long as
/// the `interpret_endpoint_errors_as_false` feature is disabled.
pub async fn change_password(
    session: &CCashSession,
    user: &mut CCashUser,
    new_password: &str,
) -> Result<bool, CCashError> {
    let url = format!("{}/v1/user/change_password", &session.session_url);
    let body = hash_map! { "pass": new_password };

    let r = request(Method::PATCH, session, &url, Some(user), Some(&body)).await?;
    match r {
        CCashResponse::Success { .. } => {
            user.update_password(new_password);
            Ok(true)
        },
        #[cfg(feature = "interpret_endpoint_errors_as_false")]
        CCashResponse::Error { .. } => Ok(false),
        #[cfg(not(feature = "interpret_endpoint_errors_as_false"))]
        CCashResponse::Error { .. } => Err(r.into()),
    }
}

/// Sends funds from the [`user`](CCashUser) to the user with the
/// `recipient_name`. This function returns the [`user`](CCashUser)'s balance
/// after a successful transaction has been made.
///
/// # Errors
///
/// Will return a `CCashError` if the request fails or if the message returned
/// back by `CCash` cannot be parsed into a `u32`.
pub async fn send_funds(
    session: &CCashSession,
    user: &CCashUser,
    recipient_name: &str,
    amount: u32,
) -> Result<u32, CCashError> {
    #[derive(serde::Serialize)]
    struct FundsTransfer {
        name: String,
        amount: u32,
    }

    let url = format!("{}/v1/user/transfer", &session.session_url);
    let body = FundsTransfer {
        name: recipient_name.into(),
        amount,
    };

    let r = request(Method::POST, session, &url, Some(user), Some(&body)).await?;

    if let Ok(v) = r.convert_message::<u32>() {
        Ok(v)
    } else {
        Err(r.into())
    }
}

/// Adds a [`user`](CCashUser) with a balance of 0.
///
/// Returns:
/// * `Ok(true)` if the `user` was created successfully on the `CCash` instance.
/// * `Ok(false)` if the `user` failed to be created on the `CCash` instance.
///   This either happens only an a 409 response from the instance or if the
///   feature `interpret_endpoint_errors_as_false` is enabled.
///
/// # Errors
///
/// Will return `CCashError` if the instance returns an error response
/// (other than a 409) *and* the feature `interpret_endpoint_errors_as_false` is
/// disabled.
pub async fn add_user(
    session: &CCashSession,
    user: &CCashUser,
) -> Result<bool, CCashError> {
    let url = format!("{}/v1/user/register", &session.session_url);

    let r = request(Method::POST, session, &url, None, Some(user)).await?;
    match r {
        CCashResponse::Success { .. } => Ok(true),
        #[cfg(feature = "interpret_endpoint_errors_as_false")]
        CCashResponse::Error { .. } => Ok(false),
        #[cfg(not(feature = "interpret_endpoint_errors_as_false"))]
        CCashResponse::Error { code: 409, .. } => Ok(false),
        #[cfg(not(feature = "interpret_endpoint_errors_as_false"))]
        CCashResponse::Error { .. } => Err(r.into()),
    }
}

/// Removes the [`user`](CCashUser). This function requires the
/// [`user`](CCashUser) to be a valid username and password otherwise the
/// endpoint will return an error.
///
/// # Errors
///
/// Will return [`CCashError`] if request fails.
pub async fn delete_user(
    session: &CCashSession,
    user: &CCashUser,
) -> Result<(), CCashError> {
    let url = format!("{}/v1/user/delete", &session.session_url);

    let r = request::<()>(Method::DELETE, session, &url, Some(user), None).await?;
    match r {
        CCashResponse::Success { .. } => Ok(()),
        CCashResponse::Error { .. } => Err(r.into()),
    }
}
