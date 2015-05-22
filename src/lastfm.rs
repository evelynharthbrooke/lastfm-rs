use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

use rustc_serialize::json::Json;

use ::SearchResults;
use ::Artist;

pub struct LastFM<'a> {
  api_key: &'a str
}

impl<'a> LastFM<'a> {
  pub fn new(api_key: &'a str) -> LastFM {
    return LastFM { api_key: api_key };
  }

  fn request(&self, query: &str) -> Option<Json> {
    let mut client = Client::new();

    let url = format!(
      "http://ws.audioscrobbler.com/2.0/?method=artist.search&artist={}&api_key={}&format=json",
      query,
      self.api_key
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

  pub fn search_artists(&self, query: &'a str) -> SearchResults<Artist> {
    let response = self.request(query).unwrap();
    let response_obj = response.as_object().unwrap();

    let artists = match response_obj.get("artistmatches").unwrap().as_object() {
      Some(artist_matches) => artist_matches.get("artist").unwrap().as_array().unwrap(),
      None => panic!("No results :(")
    };

    return SearchResults {
      query:   query,
      results: artists.iter().map(|a|
        Artist::from_json(a.clone())
      ).collect()
    };
  }
}
