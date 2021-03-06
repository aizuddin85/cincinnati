//! This plugin downloads repository content and extracts it to a given output directory.
//!
//! It is meant to be included in the plugin chain, preceding other plugins who
//! rely on the data being in the output directory.
//! The plugin will only download a tarball if detects a change of revision or on first run.

mod github_v3;
pub mod plugin;

pub use plugin::{
    GithubOpenshiftSecondaryMetadataScraperPlugin, GithubOpenshiftSecondaryMetadataScraperSettings,
};
