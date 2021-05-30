# `lastfm-rs` Changelog

All notable changes to the `lastfm-rs` library will be documented in this file. This project adheres to the standards
set out by the [Semantic Versioning][semver] specification.

**NOTE**: This changelog only covers 0.3.x onwards. 0.2.x were more of a starting foundation, and as such I don't see
much point in detailing those releases.

## 0.6.0 — TBD (TBD)

### New Features

- Support for the `track.getSimilar` endpoint. (thanks [@PartialDragster]! — [PR #9][pr:9])

### Improvements

- Restructuring of the various structure models, e.g. `Track`. **NOTE**: This will break anything currently using
  these models, as a lot of properties have been made Options, due to inconsistencies in the Last.fm API, but a
  simple fix for this should be just using `.unwrap()` on these properties. (thanks [@PartialDragster] — [PR #10][pr:10])

## 0.5.0 — The Great Reqwest Update (April 3, 2021)

Welcome to `lastfm-rs` v0.5.0. It isn't a big update, however it is bigger than 0.4.2 in terms of changes. This release
pretty much just updates reqwest to version 0.11.x, which means that as of this release, `lastfm-rs` is now based on Tokio
1.x. Recommended Tokio release as of this version is the latest 1.x version, which is version 1.4.0 at the time of me writing
this. A few other additions are included in this release too, including the addition of a couple deserialization tests.

The next release after this will probably focus on fleshing out the rest of the library, including support for more features
of the Last.fm API.

### New Features

- Introduced deserialization tests. (thanks [@drklee3]! — [PR #5][pr:5])
- Allow cloning, copying, etc on the `Period` enum. (thanks [@drklee3] — [PR #6][pr:6])

### Improvements

- Updated dependencies to their latest versions, including the Tokio 1.x upgrade. (thanks [@strohel] — [PR #7][pr:7])

### Bug Fixes

- Use actual source for `Error::source` instead of using `self`. (thanks [@drklee3] — [PR #8][pr:8])

## 0.4.2 — Better Date Formatting (December 12, 2020)

Fairly minor release today; only improving date formatting via `chrono` and merging another model. No bug fixes are
in this release.

### Improvements

- Improved the formatting of dates throughout the library, adapting the use of the `chrono` date & time library. (thanks
  [@drklee3]! — [PR #3][pr:3]!)
- Merged the Date structs for LovedTracks and RecentTracks to be just one model under the main model file. This makes the
  model easier to update if Last.fm were to ever change or add anything to it.

## 0.4.1 — December 10, 2020

### Bug Fixes

- Fixed library version reference in the main library documentation block.

## 0.4.0 — The Great Documentation Rework (December 10, 2020)

This release brings some significant improvements to the library's documentation. Prior to this release, this library
had some pretty lackluster and incomplete documentation, so the aim for this release was to make an attempt to improve
the documentation in a noticable way. Please note however that examples are still not present as of this release, however
I intend on adding some examples in the near future with a later release. Anyways, onto the changelog!

### Improvements

- Dramatically improved documentation. No examples yet, this will be a focus for a future release.
- Moved a couple models to a file under `model` to reduce code duplication.
- Renamed "total_tracks" and "playcount" to "scrobbles". This is the internal name used by Last.fm,
  so I figured that it makes a tad bit more sense to use Last.fm's naming scheme.

### Bug Fixes

- Removed unnecessary serde_json use statemnet.

## 0.3.3 — December 8, 2020

Only a minor release today with a few improvements and a bug fix.

### Improvements / Minor Features

- Added method to use predefined `reqwest` client (thanks [@drklee3]! — [PR #1][pr:1]!)
- Improved error handling (thanks again [@drklee3]! — [PR #2][pr:2])

### Bug Fixes

- `display_name` in the UserInfo struct is no longer an Option.

## 0.3.0 / 0.3.1 / 0.3.2 — November 21, 2020

Triple release! Adds async compatibility, renames the library internally, and also removes
an unused feature flag from `reqwest`. This set of releases also drops support for syncrhonous
Rust in favor of an entirely async approach.

### New Features

- Added async compatibility.

### Minor Changes

- Internally renamed the library from `rustfm`, the crate's prior name, to `lastfm_rs`.
- Removed the `blocking` feature flag from the `reqwest` crate.

[semver]: http://semver.org

[@drklee3]: https://github.com/drklee3
[@PartialDragster]: https://github.com/PartialDragster
[@strohel]: https://github.com/strohel

[pr:1]: https://github.com/KamranMackey/lastfm-rs/pull/1
[pr:2]: https://github.com/KamranMackey/lastfm-rs/pull/2
[pr:3]: https://github.com/KamranMackey/lastfm-rs/pull/3
[pr:5]: https://github.com/KamranMackey/lastfm-rs/pull/5
[pr:6]: https://github.com/KamranMackey/lastfm-rs/pull/6
[pr:7]: https://github.com/KamranMackey/lastfm-rs/pull/7
[pr:8]: https://github.com/KamranMackey/lastfm-rs/pull/8
[pr:9]: https://github.com/KamranMackey/lastfm-rs/pull/9
[pr:10]: https://github.com/KamranMackey/lastfm-rs/pull/10
