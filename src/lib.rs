#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate url;

#[macro_use] extern crate lazy_static;

use std::fmt;
use url::Url;
use hyper::client::Client as HTTPClient;

macro_rules! add_param {
    ($f:ident, $p:ident, $t:ty) => {
        pub fn $f(&'a mut self, v: $t) -> &'a mut Self {
            self.url.query_pairs_mut().append_pair(
                stringify!($p), &*v.to_string());
            self
        }
    }
}

pub mod user;

type HTTPResult = hyper::error::Result<hyper::client::response::Response>;

#[derive(Debug)]
pub enum Error {
    ParsingError(serde_json::error::Error),
    HTTPError(hyper::error::Error)
}

#[derive(Deserialize)]
pub struct RawData {
    #[serde(rename = "#text")]
    pub text: String
}

impl fmt::Debug for RawData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#""{}""#, self.text)
    }
}

pub struct Client {
    api_key:     String,
    http_client: HTTPClient
}

impl Client {
    pub fn new(api_key: &str) -> Client {
        Client {
            api_key:     api_key.to_owned(),
            http_client: HTTPClient::new()
        }
    }

    fn build_url(&self, params: Vec<(&str, &str)>) -> Url {
        let mut url = Url::parse("http://ws.audioscrobbler.com/2.0/").unwrap();

        url.query_pairs_mut().clear()
            .append_pair("api_key", &*self.api_key)
            .append_pair("format", "json");

        for (key, value) in params {
            url.query_pairs_mut().append_pair(key, value);
        }

        url
    }

    fn request(&mut self, url: &Url) -> HTTPResult {
        self.http_client.get(url.as_str()).send()
    }
}

#[cfg(test)]
mod tests {
    use super::Client;

    pub fn make_client() -> Client {
        Client::new("572b13444704f89c67b07a713d5e5de1")
    }

    #[test]
    fn test_build_url() {
        let client = make_client();
        let url    = client.build_url(vec![
                                      ("method", "user.getrecenttracks"),
                                      ("user",   "RoxasShadow")
        ]);

        assert_eq!(url.as_str(),
            "http://ws.audioscrobbler.com/2.0/?api_key=572b13444704f89c67b07a713d5e5de1&format=json&method=user.getrecenttracks&user=RoxasShadow");

        let url = client.build_url(vec![]);
        assert_eq!(url.as_str(),
            "http://ws.audioscrobbler.com/2.0/?api_key=572b13444704f89c67b07a713d5e5de1&format=json");
    }
}
