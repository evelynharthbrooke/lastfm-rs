# `lastfm-rs` Changelog

All notable changes to the `lastfm-rs` library will be documented in this file. This project adheres to the standards
set out by the [Semantic Versioning][semver] specification.

**NOTE**: This changelog only covers 0.3.x onwards. 0.2.x were more of a starting foundation, and as such I don't see
much point in detailing those releases.

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

[pr:1]: https://github.com/KamranMackey/lastfm-rs/pull/1
[pr:2]: https://github.com/KamranMackey/lastfm-rs/pull/2
