extern crate hyper;
extern crate rustc_serialize;

pub mod image;
pub mod artist;
pub mod search_results;
pub mod lastfm;

pub use artist::Artist;
pub use image::Image;
pub use search_results::SearchResults;
pub use lastfm::LastFM;
