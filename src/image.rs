use rustc_serialize::{Decoder, Decodable};

pub struct Image {
  pub size: Option<String>,
  pub url:  Option<String>
}

impl Decodable for Image {
  fn decode<D: Decoder>(decoder: &mut D) -> Result<Image, D::Error> {
    decoder.read_struct("root", 0, |decoder| {
      Ok(Image {
        size: to_option!(decoder.read_struct_field("size",  0, Decodable::decode)),
        url: to_option!(decoder.read_struct_field("#text", 0, Decodable::decode))
      })
    })
  }
}

impl Image {
  pub fn to_string(&self) -> String {
    return format!("{}: {}", debug!(self.size), debug!(self.url));
  }
}
