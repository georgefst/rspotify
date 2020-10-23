use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;
use strum::{AsRefStr, Display, EnumString};

#[derive(Clone, Debug)]
pub struct EnumError {
    kind: ErrorKind,
}

impl EnumError {
    pub(crate) fn new(kind: ErrorKind) -> EnumError {
        EnumError { kind }
    }

    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}
/// The kind of an error that can occur.
#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// This error occurs when no proper enum was found.
    NoEnum(String),
}
impl error::Error for EnumError {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::NoEnum(_) => "no proper enum was found",
        }
    }
}

impl fmt::Display for EnumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::NoEnum(ref s) => write!(f, "can't find proper enum of `{:?}`", s),
        }
    }
}
/// Album type - 'album', 'single', 'appears_on', 'compilation'
#[derive(
    Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, EnumString, AsRefStr, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum AlbumType {
    #[strum(serialize = "album")]
    Album,
    #[strum(serialize = "single")]
    Single,
    #[strum(serialize = "appears_on")]
    AppearsOn,
    #[strum(serialize = "compilation")]
    Compilation,
}

///  Type: 'artist', 'album','track', 'playlist', 'show' or 'episode'
#[derive(
    Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, EnumString, AsRefStr, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    #[strum(serialize = "artist")]
    Artist,
    #[strum(serialize = "album")]
    Album,
    #[strum(serialize = "track")]
    Track,
    #[strum(serialize = "playlist")]
    Playlist,
    #[strum(serialize = "user")]
    User,
    #[strum(serialize = "show")]
    Show,
    #[strum(serialize = "episode")]
    Episode,
}

/// additional_typs: track, episode
#[derive(
    Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, EnumString, AsRefStr, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum AdditionalType {
    #[strum(serialize = "track")]
    Track,
    #[strum(serialize = "episode")]
    Episode,
}

/// currently_playing_type: track, episode, ad, unknown.
#[derive(
    Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, EnumString, AsRefStr, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum CurrentlyPlayingType {
    #[strum(serialize = "track")]
    Track,
    #[strum(serialize = "episode")]
    Episode,
    #[strum(serialize = "ad")]
    Advertisement,
    #[strum(serialize = "unknown")]
    Unknown,
}

/// Type for search: artist, album, track, playlist, show, episode
#[derive(
    Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, EnumString, AsRefStr, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    #[strum(serialize = "artist")]
    Artist,
    #[strum(serialize = "album")]
    Album,
    #[strum(serialize = "track")]
    Track,
    #[strum(serialize = "playlist")]
    Playlist,
    #[strum(serialize = "show")]
    Show,
    #[strum(serialize = "episode")]
    Episode,
}

/// Device Type: computer, smartphone, speaker, TV, etc.
/// See the [Spotify developer
/// docs](https://developer.spotify.com/documentation/web-api/reference/player/get-a-users-available-devices/#device-types)
/// for more information, or in case we are missing a device type here.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeviceType {
    Computer,
    Tablet,
    Smartphone,
    Speaker,
    TV,
    AVR,
    STB,
    AudioDongle,
    GameConsole,
    CastVideo,
    CastAudio,
    Automobile,
    Unknown,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_album_type_convert_from_str() {
        let album_type = AlbumType::from_str("album");
        assert_eq!(album_type.unwrap(), AlbumType::Album);
        assert_eq!(album_type.unwrap().to_string(), "album".to_string());
        let empty_type = AlbumType::from_str("not exist album");
        assert!(empty_type.is_err());
        let appears_on = AlbumType::AppearsOn;
        assert_eq!("appears_on".to_string(), appears_on.to_string());
        let compilation = AlbumType::Compilation;
        assert_eq!("compilation", compilation.as_ref());
    }
    #[test]
    fn test_convert_search_type_from_str() {
        let search_type = SearchType::from_str("artist");
        assert_eq!(search_type.unwrap(), SearchType::Artist);
        let unknown_search_type = SearchType::from_str("unknown_search_type");
        assert_eq!(unknown_search_type.is_err(), true);
    }

    #[test]
    fn test_type_convert_from_str() {
        let _type = Type::from_str("album");
        assert_eq!(_type.unwrap(), Type::Album);
        let artist = Type::Artist;
        assert_eq!(artist.as_ref(), "artist");
        assert_eq!(artist.to_string(), "artist".to_string());

        let empty_type = Type::from_str("not_exist_type");
        assert!(empty_type.is_err());
    }
    #[test]
    fn test_additional_type() {
        let track = AdditionalType::from_str("track");
        assert_eq!(track.unwrap(), AdditionalType::Track);
        let episode = AdditionalType::Episode;
        assert_eq!(episode.to_string(), "episode".to_string());
        assert_eq!(episode.as_ref(), "episode".to_string());
    }
    #[test]
    fn test_current_playing_type() {
        let track = CurrentlyPlayingType::from_str("track");
        assert_eq!(track.unwrap(), CurrentlyPlayingType::Track);
        let episode = CurrentlyPlayingType::Episode;
        assert_eq!(episode.as_ref(), "episode");
        let ad = CurrentlyPlayingType::Advertisement;
        assert_eq!(ad.as_ref(), "ad");
    }
    #[test]
    fn test_search_type() {
        let artist = SearchType::from_str("artist");
        assert_eq!(artist.unwrap(), SearchType::Artist);
        let episode = SearchType::Episode;
        assert_eq!(episode.as_ref(), "episode");
        assert_eq!(episode.to_string(), "episode".to_string());
    }
}