extern crate rustfm;

use rustfm::LastFM;
use rustfm::Artist;
use rustfm::Event;

#[test]
pub fn artist_events() {
  let last_fm = LastFM::new("572b13444704f89c67b07a713d5e5de1");
  let search  = Artist::events(last_fm, "nightwish");

  let events : Vec<Event> = search.results;
  assert!(!events.is_empty());
}
