# lastfm-rs

A friendly Rust library for interacting with the Last.fm API endpoints.

## Supported endpoints

These are the currently supported API endpoints by lastfm-rs. Only a few user
endpoints are currently supported, however support for other endpoints will be
considered and / or added later on.

**Note**: There are currently no plans to add support for the Scrobbling API, but
it will be considered.

### User

`user.getRecentTracks` - Gets a users' recent tracks.

`user.getLovedTracks` - Gets a users' loved tracks.

`user.getTopArtists` - Gets a users' top artists.

`user.getUserInfo` - Gets basic information about a given user.
