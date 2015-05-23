use rustc_serialize::Decoder;

use ::Location;

#[derive(RustcDecodable)]
pub struct Venue {
  pub id:          u32,
  pub name:        String,
  pub url:         String,
  pub website:     String,
  pub phonenumber: String,
  pub location:    Location
}

impl Venue {
  pub fn to_string(&self) -> String {
    return format!("  Name: {}\n  Phone Number: {}\n  Location:\n{}", self.name, self.phonenumber, self.location.to_string());
  }
}
