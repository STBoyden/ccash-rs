//! This module contains all the admin functions that can be mapped to an
//! endpoint provided by the [`CCash`](https://github.com/EntireTwix/CCash) API.
//! Non-admin functions can be found within [`methods`].
//!
//! [`methods`]: crate::methods

use crate::{request::request, CCashError, CCashResponse, CCashSession, CCashUser};
use reqwest::Method;
use velcro::hash_map;

/// Returns a boolean whether or not the [`user`](CCashUser) is an admin
/// account.
///
/// # Errors
///
/// Will return a `CCashError` if the request fails or if the `CCash` instance
/// returns an error code other than 401, as long as the
/// `interpret_endpoint_errors_as_false` feature is disabled.
pub async fn verify_account(
    session: &CCashSession,
    user: &CCashUser,
) -> Result<bool, CCashError> {
    let url = format!("{}/v1/admin/verify_account", &session.session_url);

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

/// Changes the password for the [`user`](CCashUser). This function modifies
/// `user` to use the `new_password` instead of the previous password in the
/// case of a successful password change. This function requires
/// [`admin_user`](CCashUser) to be equal to the admin user of the `CCash`
/// instance.
///
/// # Errors
///
/// Will return a `CCashError` if the request fails or if the `CCash` instance
/// returns an error code as long as the `interpret_endpoint_errors_as_false`
/// feature is disabled.
pub async fn change_password(
    session: &CCashSession,
    admin_user: &CCashUser,
    user: &mut CCashUser,
    new_password: &str,
) -> Result<bool, CCashError> {
    let url = format!("{}/v1/admin/user/change_password", &session.session_url);

    let new_user = CCashUser {
        username: user.username.clone(),
        password: new_password.into(),
    };

    let r = request(
        Method::PATCH,
        session,
        &url,
        Some(admin_user),
        Some(&new_user),
    )
    .await?;

    match r {
        CCashResponse::Success { .. } => {
            *user = new_user;
            Ok(true)
        },
        #[cfg(feature = "interpret_endpoint_errors_as_false")]
        CCashResponse::Error { .. } => Ok(false),
        #[cfg(not(feature = "interpret_endpoint_errors_as_false"))]
        CCashResponse::Error { .. } => Err(r.into()),
    }
}

/// Sets the balance of a user with the given `username` to the amount described
/// by `new_balance`. This function requires [`admin_user`](CCashUser) to be the
/// admin account for the `CCash` instance.
///
/// # Errors
///
/// Will return a `CCashError` if the request fails or if the `CCash` instance
/// returns an error code.
pub async fn set_balance(
    session: &CCashSession,
    admin_user: &CCashUser,
    username: &str,
    new_balance: u32,
) -> Result<(), CCashError> {
    #[derive(serde::Serialize)]
    struct SetBalanceData {
        name: String,
        amount: u32,
    }

    let url = format!("{}/v1/admin/set_balance", &session.session_url);

    let body = SetBalanceData {
        name: username.into(),
        amount: new_balance,
    };

    let r = request(Method::PATCH, session, &url, Some(admin_user), Some(&body)).await?;
    match r {
        CCashResponse::Success { .. } => Ok(()),
        CCashResponse::Error { .. } => Err(r.into()),
    }
}

/// Impacts the balance of user with the given `username` by the amount
/// described by `amount`. This function requires [`admin_user`](CCashUser) to
/// be the admin account for the `CCash` instance.
///
/// # Errors
///
/// Will return a `CCashError` if the request fails or if the `CCash` instance
/// returns an error code.
pub async fn impact_balance(
    session: &CCashSession,
    admin_user: &CCashUser,
    username: &str,
    amount: i64,
) -> Result<(), CCashError> {
    #[derive(serde::Serialize)]
    struct ImpactBalanceData {
        name: String,
        amount: i64,
    }

    let url = format!("{}/v1/admin/impact_balance", &session.session_url);

    let body = ImpactBalanceData {
        name: username.into(),
        amount,
    };

    let r = request(Method::POST, session, &url, Some(admin_user), Some(&body)).await?;
    match r {
        CCashResponse::Success { .. } => Ok(()),
        CCashResponse::Error { .. } => Err(r.into()),
    }
}

/// Adds a [`user`](CCashUser) to the `CCash` session described by
/// [`session`](CCashSession) with the balance determined by `amount`. This
/// function requires that [`admin_user`](CCashUser) to be the admin account.
///
/// # Errors
///
/// Will return `CCashError` if the request fails or if the `CCash` instance
/// returns an error code other than a 409 as long as the
/// `interpret_endpoint_errors_as_false` feature is disabled.
pub async fn add_user(
    session: &CCashSession,
    admin_user: &CCashUser,
    new_user: &CCashUser,
    amount: u32,
) -> Result<bool, CCashError> {
    #[derive(serde::Serialize)]
    struct AddUserData {
        #[serde(flatten)]
        user: CCashUser,
        amount: u32,
    }

    let url = format!("{}/v1/admin/user/register", &session.session_url);

    let body = AddUserData {
        user: new_user.clone(),
        amount,
    };

    let r = request(Method::POST, session, &url, Some(admin_user), Some(&body)).await?;
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

/// Removes a user associated with the `username` on the `CCash` instance
/// described by [`session`](CCashSession). This function requires that
/// [`admin_user`](CCashUser) to be the admin account for the `CCash` instance.
///
/// # Errors
///
/// Will return `CCashError` if request fails or if the `CCash` instance returns
/// an error code.
pub async fn delete_user(
    session: &CCashSession,
    admin_user: &CCashUser,
    username: &str,
) -> Result<(), CCashError> {
    let url = format!("{}/v1/admin/user/delete", &session.session_url);
    let body = hash_map! { "name": username };

    let r = request(Method::DELETE, session, &url, Some(admin_user), Some(&body)).await?;
    match r {
        CCashResponse::Success { .. } => Ok(()),
        CCashResponse::Error { .. } => Err(r.into()),
    }
}

/// Prunes users with less than `amount` in balance or users with transactions
/// older than `time` and less than `amount` in balance. This function requires
/// that [`admin_user`](CCashUser) be the admin user for the `CCash` instance.
///
/// # Errors
///
/// Will return `CCashError` if the request fails (could be down to
/// wrong/incorrect admin credientials) or if the `CCash` instance refuses to
/// prune it's users for another reason.
pub async fn prune_users(
    session: &CCashSession,
    admin_user: &CCashUser,
    amount: u32,
    time: Option<i64>,
) -> Result<u64, CCashError> {
    #[derive(serde::Serialize)]
    struct PruneUsersData {
        amount: u32,
        time: Option<i64>,
    }

    let url = format!("{}/v1/admin/prune_users", &session.session_url);

    let body = PruneUsersData { amount, time };

    let r = request(Method::POST, session, &url, Some(admin_user), Some(&body)).await?;
    match r {
        CCashResponse::Success { .. } =>
            if let Ok(amount) = r.convert_message::<u64>() {
                Ok(amount)
            } else {
                Err(CCashError::Error(
                    "Could not parse amount of users pruned into a valid u64".into(),
                ))
            },
        CCashResponse::Error { .. } => Err(r.into()),
    }
}

/// Saves and closes the `CCash` instance. This updates
/// [`session`](CCashSession) to reflect that the connection to the `CCash`
/// instance has closed. This function requires that [`admin_user`](CCashUser)
/// to be the admin user for the `CCash` instance.
///
/// # Errors
///
/// Will return `CCashError` if the request fails (could be down to
/// wrong/incorrect admin credientials) or if the `CCash` instance refuses to
/// close for another reason.
pub async fn close(
    session: &mut CCashSession,
    admin_user: &CCashUser,
) -> Result<(), CCashError> {
    let url = format!("{}/v1/admin/shutdown", &session.session_url);

    let r = request::<()>(Method::POST, session, &url, Some(admin_user), None).await?;
    match r {
        CCashResponse::Success { .. } => {
            session.is_connected = false;
            session.client = None;
            session.properties = None;

            Ok(())
        },
        CCashResponse::Error { .. } => Err(r.into()),
    }
}
