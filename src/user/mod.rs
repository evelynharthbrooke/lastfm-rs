use serde::Deserialize;

pub mod recent_tracks;
pub mod loved_tracks;
pub mod user_info;

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "recenttracks")]
    pub recent_tracks: Option<recent_tracks::RecentTracks>,
    #[serde(rename = "lovedtracks")]
    pub loved_tracks: Option<loved_tracks::LovedTracks>,
    #[serde(rename = "user")]
    pub user_info: Option<user_info::UserInfo>
}
