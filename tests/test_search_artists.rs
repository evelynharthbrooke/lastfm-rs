extern crate rustfm;

use rustfm::LastFM;
use rustfm::Artist;

#[test]
pub fn search_artists() {
  let last_fm = LastFM::new("572b13444704f89c67b07a713d5e5de1");
  let search  = Artist::search(last_fm, "nightwish");

  let artists : Vec<Artist> = search.results;
  assert!(!artists.is_empty());
}
