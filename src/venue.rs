use rustc_serialize::Decoder;

use ::Location;

#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct Venue {
  pub id:          Option<u32>,
  pub name:        Option<String>,
  pub url:         Option<String>,
  pub website:     Option<String>,
  pub phonenumber: Option<String>,
  pub location:    Option<Location>
}
