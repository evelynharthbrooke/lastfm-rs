use crate::error::{Error, LastFMError};
use crate::user::User;
use crate::{Client, RequestBuilder};
use serde::Deserialize;
use std::io::Read;
use std::marker::PhantomData;

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
    #[serde(rename = "@attr")]
    pub attrs: Attributes,
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
    pub artist: Artist,
    pub mbid: String,
    pub date: Option<Date>,
    pub name: String,
    pub url: String,
    #[serde(rename = "image")]
    pub images: Vec<Image>,
    pub streamable: Streamable,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub url: String,
    pub name: String,
    pub mbid: String,
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

#[derive(Debug, Deserialize)]
pub struct Streamable {
    pub fulltrack: String,
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    #[serde(rename = "size")]
    pub image_size: String,
    #[serde(rename = "#text")]
    pub image_url: String,
}

impl LovedTracks {
    pub fn build<'a>(client: &'a mut Client, user: &str) -> RequestBuilder<'a, LovedTracks> {
        let url = client.build_url(vec![("method", "user.getLovedTracks"), ("user", user)]);

        RequestBuilder { client, url, phantom: PhantomData }
    }
}

impl<'a> RequestBuilder<'a, LovedTracks> {
    add_param!(with_limit, limit, usize);
    add_param!(with_page, page, usize);

    pub fn send(&'a mut self) -> Result<LovedTracks, Error> {
        match self.client.request(&self.url) {
            Ok(mut response) => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();

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
    pub fn loved_tracks(&'a mut self, user: &str) -> RequestBuilder<'a, LovedTracks> {
        LovedTracks::build(self, user)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::make_client;

    #[test]
    fn test_loved_tracks() {
        let mut client = make_client();
        let loved_tracks = client.loved_tracks("LAST.HQ").with_limit(1).send();
        println!("{:#?}", loved_tracks);
        assert!(loved_tracks.is_ok());
    }
}
