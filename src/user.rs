use std::io::Read;
use std::marker::PhantomData;
use url::Url;
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

pub struct RequestBuilder<'a, T: 'a> {
    client:  &'a mut Client,
    url:     Url,
    phantom: PhantomData<&'a T>
}

impl RecentTracks {
    pub fn build<'a>(client: &'a mut Client, user: &str) -> RequestBuilder<'a, RecentTracks> {
        let url = client.build_url(vec![
                                   ("method", "user.getRecentTracks"),
                                   ("user",   user)
        ]);
        RequestBuilder { client: client, url: url, phantom: PhantomData }
    }
}

impl<'a> RequestBuilder<'a, RecentTracks> {
    add_param!(with_limit, limit, u32);
    add_param!(with_page,  page,  u32);

    pub fn send(&'a mut self) -> Result<RecentTracks, Error> {
        match self.client.request(&self.url) {
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

impl<'a> Client {
    pub fn recent_tracks(&'a mut self, user: &str) -> RequestBuilder<'a, RecentTracks> {
        RecentTracks::build(self, user)
    }
}

#[cfg(test)]
mod tests {
    use ::tests::make_client;

    #[test]
    fn test_recent_tracks() {
        let mut client        = make_client();
        let     recent_tracks = client.recent_tracks("RoxasShadow")
            .with_limit(1)
            .send();
        assert!(recent_tracks.is_ok());
    }
}
