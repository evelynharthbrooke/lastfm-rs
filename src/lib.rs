extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

use reqwest::Client as ReqwestClient;
use reqwest::Response;
use reqwest::Error;

use std::marker::PhantomData;

use url::Url;

macro_rules! add_param {
    ($f:ident, $p:ident, $t:ty) => {
        pub fn $f(&'a mut self, v: $t) -> &'a mut Self {
            self.url.query_pairs_mut().append_pair(stringify!($p), &*v.to_string());
            self
        }
    };
}

pub mod error;
pub mod user;

pub struct RequestBuilder<'a, T: 'a> {
    client: &'a mut Client,
    url: Url,
    phantom: PhantomData<&'a T>,
}

pub struct Client {
    api_key: String,
    client: ReqwestClient,
}

impl Client {
    pub fn new(api_key: &str) -> Client {
        Client {
            api_key: api_key.to_owned(),
            client: ReqwestClient::new(),
        }
    }

    pub fn from_reqwest_client(client: ReqwestClient, api_key: &str) -> Client {
        Client {
            api_key: api_key.to_owned(),
            client,
        }
    }

    /// Build a new URL with given query params pointing to the LastFM APIs.
    async fn build_url(&self, params: Vec<(&str, &str)>) -> Url {
        let mut url = Url::parse("http://ws.audioscrobbler.com/2.0/").unwrap();

        url.query_pairs_mut().clear().append_pair("api_key", &*self.api_key).append_pair("format", "json");

        for (key, value) in params {
            url.query_pairs_mut().append_pair(key, value);
        }

        url
    }

    /// Send a GET request to given `Url`.
    async fn request(&mut self, url: &Url) -> Result<Response, Error> {
        self.client.get(url.as_str()).send().await
    }
}
