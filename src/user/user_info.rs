use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::{DateTime, Utc};
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

#[derive(Debug, Deserialize)]
pub struct User {
    /// The username of the user. Assigned when the account was created.
    #[serde(rename = "name")]
    pub username: String,
    /// Whether or not the user is a Last.fm Pro subscriber. 0 if false, 1 if true.
    pub subscriber: String,
    /// The display name of the user. Empty string if unset.
    #[serde(rename = "realname")]
    pub display_name: String,
    /// The total amount of scrobbles or track plays listed on the user's account.
    #[serde(rename = "playcount")]
    pub scrobbles: String,
    /// The amount of unique artists the user has scrobbled / played.
    pub artist_count: String,
    /// The amount of unique tracks the user has scrobbled / played.
    pub track_count: String,
    /// The amount of unique albums the user has scrobbled / played.
    pub album_count: String,
    /// The user's profile picture. Available in multiple sizes.
    #[serde(rename = "image")]
    pub images: Vec<Image>,
    /// The date of when the user joined the Last.fm service.
    pub registered: Registered,
    /// The country the user lives in. Will be empty if unset.
    pub country: String,
    /// The website link for the user's public Last.fm profile.
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Registered {
    /// The UNIX timestamp of when the user registered their Last.fm account.
    #[serde(rename = "unixtime")]
    pub unix_timestamp: String,
    /// A [chrono::DateTime] representation of the user's join date. Easily formattable.
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
    pub async fn user_info(&'a mut self, user: &str) -> RequestBuilder<'a, UserInfo> {
        UserInfo::build(self, user).await
    }
}
