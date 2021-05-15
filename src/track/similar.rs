use serde::Deserialize;
use std::marker::PhantomData;

use crate::{
    error::{Error, LastFMError},
    model::{Attributes, Image},
    track::{Track as OriginalTrack},
    Client, RequestBuilder,
};

/// The main similar structure.
///
/// This is split off into two areas: One, the attributes (used
/// for displaying various user-associated attributes), and two,
/// the similar tracks to the one provided.
///
/// For details on the attributes available, refer to [Attributes]. For
/// details on the track information available, refer to [Track].
#[derive(Debug, Deserialize)]
pub struct Similar {
    /// A [Vec] containing similar [Track]s.
    #[serde(rename = "track")]
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    /// The name of the given track
    pub name: String,
    /// The MusicBrainz ID for the given track.
    pub mbid: Option<String>,
    /// The match for the given track
    pub r#match: f32,
    /// The Last.fm URL of the track
    pub url: String,
    /// Whether or not the track is streamable
    pub streamable: Streamable,
    /// the artist who published the given track
    pub artist: Artist,
    /// The cover art for the given track. Available in small, medium,
    /// and large sizes.
    #[serde(rename="image")]
    pub images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    /// The name of the given artist
    pub name: String,
    /// The MusicBrainz ID of the given artist
    pub mbid: Option<String>,
    /// The Last.fm URL for the given artist
    pub url: String,
}

/// The streamable struct.
///
/// Available if the given track is available for streaming.
#[derive(Debug, Deserialize)]
pub struct Streamable {
    pub fulltrack: String,
    #[serde(rename = "#text")]
    pub text: String,
}

impl Similar {
    pub async fn build_by_mbid<'a>(client: &'a mut Client, mbid: &str) -> RequestBuilder<'a, Similar> {
        let url = client.build_url(vec![("method", "track.getSimilar"), ("mbid", mbid)]).await;
        RequestBuilder { client, url, phantom: PhantomData }
    }

    pub async fn build<'a>(client: &'a mut Client, artist: &str, track: &str) -> RequestBuilder<'a, Similar> {
        let url = client.build_url(vec![("method", "track.getSimilar"), ("artist", artist), ("track", track)]).await;
        RequestBuilder { client, url, phantom: PhantomData }
    }
}

impl<'a> RequestBuilder<'a, Similar> {
    add_param!(with_limit, limit, usize);

    pub async fn send(&'a mut self) -> Result<Similar, Error> {
        match self.client.request(&self.url).await {
            Ok(response) => {
                let body = response.text().await.unwrap();
                match serde_json::from_str::<LastFMError>(&body) {
                    Ok(lastfm_error) => Err(Error::LastFMError(lastfm_error.into())),
                    Err(_) => match serde_json::from_str::<OriginalTrack>(&body) {
                        Ok(tracks) => Ok(tracks.similar_tracks.unwrap()),
                        Err(e) => Err(Error::ParsingError(e)),
                    },
                }
            }
            Err(err) => Err(Error::HTTPError(err)),
        }
    }
}

impl<'a> Client {
    pub async fn similar_tracks_by_mbid(&'a mut self, mbid: &str) -> RequestBuilder<'a, Similar> { Similar::build_by_mbid(self, mbid).await }

    pub async fn similar_tracks(&'a mut self, artist: &str, track: &str) -> RequestBuilder<'a, Similar> { Similar::build(self, artist, track).await }
}
