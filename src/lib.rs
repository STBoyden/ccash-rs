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
    /// Returns the version of the `CCash` instance.
    pub version: u32,
    /// The max amount of logs that can be returned by the `CCash` instance.
    pub max_log: u32,
    /// The account that funds are returned to when an account with funds is
    /// deleted from the `CCash` instance. This field is optional as this is
    /// an option chosen by the host to include at compile-time, and may not
    /// always be present in the API properties returned by `CCash` on all
    /// instances.
    pub return_on_del: Option<String>,
}

/// Struct that describes the format of the logs returned by
/// [`get_log`](`methods::get_log`).
#[derive(Debug, Deserialize)]
#[deprecated(since = "2.0.0", note = "Prefer the usage of `TransactionLogV2`")]
pub struct TransactionLog {
    /// The account to which the funds were sent to.
    pub to: String,
    /// The account from which the funds were sent from.
    pub from: String,
    /// The amount of CSH that was sent.
    pub amount: u32,
    /// The time that the funds were sent in Unix epoch time.
    pub time: i64,
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
    /// The name of the account where the funds were sent or received from.
    pub counterparty: String,
    /// If the current [`CCashUser`] is sending or receiving funds.
    pub receiving: bool,
    /// The amount of funds in CSH.
    pub amount: u32,
    /// the time of the transaction in Unix epoch time.
    pub time: i64,
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
/// (available [here](https://git.stboyden.com/STBoyden/ccash-rs/src/branch/master/examples/get_balance.rs)):
/// ```
#[doc = include_str!("../examples/get_balance.rs")]
/// }
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
            self.properties = Some(v.clone());
            self.session_url = format!("{}/v{}", self.session_url, v.version);

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
