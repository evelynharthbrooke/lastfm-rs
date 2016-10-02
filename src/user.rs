use std::io::Read;
use serde_json;
use super::{Client, Error, RawData};

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "recenttracks")]
    recent_tracks: Option<RecentTracks>
}

#[derive(Debug, Deserialize)]
pub struct RecentTracks {
    #[serde(rename = "track")]
    tracks: Vec<Track>
}

#[derive(Debug, Deserialize)]
pub struct Track {
    artist: RawData,
    name:   String,
    album:  RawData,
    url:    String,
    #[serde(rename = "image")]
    images: Vec<RawData>,
    date:   RawData
}

impl RecentTracks {
    pub fn fetch(client: &mut Client, user: &str) -> Result<User, Error> {
        let url = client.build_url(vec![
                                   ("method", "user.getRecentTracks"),
                                   ("user",   user)
        ]);

        match client.request(url) {
            Ok(mut response) => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();

                serde_json::from_str(&*body).map_err(|e| Error::ParsingError(e))
            },
            Err(err) => Err(Error::HTTPError(err))
        }
    }
}

impl<'a> Client<'a> {
    pub fn recent_tracks(&mut self, user: &str) -> Result<User, Error> {
        RecentTracks::fetch(self, user)
    }
}

#[cfg(test)]
mod tests {
    use ::tests::make_client;

    #[test]
    fn test_recent_tracks() {
        let mut client        = make_client();
        let     recent_tracks = client.recent_tracks("RoxasShadow");
        assert!(recent_tracks.is_ok());
    }
}
