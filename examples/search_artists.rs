extern crate rustfm;

use rustfm::LastFM;
use rustfm::Artist;

use std::env;

fn main() {
  let args : Vec<_> = env::args().collect();
  if args.len() == 1 {
    panic!("artist's name is required");
  }

  let last_fm = LastFM::new("572b13444704f89c67b07a713d5e5de1");
  let search  = last_fm.search_artists(&args[1]);

  let artists : Vec<Artist> = search.results;
  for artist in artists.iter() {
    println!("{}\n", &artist.to_string());
  }
}
