use rustc_serialize::Decoder;

#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct Location {
  pub city:       Option<String>,
  pub country:    Option<String>,
  pub street:     Option<String>,
  pub postalcode: Option<String>,
}
