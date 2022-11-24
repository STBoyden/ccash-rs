***This is an old repository, as such the code here is not of great quality. You've been warned.***

The Rust API for the online [CCash bank API](https://github.com/EntireTwix/CCash).

The library is intended to be used in an asynchronous context.

Documentation is available [here](https://docs.rs/ccash-rs)!

**Important**: The minimum supported Rust version is 1.54.0.

# Versioning

The versioning scheme this API crate uses is as follows:

- Incrementing, for example, `1.21.3` -> `2.21.3` indicates breaking changes
  and X in `X.Y.Z` matches that of the max CCash version the API can comply to, 
  Y being the major version of the API crate and Z being the current patch/minor
  version of the crate.
- Incrementing, for example, `1.21.3` -> `1.22.2` indicates non-breaking changes
  that is still backwards-compatible with the previous version.
