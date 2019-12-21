use error::{Error, LastFMError};
use serde_json;
use std::io::Read;
use std::marker::PhantomData;
use user::User;
use {Client, RawData, RequestBuilder};

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
pub struct Attributes {
    pub page: String,
    pub total: String,
    pub user: String,
    #[serde(rename = "perPage")]
    pub per_page: String,
    #[serde(rename = "totalPages")]
    pub total_pages: String,
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
    #[serde(rename = "#text")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Album {
    #[serde(rename = "#text")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TrackAttributes {
    /// Whether or not the user's first available track is the
    /// one the user is currently playing. This is technically
    /// "nowplaying" in the Last.fm API, however it was renamed
    /// to cohere to Rust's naming conventions.
    #[serde(rename = "nowplaying")]
    pub now_playing: String,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    #[serde(rename = "size")]
    pub image_size: String,
    #[serde(rename = "#text")]
    pub image_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Date {
    /// The timestamp of a [Track] in the form of a UNIX Epoch /
    /// Timestamp.
    #[serde(rename = "uts")]
    pub unix_timestamp: String,
    /// The friendly date of when a [Track] was first scrobbled
    /// on Last.fm.
    #[serde(rename = "#text")]
    pub friendly_date: String,
}

impl RecentTracks {
    pub fn build<'a>(client: &'a mut Client, user: &str) -> RequestBuilder<'a, RecentTracks> {
        let url = client.build_url(vec![("method", "user.getRecentTracks"), ("user", user)]);

        RequestBuilder {
            client,
            url,
            phantom: PhantomData,
        }
    }
}

impl<'a> RequestBuilder<'a, RecentTracks> {
    add_param!(with_limit, limit, usize);
    add_param!(with_page, page, usize);

    pub fn send(&'a mut self) -> Result<RecentTracks, Error> {
        match self.client.request(&self.url) {
            Ok(mut response) => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();

                match serde_json::from_str::<LastFMError>(&*body) {
                    Ok(lastm_error) => Err(Error::LastFMError(lastm_error.into())),
                    Err(_) => match serde_json::from_str::<User>(&*body) {
                        Ok(user) => Ok(user.recent_tracks.unwrap()),
                        Err(e) => Err(Error::ParsingError(e)),
                    },
                }
            }
            Err(err) => Err(Error::HTTPError(err)),
        }
    }
}

impl<'a> Client {
    pub fn recent_tracks(&'a mut self, user: &str) -> RequestBuilder<'a, RecentTracks> {
        RecentTracks::build(self, user)
    }
}

#[cfg(test)]
mod tests {
    use tests::make_client;

    #[test]
    fn test_recent_tracks() {
        let mut client = make_client();
        let recent_tracks = client.recent_tracks("MackeyKamran").with_limit(1).send();
        assert!(recent_tracks.is_ok());
    }

    #[test]
    fn test_recent_tracks_not_found() {
        let mut client = make_client();
        let recent_tracks = client.recent_tracks("nonesistinonesistinonesisti").send();
        assert_eq!(&*format!("{:?}", recent_tracks),
           "Err(LastFMError(InvalidParameter(LastFMError { error: 6, message: \"User not found\", links: [] })))");
    }
}
