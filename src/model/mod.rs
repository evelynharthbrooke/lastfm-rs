//! Last.fm Data Models
//!
//! These are the various models the crate uses throughout the library, centralized
//! in this file to ease development and remove code duplication.

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::deserialize_datetime_from_str;

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
    /// `%d %b %Y, %H:%M` date string, e.g. "11 Dec 2020, 23:12".
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
