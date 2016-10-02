pub mod recent_tracks;

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "recenttracks")]
    pub recent_tracks: Option<recent_tracks::RecentTracks>
}
