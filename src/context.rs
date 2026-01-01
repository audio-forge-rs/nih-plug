//! Different contexts the plugin can use to make callbacks to the host in different...contexts.

use std::fmt::Display;

pub mod gui;
pub mod init;
pub mod process;

// Contexts for more plugin-API specific features
pub mod remote_controls;

/// Information about the host application (DAW).
/// Available when running as a CLAP plugin.
#[derive(Debug, Clone, Default)]
pub struct HostInfo {
    /// The host's name (e.g., "Bitwig Studio", "REAPER").
    pub name: String,
    /// The host's vendor/developer.
    pub vendor: String,
    /// The host's version string.
    pub version: String,
    /// The host's URL (if provided).
    pub url: String,
}

/// Information about the track the plugin is inserted on.
/// Available via the CLAP track-info extension.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackInfo {
    /// The track's name (if provided by host).
    pub name: Option<String>,
    /// The track's color as RGB values (if provided by host).
    pub color: Option<(u8, u8, u8)>,
    /// Number of audio channels on this track.
    pub audio_channel_count: Option<i32>,
    /// The audio port type (e.g., "stereo", "mono").
    pub audio_port_type: Option<String>,
    /// True if this is the master/main output track.
    pub is_for_master: bool,
    /// True if this is a return/aux track (initialize with 100% wet).
    pub is_for_return_track: bool,
    /// True if this is a bus track.
    pub is_for_bus: bool,
}

/// The currently active plugin API. This may be useful to display in an about screen in the
/// plugin's GUI for debugging purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginApi {
    Clap,
    Standalone,
    Vst3,
}

impl Display for PluginApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginApi::Clap => write!(f, "CLAP"),
            PluginApi::Standalone => write!(f, "standalone"),
            PluginApi::Vst3 => write!(f, "VST3"),
        }
    }
}
