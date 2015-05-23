extern crate rustfm;

use rustfm::{LastFM, Artist, Event};

use std::env;

fn main() {
  let args : Vec<_> = env::args().collect();
  if args.len() == 1 {
    panic!("artist's name is required");
  }

  let last_fm = LastFM::new("572b13444704f89c67b07a713d5e5de1");
  let search  = Artist::events(last_fm, &args[1]);

  let events : Vec<Event> = search.results;
  for event in events.iter() {
    println!("{}\n", &event.to_string());
  }
}
