use rustc_serialize::json::Json;
use rustc_serialize::json::Decoder as JsonDecoder;
use rustc_serialize::{Decoder, Decodable};

use ::Image;
use ::Event;
use ::LastFM;
use ::SearchResults;

#[derive(Debug)]
pub struct Artist {
  pub name:      Option<String>,
  pub listeners: Option<u32>,
  pub mbid:      Option<String>,
  pub url:       Option<String>,
  pub images:    Option<Vec<Image>>,
  pub biography: Option<Biography>,
  pub stats:     Option<Stats>,
  pub members:   Option<Vec<BandMember>>/*,
  pub tags:      Option<Vec<Tag>>,
  pub similar:   Option<Vec<SuggestedArtist>>*/
}

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct SuggestedArtist {
  pub name:      Option<String>,
  pub url:       Option<String>,
  pub images:    Option<Vec<Image>>,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct BandMemberContainer {
  pub member: Vec<BandMember>
}

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct BandMember {
  pub name:     Option<String>,
  pub yearfrom: Option<u32>,
  pub yearto:   Option<u32>,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct Biography {
  pub published:   Option<String>,
  pub content:     Option<String>,
  pub placeformed: Option<String>,
  pub yearformed:  Option<String>
}

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct Stats {
  pub listeners:   Option<u32>,
  pub playcount:   Option<u32>
}

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct Tag {
  pub name: Option<String>,
  pub url:  Option<String>
}

impl Decodable for Artist {
  fn decode<D: Decoder>(decoder: &mut D) -> Result<Artist, D::Error> {
    decoder.read_struct("root", 0, |decoder| {
      Ok(Artist {
        name:      to_option!(decoder.read_struct_field("name",        0, Decodable::decode)),
        listeners: to_option!(decoder.read_struct_field("listeners",   0, Decodable::decode)),
        mbid:      to_option!(decoder.read_struct_field("mbid",        0, Decodable::decode)),
        url:       to_option!(decoder.read_struct_field("url",         0, Decodable::decode)),
        images:    to_option!(decoder.read_struct_field("image",       0, Decodable::decode)),
        biography: to_option!(decoder.read_struct_field("bio",         0, Decodable::decode)),
        stats:     to_option!(decoder.read_struct_field("stats",       0, Decodable::decode)),
        members:   to_option!(decoder.read_struct_field("bandmembers", 0, |decoder| {
          decoder.read_struct_field("member", 0, Decodable::decode)
        }))
        /* tags:      to_option!(decoder.read_struct_field("tags",    0, |decoder| {
          decoder.read_struct_field("tag", 0, Decodable::decode)
        })),
        similar:   to_option!(decoder.read_struct_field("similar", 0, |decoder| {
          decoder.read_struct_field("artist", 0, Decodable::decode)
        }))*/
      })
    })
  }
}

impl<'a> Artist {
  pub fn from_json(artist: Json) -> Artist {
    let mut decoder             = JsonDecoder::new(artist);
    let     artist_obj : Artist = match Decodable::decode(&mut decoder) {
      Ok(artist) => artist,
      Err(err)   => panic!(err)
    };

    return artist_obj;
  }

  pub fn info(lastfm: LastFM, query: &'a str) -> SearchResults<'a, Artist> {
    let response = lastfm.request("artist", "getinfo", &query).unwrap();

    let     artist                = response.get("artist").unwrap();
    let mut artists : Vec<Artist> = Vec::new();
    artists.push(Artist::from_json(artist.clone()));

    return SearchResults {
      query:   &query,
      results: artists
    };
  }

  pub fn search(lastfm: LastFM, query: &'a str) -> SearchResults<'a, Artist> {
    let response     = lastfm.request("artist", "search", &query).unwrap();
    let response_obj = response.get("results").unwrap()
      .as_object().unwrap();

    let artists = match response_obj.get("artistmatches").unwrap().as_object() {
      Some(artist_matches) => artist_matches.get("artist").unwrap().as_array().unwrap(),
      None                 => panic!("No results :(")
    };

    return SearchResults {
      query:   &query,
      results: artists.iter().map(|a|
        Artist::from_json(a.clone())
      ).collect()
    };
  }

  pub fn events(lastfm: LastFM, query: &'a str) -> SearchResults<'a, Event> {
    let response = lastfm.request("artist", "getevents", &query).unwrap();

    let events = match response.get("events").unwrap().as_object() {
      Some(event_matches) => event_matches.get("event").unwrap().as_array().unwrap(),
      None                => panic!("No results :(")
    };

    return SearchResults {
      query:   &query,
      results: events.iter().map(|a|
        Event::from_json(a.clone())
      ).collect()
    };
  }
}
