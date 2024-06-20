use serde::Deserialize;
use std::marker::PhantomData;

use crate::{
    error::{Error, LastFMError},
    model::Attributes,
    user::{Track, User},
    Client, RequestBuilder,
};

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
    /// A [Vec] containiing the recent [Track]'s a user has played.
    #[serde(rename = "track")]
    pub tracks: Vec<Track>,
}

impl RecentTracks {
    pub async fn build<'a>(client: &'a mut Client, user: &str) -> RequestBuilder<'a, RecentTracks> {
        let url = client.build_url(vec![("method", "user.getRecentTracks"), ("user", user)]).await;

        RequestBuilder { client, url, phantom: PhantomData }
    }
}

impl<'a> RequestBuilder<'a, RecentTracks> {
    add_param!(with_limit, limit, usize);
    add_param!(with_page, page, usize);

    pub async fn send(&'a mut self) -> Result<RecentTracks, Error> {
        match self.client.request(&self.url).await {
            Ok(response) => {
                let body = response.text().await.unwrap();
                match serde_json::from_str::<LastFMError>(&body) {
                    Ok(lastm_error) => Err(Error::LastFMError(lastm_error.into())),
                    Err(_) => match serde_json::from_str::<User>(&body) {
                        Ok(user) => Ok(user.recent_tracks.ok_or("Error while getting recent tracks").unwrap()),
                        Err(e) => Err(Error::ParsingError(e)),
                    },
                }
            }
            Err(err) => Err(Error::HTTPError(err)),
        }
    }
}

impl<'a> Client {
    pub async fn recent_tracks(&'a mut self, user: &str) -> RequestBuilder<'a, RecentTracks> {
        RecentTracks::build(self, user).await
    }
}
