extern crate rustfm;

use rustfm::LastFM;
use rustfm::Artist;

#[test]
pub fn artist_info() {
  let last_fm = LastFM::new("572b13444704f89c67b07a713d5e5de1");
  let search  = Artist::info(last_fm, "nightwish");

  let mut results : Vec<Artist> = search.results;
  assert!(!results.is_empty());

  let members = results.pop().unwrap().members.unwrap();
  assert!(!members.member.is_empty())
}
