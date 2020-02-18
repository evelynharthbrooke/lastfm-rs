use serde::Deserialize;

pub mod loved_tracks;
pub mod recent_tracks;
pub mod top_artists;
pub mod user_info;

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "lovedtracks")]
    pub loved_tracks: Option<loved_tracks::LovedTracks>,
    #[serde(rename = "recenttracks")]
    pub recent_tracks: Option<recent_tracks::RecentTracks>,
    #[serde(rename = "topartists")]
    pub top_artists: Option<top_artists::TopArtists>,
    #[serde(rename = "user")]
    pub user_info: Option<user_info::UserInfo>,
}
