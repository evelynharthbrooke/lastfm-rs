use rustc_serialize::json::Json;
use rustc_serialize::json::Decoder as JsonDecoder;
use rustc_serialize::{Decoder, Decodable};

use ::Image;

pub struct Artist {
  pub name:      String,
  pub listeners: u32,
  pub mbid:      String,
  pub url:       String,
  pub images:    Vec<Image>
}

impl Decodable for Artist {
  fn decode<D: Decoder>(decoder: &mut D) -> Result<Artist, D::Error> {
    decoder.read_struct("root", 0, |decoder| {
      Ok(Artist {
        name:      try!(decoder.read_struct_field("name",      0, |decoder| Decodable::decode(decoder))),
        listeners: try!(decoder.read_struct_field("listeners", 0, |decoder| Decodable::decode(decoder))),
        mbid:      try!(decoder.read_struct_field("mbid",      0, |decoder| Decodable::decode(decoder))),
        url:       try!(decoder.read_struct_field("url",       0, |decoder| Decodable::decode(decoder))),
        images:    try!(decoder.read_struct_field("image",     0, |decoder| Decodable::decode(decoder))),
      })
    })
  }
}

impl Artist {
  pub fn from_json(artist: Json) -> Artist {
    let mut decoder = JsonDecoder::new(artist);
    let artist_obj : Artist = match Decodable::decode(&mut decoder) {
      Ok(artist) => artist,
      Err(err)   => panic!(err)
    };

    return artist_obj;
  }

  pub fn to_string(&self) -> String {
    return format!("Name: {}\nListeners: {}\nURL: {}\nImages: \n{}",
      self.name,
      self.listeners,
      self.url,
      self.images.iter()
        .filter(|i| !i.url.is_empty())
        .map(|i| format!("  {}", i.to_string()))
        .collect::<Vec<String>>().connect("\n")
    );
  }
}
