use crate::user::top_artists::Period;
use serde::Deserialize;
use std::marker::PhantomData;

use crate::{
    error::{Error, LastFMError},
    model::Attributes,
    user::{Track, User},
    Client, RequestBuilder,
};

/// The main top tracks structure.
///
/// This is splitted off into two areas: One, the attributes (used
/// for displaying various user-associated attributes), and two,
/// a list of the tracks the user has listened to most.
///
/// These tracks will be ordered by rank, however, due to
/// pagination, the 1st element in the list isn't guaranteed to be
/// the user's 1st most listened to track, just the most listened
/// to track in this page.  It is up to you to keep track of what
/// rank that track has.  This behavior may change later on.
///
/// For details on the attributes available, refer to [Attributes]. For
/// details on the track information available, refer to [Track].
#[derive(Debug, Deserialize)]
pub struct TopTracks {
    /// The attributes associated with the user's Top Tracks listing.
    #[serde(rename = "@attr")]
    pub attrs: Attributes,
    /// A [Vec] containing the user's top played tracks.
    #[serde(rename = "track")]
    pub tracks: Vec<Track>,
}

impl TopTracks {
    pub async fn build<'a>(client: &'a mut Client, user: &str) -> RequestBuilder<'a, TopTracks> {
        let url = client.build_url(vec![("method", "user.getTopTracks"), ("user", user)]).await;
        RequestBuilder { client, url, phantom: PhantomData }
    }
}

impl<'a> RequestBuilder<'a, TopTracks> {
    add_param!(with_limit, limit, usize);
    add_param!(with_page, page, usize);
    add_param!(within_period, period, Period);

    pub async fn send(&'a mut self) -> Result<TopTracks, Error> {
        match self.client.request(&self.url).await {
            Ok(response) => {
                let body = response.text().await.unwrap();
                match serde_json::from_str::<LastFMError>(&*body) {
                    Ok(lastm_error) => Err(Error::LastFMError(lastm_error.into())),
                    Err(_) => match serde_json::from_str::<User>(&*body) {
                        Ok(user) => Ok(user.top_tracks.unwrap()),
                        Err(e) => Err(Error::ParsingError(e)),
                    },
                }
            }
            Err(err) => Err(Error::HTTPError(err)),
        }
    }
}

impl<'a> Client {
    pub async fn top_tracks(&'a mut self, user: &str) -> RequestBuilder<'a, TopTracks> {
        TopTracks::build(self, user).await
    }
}
