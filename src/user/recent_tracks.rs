use error::{Error, LastFMError};
use serde_json;
use std::io::Read;
use std::marker::PhantomData;
use user::User;
use {Client, RawData, RequestBuilder};

/// http://www.last.fm/api/show/user.getRecentTracks
#[derive(Debug, Deserialize)]
pub struct RecentTracks {
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
    pub artist: RawData,
    #[serde(rename = "@attr")]
    pub attrs: Option<TrackAttributes>,
    pub name: String,
    pub album: RawData,
    pub url: String,
    pub streamable: String,
    #[serde(rename = "image")]
    pub images: Vec<Image>,
    pub date: Option<RawData>,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    /// The size of the image.
    pub size: String,
    /// the text (or URL) associated with the image.
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct TrackAttributes {
    #[serde(rename = "nowplaying")]
    pub now_playing: String
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
    add_param!(with_limit, limit, u32);
    add_param!(with_page, page, u32);

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
        let recent_tracks = client.recent_tracks("MackeyKamran").with_limit(2).send();
        println!("{:#?}", recent_tracks);
        assert!(recent_tracks.is_ok());
    }

    // #[test]
    // fn test_recent_tracks_not_found() {
    //     let mut client = make_client();
    //     let recent_tracks = client.recent_tracks("nonesistinonesistinonesisti").send();
    //     assert_eq!(&*format!("{:?}", recent_tracks),
    //        "Err(LastFMError(InvalidParameter(LastFMError { error: 6, message: \"User not found\", links: [] })))");
    // }
}
