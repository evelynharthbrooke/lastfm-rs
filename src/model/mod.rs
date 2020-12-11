//! Last.fm Data Models
//!
//! These are the various models the crate uses throughout the library, centralized
//! in this file to ease development and remove code duplication.

use serde::Deserialize;

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
