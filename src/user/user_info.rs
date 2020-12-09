use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds::deserialize as from_ts;
use serde::Deserialize;
use std::marker::PhantomData;

use crate::{
    error::{Error, LastFMError},
    model::Image,
    Client, RequestBuilder,
};

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub user: User,
}

/// The main user structure. This contains all of the necessary profile fields
/// exposed by the Last.fm API, such as scrobble count, username, the date of
/// when the user joined Last.fm, and more.
#[derive(Debug, Deserialize)]
pub struct User {
    /// The total scrobbles or plays for a user.
    #[serde(rename = "playcount")]
    pub scrobbles: String,
    #[serde(rename = "name")]
    /// The user's username. Assigned when the user
    /// created their Last.fm account.
    pub username: String,
    /// A link to the user's Last.fm profile.
    pub url: String,
    /// The user's listed country. Will be an empty string if
    /// the user has not set their country.
    pub country: String,
    /// The user's profile picture(s), in multiple sizes.
    #[serde(rename = "image")]
    pub images: Vec<Image>,
    /// The registration / join date of the user.
    pub registered: Registered,
    /// The display name of the user.
    ///
    /// This will be an empty string if the user has not yet
    /// set their display name.
    #[serde(rename = "realname")]
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Registered {
    /// The UNIX timestamp of when the user registered their Last.fm account.
    #[serde(rename = "unixtime")]
    pub unix_timestamp: String,
    /// [chrono] DateTime
    ///
    /// [chrono]: https://crates.io/crates/chrono
    #[serde(rename = "#text")]
    #[serde(deserialize_with = "from_ts")]
    pub date: DateTime<Utc>,
}

impl UserInfo {
    pub async fn build<'a>(client: &'a mut Client, user: &str) -> RequestBuilder<'a, UserInfo> {
        let url = client.build_url(vec![("method", "user.getInfo"), ("user", user)]).await;
        RequestBuilder { client, url, phantom: PhantomData }
    }
}

impl<'a> RequestBuilder<'a, UserInfo> {
    pub async fn send(&'a mut self) -> Result<UserInfo, Error> {
        match self.client.request(&self.url).await {
            Ok(response) => {
                let body = response.text().await.unwrap();
                match serde_json::from_str::<LastFMError>(&body) {
                    Ok(lastm_error) => Err(Error::LastFMError(lastm_error.into())),
                    Err(_) => match serde_json::from_str::<UserInfo>(&body) {
                        Ok(user) => Ok(user),
                        Err(e) => Err(Error::ParsingError(e)),
                    },
                }
            }
            Err(err) => Err(Error::HTTPError(err)),
        }
    }
}

impl<'a> Client {
    pub async fn user_info(&'a mut self, user: &str) -> RequestBuilder<'a, UserInfo> { UserInfo::build(self, user).await }
}
