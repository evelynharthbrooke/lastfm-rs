use rustc_serialize::Decoder;

#[derive(RustcDecodable)]
pub struct Location {
  pub city:       String,
  pub country:    String,
  pub street:     String,
  pub postalcode: String
}

impl Location {
  pub fn to_string(&self) -> String {
    return format!("    City: {} ({})\n    Address: {} - {}", self.city, self.country, self.street, self.postalcode);
  }
}
