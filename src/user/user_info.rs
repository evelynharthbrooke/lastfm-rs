use crate::error::{Error, LastFMError};
use crate::{Client, RequestBuilder};
use serde::Deserialize;
use std::io::Read;
use std::marker::PhantomData;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    #[serde(rename = "user")]
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "playcount")]
    pub total_tracks: String,
    #[serde(rename = "name")]
    pub username: String,
    pub url: String,
    pub country: String,
    #[serde(rename = "image")]
    pub images: Vec<Image>,
    pub registered: Registered,
    #[serde(rename = "realname")]
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    #[serde(rename = "size")]
    pub image_size: String,
    #[serde(rename = "#text")]
    pub image_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Registered {
    #[serde(rename = "unixtime")]
    pub unix_timestamp: String,
    #[serde(rename = "#text")]
    pub friendly_date: i64, // use i64 format so that chrono likes us
}

impl UserInfo {
    pub fn build<'a>(client: &'a mut Client, user: &str) -> RequestBuilder<'a, UserInfo> {
        let url = client.build_url(vec![("method", "user.getInfo"), ("user", user)]);

        RequestBuilder { client, url, phantom: PhantomData }
    }
}

impl<'a> RequestBuilder<'a, UserInfo> {
    pub fn send(&'a mut self) -> Result<UserInfo, Error> {
        match self.client.request(&self.url) {
            Ok(mut response) => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();

                match serde_json::from_str::<LastFMError>(&*body) {
                    Ok(lastm_error) => Err(Error::LastFMError(lastm_error.into())),
                    Err(_) => match serde_json::from_str::<UserInfo>(&*body) {
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
    pub fn user_info(&'a mut self, user: &str) -> RequestBuilder<'a, UserInfo> {
        UserInfo::build(self, user)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::make_client;

    #[test]
    fn test_user_info() {
        let mut client = make_client();
        let user_info = client.user_info("LAST.HQ").send();
        println!("{:#?}", user_info);
        assert!(user_info.is_ok());
    }
}
