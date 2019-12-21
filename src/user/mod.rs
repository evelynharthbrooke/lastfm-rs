pub mod recent_tracks;
pub mod loved_tracks;

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "recenttracks")]
    pub recent_tracks: Option<recent_tracks::RecentTracks>,
    #[serde(rename = "lovedtracks")]
    pub loved_tracks: Option<loved_tracks::LovedTracks>
}
