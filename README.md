# lastfm-rs

A friendly Rust library for interacting with the Last.fm API endpoints.

## Note on Synchronous / Blocking I/O support

As of version 0.3.x, the library is only compatible with async Rust. No support is available
for synchronous code / blocking I/O, however version 0.2.x supports this. Please be advised
however that 0.2.x of the library is no longer being updated with new features; only 0.3.x
and later are being updated moving forward due to the lack of development horsepower.

## Supported endpoints

These are the currently supported API endpoints by lastfm-rs. Only a few user endpoints are
currently supported, however support for other endpoints will be considered and / or added
later on.

**Note**: There are currently no plans to add support for the Scrobbling API, but it will be
considered.

### Track

`track.getSimilar` - Gets tracks similar to the track provided.

### User

`user.getRecentTracks` - Gets a users' recent tracks.

`user.getLovedTracks` - Gets a users' loved tracks.

`user.getTopArtists` - Gets a users' top artists.

`user.getUserInfo` - Gets basic information about a given user.
