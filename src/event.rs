use rustc_serialize::json::Json;
use rustc_serialize::json::Decoder as JsonDecoder;
use rustc_serialize::{Decoder, Decodable};

use ::Image;
use ::ImageCollection;
use ::Venue;

pub struct Event {
  pub id:          u32,
  pub title:       String,
  pub description: String,
  pub start_date:  String,
  pub attendance:  u32,
  pub reviews:     u32,
  pub url:         String,
  pub website:     String,
  pub images:      Vec<Image>,
  pub venue:       Venue
}

impl Decodable for Event {
  fn decode<D: Decoder>(decoder: &mut D) -> Result<Event, D::Error> {
    decoder.read_struct("root", 0, |decoder| {
      Ok(Event {
        id:          try!(decoder.read_struct_field("id",          0, Decodable::decode)),
        title:       try!(decoder.read_struct_field("title",       0, Decodable::decode)),
        description: try!(decoder.read_struct_field("description", 0, Decodable::decode)),
        start_date:  try!(decoder.read_struct_field("startDate",   0, Decodable::decode)),
        attendance:  try!(decoder.read_struct_field("attendance",  0, Decodable::decode)),
        reviews:     try!(decoder.read_struct_field("reviews",     0, Decodable::decode)),
        url:         try!(decoder.read_struct_field("url",         0, Decodable::decode)),
        website:     try!(decoder.read_struct_field("website",     0, Decodable::decode)),
        images:      try!(decoder.read_struct_field("image",       0, Decodable::decode)),
        venue:       try!(decoder.read_struct_field("venue",       0, Decodable::decode)),
      })
    })
  }
}

impl Event {
  pub fn from_json(event: Json) -> Event {
    let mut decoder           = JsonDecoder::new(event);
    let     event_obj : Event = match Decodable::decode(&mut decoder) {
      Ok(event) => event,
      Err(err)  => panic!(err)
    };

    return event_obj;
  }

  pub fn to_string(&self) -> String {
    return format!("Title: {}\nDescription: {}\nStart date: {}\nURL: {}\nWebsite: {}\nImages:\n{}\nVenue:\n{}",
      self.title,
      self.description,
      self.start_date,
      self.url,
      self.website,
      self.images.to_string(),
      self.venue.to_string()
    );
  }
}
