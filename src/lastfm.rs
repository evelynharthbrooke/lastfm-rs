use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

use rustc_serialize::json::Json;

pub struct LastFM<'a> {
  api_key: &'a str
}

impl<'a> LastFM<'a> {
  pub fn new(api_key: &'a str) -> LastFM {
    return LastFM { api_key: api_key };
  }

  pub fn request(&self, object: &str, method: &str, query: &str) -> Option<Json> {
    let mut client = Client::new();

    let url = format!(
      "http://ws.audioscrobbler.com/2.0/?method={object}.{}&{object}={}&api_key={}&format=json",
      method,
      query,
      self.api_key,
      object = object
    );

    let mut res = client.get(&url)
      .header(Connection::close())
      .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let json = &Json::from_str(&body).unwrap();
    return json.as_object().unwrap()
      .get("results").map(|r| r.clone());
  }
}
