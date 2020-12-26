use serde::Deserialize;
use std::marker::PhantomData;

use crate::{
    error::{Error, LastFMError},
    model::{Attributes, Image, TrackDate},
    user::User,
    Client, RequestBuilder,
};

/// The main loved tracks structure.
///
/// This is splitted off into two areas: One, the attributes (used
/// for displaying various user-associated attributes), and two,
/// a specific number of tracks the user has marked as loved on Last.fm.
///
/// For details on the attributes available, refer to [Attributes]. For
/// details on the track information available, refer to [Track].
#[derive(Debug, Deserialize)]
pub struct LovedTracks {
    /// The attributes associated with the user's Loved Tracks
    /// listing.
    #[serde(rename = "@attr")]
    pub attrs: Attributes,
    /// A vector array containing the tracks the user has loved
    /// on Last.fm.
    #[serde(rename = "track")]
    pub tracks: Vec<Track>,
}

/// Contains information about the given track that the
/// user Loved.
#[derive(Debug, Deserialize)]
pub struct Track {
    /// The artist who published the given track.
    pub artist: Artist,
    /// The MusicBrainz ID for the given track.
    pub mbid: String,
    /// The date of when the user loved the track.
    pub date: Option<TrackDate>,
    /// The name of the track.
    pub name: String,
    /// The Last.fm URL of the track.
    pub url: String,
    /// The cover art for the given track. Available in small, medium,
    /// and large sizes.
    #[serde(rename = "image")]
    pub images: Vec<Image>,
    /// Whether or not the track is streamable.
    pub streamable: Streamable,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    /// The Last.fm URL for the given artist.
    pub url: String,
    /// The name of the given artist.
    pub name: String,
    /// The MusicBrainz ID of the given artist.
    pub mbid: String,
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

impl LovedTracks {
    pub async fn build<'a>(client: &'a mut Client, user: &str) -> RequestBuilder<'a, LovedTracks> {
        let url = client.build_url(vec![("method", "user.getLovedTracks"), ("user", user)]).await;
        RequestBuilder { client, url, phantom: PhantomData }
    }
}

impl<'a> RequestBuilder<'a, LovedTracks> {
    add_param!(with_limit, limit, usize);
    add_param!(with_page, page, usize);

    pub async fn send(&'a mut self) -> Result<LovedTracks, Error> {
        match self.client.request(&self.url).await {
            Ok(response) => {
                let body = response.text().await.unwrap();
                match serde_json::from_str::<LastFMError>(&*body) {
                    Ok(lastm_error) => Err(Error::LastFMError(lastm_error.into())),
                    Err(_) => match serde_json::from_str::<User>(&*body) {
                        Ok(user) => Ok(user.loved_tracks.unwrap()),
                        Err(e) => Err(Error::ParsingError(e)),
                    },
                }
            }
            Err(err) => Err(Error::HTTPError(err)),
        }
    }
}

impl<'a> Client {
    pub async fn loved_tracks(&'a mut self, user: &str) -> RequestBuilder<'a, LovedTracks> { LovedTracks::build(self, user).await }
}
