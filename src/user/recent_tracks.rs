use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::marker::PhantomData;

use crate::{
    error::{Error, LastFMError},
    model::{Attributes, Image},
    user::User,
    util::deserialize_datetime_from_str,
    Client, RequestBuilder,
};

/// The main recent tracks structure.
///
/// This is splitted off into two areas: One, the attributes (used
/// for displaying various user-associated attributes), and two,
/// the recent tracks the user has played.
///
/// For details on the attributes available, refer to [Attributes]. For
/// details on the track information available, refer to [Track].
#[derive(Debug, Deserialize)]
pub struct RecentTracks {
    /// Various user attributes.
    #[serde(rename = "@attr")]
    pub attrs: Attributes,
    /// A [Vec] containiing recent [Track]s.
    #[serde(rename = "track")]
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    /// The primary artist associated with the track.
    pub artist: Artist,
    /// Various attributes associated with the track.
    #[serde(rename = "@attr")]
    pub attrs: Option<TrackAttributes>,
    /// The name of the track.
    pub name: String,
    /// The album the track is associated with.
    pub album: Album,
    /// The last.fm URL of the track.
    pub url: String,
    /// Whether or not a track is streamable.
    pub streamable: String,
    /// Images associated with the track.
    #[serde(rename = "image")]
    pub images: Vec<Image>,
    /// The date of when the track was scrobbled.
    pub date: Option<Date>,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    /// The name of the artist.
    #[serde(rename = "#text")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Album {
    /// The name of the album.
    #[serde(rename = "#text")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TrackAttributes {
    /// Whether or not the user's first available track is the
    /// one the user is currently playing.
    #[serde(rename = "nowplaying")]
    pub now_playing: String,
}

#[derive(Debug, Deserialize)]
pub struct Date {
    /// The date of a [Track] in UTC
    #[serde(rename = "uts")]
    #[serde(deserialize_with = "deserialize_datetime_from_str")]
    pub date: DateTime<Utc>,

    /// The date of when a [Track] was first scrobbled on Last.fm, formatted as
    /// `%d %b %Y, %H:%M`, for example: "11 Dec 2020, 23:12"
    #[serde(rename = "#text")]
    pub date_formatted: String,
}

impl RecentTracks {
    pub async fn build<'a>(client: &'a mut Client, user: &str) -> RequestBuilder<'a, RecentTracks> {
        let url = client.build_url(vec![("method", "user.getRecentTracks"), ("user", user)]).await;

        RequestBuilder { client, url, phantom: PhantomData }
    }
}

impl<'a> RequestBuilder<'a, RecentTracks> {
    add_param!(with_limit, limit, usize);
    add_param!(with_page, page, usize);

    pub async fn send(&'a mut self) -> Result<RecentTracks, Error> {
        match self.client.request(&self.url).await {
            Ok(response) => {
                let body = response.text().await.unwrap();
                match serde_json::from_str::<LastFMError>(&body) {
                    Ok(lastm_error) => Err(Error::LastFMError(lastm_error.into())),
                    Err(_) => match serde_json::from_str::<User>(&body) {
                        Ok(user) => Ok(user.recent_tracks.ok_or("Error while getting recent tracks").unwrap()),
                        Err(e) => Err(Error::ParsingError(e)),
                    },
                }
            }
            Err(err) => Err(Error::HTTPError(err)),
        }
    }
}

impl<'a> Client {
    pub async fn recent_tracks(&'a mut self, user: &str) -> RequestBuilder<'a, RecentTracks> { RecentTracks::build(self, user).await }
}
