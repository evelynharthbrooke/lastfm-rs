use rustc_serialize::json::Json;
use rustc_serialize::json::Decoder as JsonDecoder;
use rustc_serialize::{Decoder, Decodable};

use ::Image;
use ::Venue;

#[derive(Debug)]
pub struct Event {
  pub id:          Option<u32>,
  pub title:       Option<String>,
  pub description: Option<String>,
  pub start_date:  Option<String>,
  pub attendance:  Option<u32>,
  pub reviews:     Option<u32>,
  pub url:         Option<String>,
  pub website:     Option<String>,
  pub images:      Option<Vec<Image>>,
  pub venue:       Option<Venue>
}

impl Decodable for Event {
  fn decode<D: Decoder>(decoder: &mut D) -> Result<Event, D::Error> {
    decoder.read_struct("root", 0, |decoder| {
      Ok(Event {
        id:          to_option!(decoder.read_struct_field("id",          0, Decodable::decode)),
        title:       to_option!(decoder.read_struct_field("title",       0, Decodable::decode)),
        description: to_option!(decoder.read_struct_field("description", 0, Decodable::decode)),
        start_date:  to_option!(decoder.read_struct_field("startDate",   0, Decodable::decode)),
        attendance:  to_option!(decoder.read_struct_field("attendance",  0, Decodable::decode)),
        reviews:     to_option!(decoder.read_struct_field("reviews",     0, Decodable::decode)),
        url:         to_option!(decoder.read_struct_field("url",         0, Decodable::decode)),
        website:     to_option!(decoder.read_struct_field("website",     0, Decodable::decode)),
        images:      to_option!(decoder.read_struct_field("image",       0, Decodable::decode)),
        venue:       to_option!(decoder.read_struct_field("venue",       0, Decodable::decode)),
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
}
