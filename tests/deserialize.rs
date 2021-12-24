/// Partially derived from serenity-rs/serenity tests
use lastfm_rs::user::user_info::UserInfo;
use lastfm_rs::user::User;
use serde::de::Deserialize;
use serde_json::Value;
use std::fs::File;

macro_rules! p {
    ($s:ident, $filename:expr) => {{
        let f = File::open(concat!("./tests/resources/", $filename, ".json")).expect("Opening test file");

        let v = serde_json::from_reader::<File, Value>(f).expect("Loading test file");

        $s::deserialize(v).expect("Deserializing file");
    }};
}

#[test]
fn user_recent_tracks() {
    p!(User, "user.getRecentTracks");
}

#[test]
fn user_top_artists() {
    p!(User, "user.getTopArtists");
}

#[test]
fn user_get_info() {
    p!(UserInfo, "user.getInfo");
}

#[test]
fn user_get_loved_tracks() {
    p!(User, "user.getLovedTracks");
}

#[test]
fn user_get_top_tracks() {
    p!(User, "user.getTopTracks");
}
