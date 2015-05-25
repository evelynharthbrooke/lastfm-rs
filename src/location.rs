use rustc_serialize::Decoder;

#[derive(RustcDecodable)]
pub struct Location {
  pub city:       Option<String>,
  pub country:    Option<String>,
  pub street:     Option<String>,
  pub postalcode: Option<String>,
}

impl Location {
  pub fn to_string(&self) -> String {
    return format!("    City: {} ({})\n    Address: {} - {}",
      debug!(self.city),
      debug!(self.country),
      debug!(self.street),
      debug!(self.postalcode)
    );
  }
}
