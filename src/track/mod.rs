//! Last.fm Track API Endpoints
//!
//! Contains structs and methods related to working with the track-related endpoints
//! available through the Last.fm API

use serde::Deserialize;

pub mod similar;

#[derive(Debug, Deserialize)]
pub struct Track {
    #[serde(rename = "similartracks")]
    pub similar_tracks: Option<similar::Similar>,
}
