use rustc_serialize::{Decoder, Decodable};

pub struct Image {
  pub size: String,
  pub url:  String
}

impl Decodable for Image {
  fn decode<D: Decoder>(decoder: &mut D) -> Result<Image, D::Error> {
    decoder.read_struct("root", 0, |decoder| {
      let url : String = try!(decoder.read_struct_field("#text", 0, |decoder| Decodable::decode(decoder)));

      Ok(Image {
        size: try!(decoder.read_struct_field("size",  0, |decoder| Decodable::decode(decoder))),
        url:  url
      })
    })
  }
}

impl Image {
  pub fn to_string(&self) -> String {
    return format!("{}: {}", self.size, self.url);
  }
}
