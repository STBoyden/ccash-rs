#![warn(missing_docs, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, deprecated)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod request;
pub mod methods;
pub mod responses;
pub mod user;

pub use crate::{responses::*, user::*};
use chrono::prelude::*;
use reqwest::Client;
use serde::Deserialize;
use std::fmt;

/// Struct that decribes the properties of the `CCash` instance that are
/// returned from the `properties` endpoint. Helps define the behaviour of this
/// API.
#[derive(Clone, Debug, Deserialize)]
pub struct CCashSessionProperties {
    pub(crate) version: Option<u32>,
    pub(crate) max_log: u32,
    pub(crate) add_user_open: Option<bool>,
    pub(crate) return_on_del: Option<String>,
}

impl CCashSessionProperties {
    /// Returns the version of the `CCash` instance.
    #[must_use]
    pub fn get_version(&self) -> Option<u32> { self.version }

    /// Returns the max amount of logs that can be returned by the `CCash`
    /// instance.
    #[must_use]
    pub fn get_max_log(&self) -> u32 { self.max_log }

    /// Returns whether or not any user can register without the need of a
    /// pre-existing admin account
    #[must_use]
    pub fn get_add_user_is_open(&self) -> bool {
        if let Some(b) = self.add_user_open {
            b
        } else {
            false
        }
    }

    /// Returns the account that funds are returned to when an account with
    /// funds is deleted from the `CCash` instance. This field is optional
    /// as this is an option chosen by the host to include at compile-time,
    /// and may not always be present in the API properties returned by
    /// `CCash` on all instances.
    #[must_use]
    pub fn get_return_on_delete_account(&self) -> &Option<String> { &self.return_on_del }
}

/// Struct that describes the format of the logs returned by
/// [`get_log`](`methods::get_log`).
#[derive(Debug, Deserialize)]
#[deprecated(since = "2.0.0", note = "Prefer the usage of `TransactionLogV2`")]
pub struct TransactionLog {
    pub(crate) to: String,
    pub(crate) from: String,
    pub(crate) amount: u32,
    pub(crate) time: i64,
}

impl TransactionLog {
    /// Returns the account to which the funds were sent to.
    #[must_use]
    pub fn get_to_account(&self) -> &str { &self.to }

    /// Returns the account from which the funds were sent from.
    #[must_use]
    pub fn get_from_account(&self) -> &str { &self.from }

    /// Returns the amount of CSH that was sent.
    #[must_use]
    pub fn get_amount(&self) -> u32 { self.amount }

    /// Returns the time that the funds were sent in Unix epoch time.
    #[must_use]
    pub fn get_time(&self) -> i64 { self.time }
}

impl fmt::Display for TransactionLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(self.time, 0).unwrap(),
            Utc,
        );
        write!(
            f,
            "{}: {} ({} CSH) -> {}",
            time, &self.from, self.amount, &self.to,
        )
    }
}

/// Struct that describes the format of the logs returned by
/// [`get_log_v2`](`methods::get_log_v2`).
#[derive(Debug, Deserialize)]
pub struct TransactionLogV2 {
    pub(crate) counterparty: String,
    pub(crate) receiving: bool,
    pub(crate) amount: u32,
    pub(crate) time: i64,
}

impl TransactionLogV2 {
    /// Returns the name of the account where the funds were sent or received
    /// from.
    #[must_use]
    pub fn get_counterparty(&self) -> &str { &self.counterparty }

    /// Returns if the current [`CCashUser`] is sending or receiving funds in
    /// this transaction.
    #[must_use]
    pub fn get_if_receiving(&self) -> bool { self.receiving }

    /// Returns the amount of funds in CSH.
    #[must_use]
    pub fn get_amount(&self) -> u32 { self.amount }

    /// Returns the time of the transaction in Unix epoch time.
    #[must_use]
    pub fn get_time(&self) -> i64 { self.time }
}

impl fmt::Display for TransactionLogV2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let action = if self.receiving { "Received" } else { "Sent" };
        let tofrom = if self.receiving { "from" } else { "to" };
        let time = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(self.time, 0).unwrap(),
            Utc,
        );

        write!(
            f,
            "{time}: {action} {} CSH {tofrom} {}",
            self.amount, self.counterparty
        )
    }
}

/// Struct that describes the connection to the `CCash` API instance which is
/// defined by the `session_url`.
///
/// # Usage
/// The intended usage for this struct is to provide a simple way to connect to
/// the `CCash` instance and be passed into the functions provided by
/// [`methods`] and [`methods::admin`]. This also means multiple `CCashSession`s
/// can be connected to different `CCash` instances, if need be.
///
/// An example usage is as follows
/// (available [here](https://github.com/STBoyden/ccash-rs/src/branch/master/examples/get_balance.rs)):
/// ```
#[doc = include_str!("../examples/get_balance.rs")]
/// ```
/// 
/// Before any function from [`methods`] and [`methods::admin`] is called,
/// [`establish_connection`](CCashSession::establish_connection) must be called
/// to make sure that the connection to the `CCash` instance is secured and
/// correct. This also makes sure that the properties of `CCashSession` is
/// properly set and not `None`.
#[derive(Debug, Clone)]
pub struct CCashSession {
    session_url: String,
    is_connected: bool,
    client: Option<Client>,
    properties: Option<CCashSessionProperties>,
}

impl CCashSession {
    /// Constructs a new `CCashSession` from a `base_url`
    #[must_use]
    pub fn new(base_url: &str) -> CCashSession {
        let base_url = if base_url.ends_with('/') {
            base_url.trim_end_matches('/')
        } else {
            base_url
        };

        Self {
            session_url: format!("{base_url}/api"),
            is_connected: false,
            client: None,
            properties: None,
        }
    }

    /// Establishes a connection to the `CCash` instance using the
    /// `session_url`.
    ///
    /// # Errors
    ///
    /// Will return `CCashError::CouldNotParsePropertiesResponse` if the
    /// properties returned by `CCash` could not be parsed correctly.
    pub async fn establish_connection(&mut self) -> Result<(), CCashError> {
        if self.is_connected {
            return Ok(());
        }

        let client = Client::builder().build()?;

        let request = client
            .get(format!("{}/properties", self.session_url))
            .header("Accept", "application/json")
            .build()?;

        let response = client.execute(request).await?;

        if let Ok(v) = response.json::<CCashSessionProperties>().await {
            self.properties = Some(v);
            self.is_connected = true;
            self.client = Some(client);
            Ok(())
        } else {
            Err(CCashError::CouldNotParsePropertiesResponse)
        }
    }

    /// Gets the client associated with this instance of `CCashSession`
    #[must_use]
    pub fn get_client(&self) -> &Option<Client> { &self.client }
    /// Returns whether or not the `CCashSession` is connectd to the instance.
    #[must_use]
    pub fn is_connected(&self) -> bool { self.is_connected }
    /// Returns the properties of the `CCash` instance.
    #[must_use]
    pub fn get_properties(&self) -> &Option<CCashSessionProperties> { &self.properties }
}
