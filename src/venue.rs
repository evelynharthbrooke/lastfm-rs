use rustc_serialize::Decoder;

use ::Location;

#[derive(RustcDecodable)]
pub struct Venue {
  pub id:          Option<u32>,
  pub name:        Option<String>,
  pub url:         Option<String>,
  pub website:     Option<String>,
  pub phonenumber: Option<String>,
  pub location:    Option<Location>
}

impl Venue {
  pub fn to_string(&self) -> String {
    return format!("  Name: {}\n  Phone Number: {}\n  Location:\n{}",
      debug!(self.name),
      debug!(self.phonenumber),
      debug_s!(self.location)
    );
  }
}
