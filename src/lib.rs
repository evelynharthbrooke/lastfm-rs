#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

use hyper::client::Client as HTTPClient;
use std::fmt;
use std::marker::PhantomData;
use url::Url;

macro_rules! add_param {
    ($f:ident, $p:ident, $t:ty) => {
        pub fn $f(&'a mut self, v: $t) -> &'a mut Self {
            self.url.query_pairs_mut().append_pair(
                stringify!($p), &*v.to_string());
            self
        }
    }
}

pub mod error;
pub mod user;

type HTTPResult = hyper::error::Result<hyper::client::response::Response>;

/// Generic LastFM object.
#[derive(Deserialize)]
pub struct RawData {
    #[serde(rename = "#text")]
    pub text: String,
}

impl fmt::Debug for RawData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#""{}""#, self.text)
    }
}

impl fmt::Display for RawData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

pub struct RequestBuilder<'a, T: 'a> {
    client: &'a mut Client,
    url: Url,
    phantom: PhantomData<&'a T>,
}

pub struct Client {
    api_key: String,
    http_client: HTTPClient,
}

impl Client {
    pub fn new(api_key: &str) -> Client {
        Client {
            api_key: api_key.to_owned(),
            http_client: HTTPClient::new(),
        }
    }

    /// Build a new URL with given query params pointing to the LastFM APIs.
    fn build_url(&self, params: Vec<(&str, &str)>) -> Url {
        let mut url = Url::parse("http://ws.audioscrobbler.com/2.0/").unwrap();

        url.query_pairs_mut()
            .clear()
            .append_pair("api_key", &*self.api_key)
            .append_pair("format", "json");

        for (key, value) in params {
            url.query_pairs_mut().append_pair(key, value);
        }

        url
    }

    /// Send a GET request to given `Url`.
    fn request(&mut self, url: &Url) -> HTTPResult {
        self.http_client.get(url.as_str()).send()
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use std::env;

    pub fn make_client() -> Client {
        let api_key = env::var("API_KEY").unwrap();
        Client::new(&api_key)
    }

    #[test]
    fn test_build_url() {
        let api_key = env::var("API_KEY").unwrap();
        let user = env::var("USER").unwrap();

        let client = make_client();
        let url = client.build_url(vec![("method", "user.getrecenttracks"), ("user", &user)]);

        assert_eq!(url.as_str(),
            &format!("http://ws.audioscrobbler.com/2.0/?api_key={}&format=json&method=user.getrecenttracks&user={}", api_key, user));

        let url = client.build_url(vec![]);
        assert_eq!(
            url.as_str(),
            &format!(
                "http://ws.audioscrobbler.com/2.0/?api_key={}&format=json",
                api_key
            )
        );
    }
}
