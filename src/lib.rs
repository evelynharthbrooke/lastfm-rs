//! Welcome to lastfm_rs, a Rust library for interacting with the Last.fm API.
//!
//! This library aims to be easy to work with, as well as having complete support for the majority of the
//! Last.fm API endpoints. However, at this time, only a few API endpoints are supported by the library. These
//! include a few endpoints relating to retrieving user information, such as recent tracks, or general user
//! information retrieval. For all supported API endpoints, please check the Modules section; all API endpoints
//! are organized by data type and/or category as reflected by the [Last.fm API Documentation].
//!
//! However, it should be noted that the Scrobbling API is not planned to be implemented in the near future,
//! as music / media players written in Rust are fairly uncommon at this time, meaning support for scrobbling
//! is not currently a high priority for the library. This can always change however, but the main focus is
//! being as robust a library as possible for working with the heavily data-driven Last.fm API endpoints.
//!
//! The `error` and `model` modules are only used for error handling and models that are used across several
//! API endpoints, so they can be ignored, unless you intend on implementing error handling in your application,
//! in which case the error module should be looked at as the module has support for all of Last.fm's error types.
//!
//! # Installation
//!
//! lastfm_rs is very easy to install into a project; just add the following to the `[dependencies]`
//! section of your `Cargo.toml` file in your project's main directory.
//!
//! ```toml
//! lastfm_rs = "0.4"
//! ```
//!
//! [Last.fm API Documentation]: https://www.last.fm/api/intro/

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

use reqwest::{Client as ReqwestClient, Error, Response};
use std::marker::PhantomData;
use url::Url;

pub mod error;
pub mod macros;
pub mod model;
pub mod user;

/// The Request Builder.
///
/// This is the main request builder, used for constructing any and all requests to the Last.fm API.
///
/// * `client` - An instance of the Last.fm API client.
/// * `url` - The Last.fm API endpoint URL to feed to the request builder.
/// * `phantom` - An unused parameter, only used to satisfy the type checker.
pub struct RequestBuilder<'a, T: 'a> {
    client: &'a mut Client,
    url: Url,
    phantom: PhantomData<&'a T>,
}

/// The last.fm client.
///
/// The main client, used for interacting with the Last.fm API. This client is where you will use any
/// given API methods / calls, such as when you want to retrieve a user's recent tracks. All of the
/// available methods can be seen below.
///
/// * `api_key` - The API key used to authenticate to the Last.fm API.
/// * `client` - The given `reqwest` client. Used to send API requests.
pub struct Client {
    /// The API key used to authenticate with Last.fm.
    api_key: String,
    /// The `reqwest` client. Used to transmit and receive API requests and responses.
    client: ReqwestClient,
}

impl Client {
    /// Initializes a new Last.fm API client with a new `reqwest` client set to defaults.
    ///
    /// * `api_key` - The API key used to authenticate with the Last.fm API.
    pub fn new(api_key: &str) -> Client {
        Client {
            api_key: api_key.to_owned(),
            client: ReqwestClient::new(),
        }
    }

    /// Initializes a Last.fm API client from a pre-existing reqwest client. This is useful for when
    /// you already have a `reqwest` client already initialized and don't need a brand new client to
    /// be initialized each time.
    ///
    /// * `client` - The reqwest client to hook into.
    /// * `api_key` - The API key used to authenticate with the Last.fm API.
    pub fn from_reqwest_client(client: ReqwestClient, api_key: &str) -> Client {
        Client {
            api_key: api_key.to_owned(),
            client,
        }
    }

    /// Build a new URL with the given query parameters pointing to a given Last.fm API endpoint.
    async fn build_url(&self, params: Vec<(&str, &str)>) -> Url {
        let mut url = Url::parse("http://ws.audioscrobbler.com/2.0/").unwrap();

        url.query_pairs_mut().clear().append_pair("api_key", &*self.api_key).append_pair("format", "json");

        for (key, value) in params {
            url.query_pairs_mut().append_pair(key, value);
        }

        url
    }

    /// Send a GET request to the provided [`Url`].
    ///
    /// [`Url`]: url::Url
    async fn request(&mut self, url: &Url) -> Result<Response, Error> { self.client.get(url.as_str()).send().await }
}
