//! Last.fm Data Models
//!
//! These are the various models the crate uses throughout the library, centralized
//! in this file to ease development and remove code duplication.

use crate::model::custom_deserialization::{option_string_or_struct, string_or_struct};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::str::FromStr;
use void::Void;

use crate::utilities::deserialize_datetime_from_str;

pub mod custom_deserialization;

/// Various attributes transmitted by several API endpoints.
#[derive(Debug, Deserialize)]
pub struct Attributes {
    /// The given Page currently paginated in the API.
    pub page: String,
    /// The total amount of items.
    pub total: String,
    /// The user associated with the given item.
    pub user: String,
    /// The amount of items listed in a single page.
    #[serde(rename = "perPage")]
    pub per_page: String,
    /// The total amount of Pages available to paginate.
    #[serde(rename = "totalPages")]
    pub total_pages: String,
}

/// The Date object. Consists of a raw UTC date (able to be formatted), and an already
/// formatted date string ready to be used. The raw date uses the chrono date & time
/// library to format the date.
#[derive(Debug, Deserialize)]
pub struct TrackDate {
    /// The date of when the given Track was scrobbled or loved, in UTC.
    ///
    /// This is not formatted by default, meaning you will have to format
    /// this yourself.
    #[serde(rename = "uts")]
    #[serde(deserialize_with = "deserialize_datetime_from_str")]
    pub raw_date: DateTime<Utc>,
    /// The date of when the given Track was scrobbled or loved, formatted in the
    /// `%d %b %Y, %H:%M` date format, e.g. "11 Dec 2020, 23:12".
    #[serde(rename = "#text")]
    pub formatted_date: String,
}

/// The Image structure. Contains a couple fields solely related to images
/// such as the image's size, and a link to the image hosted on Last.fm's
/// content delivery network.
#[derive(Debug, Deserialize)]
pub struct Image {
    /// The size of the image. Can be small, medium, or large.
    #[serde(rename = "size")]
    pub image_size: String,
    /// A URL to the image, hosted on Last.fm's content delivery network.
    #[serde(rename = "#text")]
    pub image_url: String,
}

/// Contains information about the given track
#[derive(Debug, Deserialize)]
pub struct Track {
    /// The artist who published the  given track.
    #[serde(deserialize_with = "string_or_struct")]
    pub artist: Artist,
    /// Various attributes associated with the track.
    #[serde(rename = "@attr")]
    pub attrs: Option<TrackAttributes>,
    /// The MusicBrainz ID for the given track.
    pub mbid: Option<String>,
    /// The name of the track.
    pub name: String,
    /// The album the track is associated with.
    pub album: Option<Album>,
    /// The last.fm URL of the track.
    pub url: String,
    /// Images associated with the track.
    #[serde(rename = "image")]
    pub images: Vec<Image>,
    /// The date of when the track was scrobbled.
    /// Returned when output comes from some endpoints such as loved_tracks
    pub date: Option<TrackDate>,
    /// Whether or not the track is streamable
    #[serde(deserialize_with = "option_string_or_struct")]
    pub streamable: Option<Streamable>,
    /// The match for the given track
    /// Returned when output comes from some endpoints such as similar
    pub r#match: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct TrackAttributes {
    /// Whether or not the user's first available track is the
    /// one the user is currently playing.
    #[serde(rename = "nowplaying")]
    pub now_playing: Option<String>,
}

impl FromStr for Streamable {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Streamable {
            text: s.to_string(),
            fulltrack: None,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Album {
    /// The name of the album.
    #[serde(rename = "#text")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    /// The name of the artist.
    #[serde(alias = "#text")]
    pub name: String,
    /// The last.fm URL for the given artist.
    pub url: Option<String>,
    /// The MusicBrainz ID of the given artist.
    pub mbid: Option<String>,
    /// Attributes associated with the artist.
    #[serde(rename = "@attr")]
    pub attrs: Option<ArtistAttributes>,
    /// How many times the user has scrobbled the artist.
    #[serde(rename = "playcount")]
    pub scrobbles: Option<String>,
    /// The main images linked to the artist.
    #[serde(rename = "image")]
    pub images: Option<Vec<Image>>,
}

impl FromStr for Artist {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Artist {
            name: s.to_string(),
            url: None,
            mbid: None,
            attrs: None,
            scrobbles: None,
            images: None,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ArtistAttributes {
    /// Where the artist is ranked in the user's profile.
    pub rank: Option<String>,
}

/// The streamable struct.
///
/// Available if the given track is available for streaming.
#[derive(Debug, Deserialize)]
pub struct Streamable {
    pub fulltrack: Option<String>,
    #[serde(rename = "#text")]
    pub text: String,
}
