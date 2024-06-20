use serde::Deserialize;
use std::marker::PhantomData;

use crate::{
    error::{Error, LastFMError},
    model::Attributes,
    user::{Artist, User},
    Client, RequestBuilder,
};

/// The main top artists structure.
///
/// This is splitted off into two areas: One, the attributes (used for displaying various
/// user-associated attributes), and two, the user's top artists.
///
/// For details on the attributes available, refer to [Attributes]. For details on the artist information
/// available, refer to [Artist].
#[derive(Debug, Deserialize)]
pub struct TopArtists {
    /// A [Vec] array containing a user's Top Artists.
    #[serde(rename = "artist")]
    pub artists: Vec<Artist>,
    /// Various internal API attributes.
    #[serde(rename = "@attr")]
    pub attrs: Attributes,
}

impl TopArtists {
    /// Constructs / builds the request to the user.getTopArtists API endpoint.
    pub async fn build<'a>(client: &'a mut Client, user: &str) -> RequestBuilder<'a, TopArtists> {
        let url = client.build_url(vec![("method", "user.getTopArtists"), ("user", user)]).await;
        RequestBuilder { client, url, phantom: PhantomData }
    }
}

/// Allows users to specify the period of which they'd like to retrieve top artist data for.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Period {
    /// Retrieves data collected over the past 7 days.
    SevenDays,
    /// Retrieves data collected over the past month.
    OneMonth,
    /// Retrieves data collected over the past three months.
    ThreeMonths,
    /// Retrieves data collected over the past six months.
    SixMonths,
    /// Retrieves data collected over the past twelve months.
    TwelveMonths,
    /// Retrieves data collected over the past year. Retrieves the same data as
    /// [TwelveMonths]. This only serves as a shorter shortcut to the same thing.
    ///
    /// [TwelveMonths]: Period::TwelveMonths
    OneYear,
    /// Retrieves data collected overall, e.g. since the user's Last.fm account
    /// was created.
    Overall,
}

impl ToString for Period {
    /// Converts the given period to a string. In most cases, you won't have to use this
    /// yourself. Period durations will usually be automatically converted to their string
    /// form when fed to the `with_period` parameter function.
    fn to_string(&self) -> String {
        match self {
            Self::Overall => String::from("overall"),
            Self::SevenDays => String::from("7day"),
            Self::OneMonth => String::from("1month"),
            Self::ThreeMonths => String::from("3month"),
            Self::SixMonths => String::from("6month"),
            Self::TwelveMonths | Self::OneYear => String::from("12month"),
        }
    }
}

impl<'a> RequestBuilder<'a, TopArtists> {
    add_param!(with_limit, limit, usize);
    add_param!(within_period, period, Period);
    add_param!(with_page, page, usize);

    pub async fn send(&'a mut self) -> Result<TopArtists, Error> {
        match self.client.request(&self.url).await {
            Ok(response) => {
                let body = response.text().await.unwrap();
                match serde_json::from_str::<LastFMError>(&body) {
                    Ok(lastm_error) => Err(Error::LastFMError(lastm_error.into())),
                    Err(_) => match serde_json::from_str::<User>(&body) {
                        Ok(user) => Ok(user.top_artists.unwrap()),
                        Err(e) => Err(Error::ParsingError(e)),
                    },
                }
            }
            Err(err) => Err(Error::HTTPError(err)),
        }
    }
}

impl<'a> Client {
    pub async fn top_artists(&'a mut self, user: &str) -> RequestBuilder<'a, TopArtists> {
        TopArtists::build(self, user).await
    }
}
