use std::io::Read;
use serde_json;
use super::{Client, Error, RawData};

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "recenttracks")]
    pub recent_tracks: Option<RecentTracks>
}

#[derive(Debug, Deserialize)]
pub struct RecentTracks {
    #[serde(rename = "track")]
    pub tracks: Vec<Track>
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub artist: RawData,
    pub name:   String,
    pub album:  RawData,
    pub url:    String,
    #[serde(rename = "image")]
    pub images: Vec<RawData>,
    pub date:   RawData
}

impl RecentTracks {
    pub fn fetch(client: &mut Client, user: &str) -> Result<RecentTracks, Error> {
        let url = client.build_url(vec![
                                   ("method", "user.getRecentTracks"),
                                   ("user",   user)
        ]);

        match client.request(url) {
            Ok(mut response) => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();

                match serde_json::from_str::<User>(&*body) {
                    Ok(user) => Ok(user.recent_tracks.unwrap()),
                    Err(e)   => Err(Error::ParsingError(e))
                }
            },
            Err(err) => Err(Error::HTTPError(err))
        }
    }
}

impl<'a> Client<'a> {
    pub fn recent_tracks(&mut self, user: &str) -> Result<RecentTracks, Error> {
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
