use serde::Deserialize;
use std::marker::PhantomData;

use crate::{
    error::{Error, LastFMError},
    model::Track,
    track::Endpoints,
    Client, RequestBuilder,
};

/// The main similar structure.
///
/// This structure only has one component to it: the tracks that are similar to the
/// one provided. For details on the information available for the tracks, refer to
/// the [Track] struct.
#[derive(Debug, Deserialize)]
pub struct Similar {
    /// A [Vec] containing similar [Track]s.
    #[serde(rename = "track")]
    pub tracks: Vec<Track>,
}

impl Similar {
    pub async fn build_by_mbid<'a>(client: &'a mut Client, mbid: &str) -> RequestBuilder<'a, Similar> {
        let url = client.build_url(vec![("method", "track.getSimilar"), ("mbid", mbid)]).await;
        RequestBuilder { client, url, phantom: PhantomData }
    }

    pub async fn build<'a>(client: &'a mut Client, artist: &str, track: &str) -> RequestBuilder<'a, Similar> {
        let url = client.build_url(vec![("method", "track.getSimilar"), ("artist", artist), ("track", track)]).await;
        RequestBuilder { client, url, phantom: PhantomData }
    }
}

impl<'a> RequestBuilder<'a, Similar> {
    add_param!(with_limit, limit, usize);

    pub async fn send(&'a mut self) -> Result<Similar, Error> {
        match self.client.request(&self.url).await {
            Ok(response) => {
                let body = response.text().await.unwrap();
                match serde_json::from_str::<LastFMError>(&body) {
                    Ok(lastfm_error) => Err(Error::LastFMError(lastfm_error.into())),
                    Err(_) => match serde_json::from_str::<Endpoints>(&body) {
                        Ok(tracks) => Ok(tracks.similar_tracks.unwrap()),
                        Err(e) => Err(Error::ParsingError(e)),
                    },
                }
            }
            Err(err) => Err(Error::HTTPError(err)),
        }
    }
}

impl<'a> Client {
    pub async fn similar_tracks_by_mbid(&'a mut self, mbid: &str) -> RequestBuilder<'a, Similar> {
        Similar::build_by_mbid(self, mbid).await
    }

    pub async fn similar_tracks(&'a mut self, artist: &str, track: &str) -> RequestBuilder<'a, Similar> {
        Similar::build(self, artist, track).await
    }
}
