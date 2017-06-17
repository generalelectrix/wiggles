//! Saving and loading shows to and from disk.
//! Shows are stored in a folder hierarchy.  The outermost folder has the same name as the show.
//! Individual saves will have a timestamp as their name, in this format:
//! yyyy_mm_dd_hh:mm:ss_mmmm
//! where hh employs 24 time.  This will get interesting when we inevitably run this console on the
//! night in fall where DST ends and the system clock falls back an hour.
//! Inside this folder is a series of save files in a marginally human-readable format, probably
//! a big pile of json.  They are named with the timestamp format above with a .json extension.
//! Inside this folder is a folder named "autosave" which stores show snapshots in a more compact
//! but non-human-readable binary format, probably bincode.  These autosaves are saved with the same
//! filename as a regular save but with the extension .wiggles
use super::Console;
use std::path::PathBuf;
use std::marker::PhantomData;
use std::error::Error;
use std::fmt;
use chrono::prelude::{TimeZone, FixedOffset};
use serde_json::Error as JsonError;

/// Top-level object owning a path to the directory that this console saves and loads shows from.
pub struct Shows {
    base_folder: PathBuf,
}

impl Shows {
    pub fn new(base_folder: PathBuf) -> Self {
        Shows {
            base_folder: base_folder,
        }
    }

    /// Open a show by name.
    fn open<N: Into<String>>(&self, name: N) -> Result<Show, LibraryError> {
        let name = name.into();
        let mut show_path = self.base_folder.clone();
        show_path.push(&name);
        if ! show_path.is_dir() {
            Err(LibraryError::DoesNotExist(name))
        }
        else {
            Ok(Show {
                base_folder: show_path,
                name: name,
            })
        }
    }
}

#[derive(Debug)]
pub struct Load {
    show_name: String,
    spec: LoadSpec,
}

#[derive(Debug)]
pub enum LoadSpec {
    Latest,
    Exact(String),
    LatestAutosave,
    ExactAutosave(String),
}

impl fmt::Display for LoadSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LoadSpec::*;
        match *self {
            Latest => write!(f, "latest save"),
            LatestAutosave => write!(f, "latest autosave"),
            Exact(ref s) => write!(f, "the saved file '{}'", s),
            ExactAutosave(ref s) => write!(f, "the autosaved file '{}'", s),
        }
    }
}

// 2017-06-17_15:44:32.345768000
const DATE_FORMAT: &'static str = "%Y-%m-%d_%H:%M:%S%.9f";

/// Parse a slice of strings as dates and return the index of the latest one, or None if none were
/// valid dates.
fn index_of_latest_date(candidates: &[&str]) -> Option<usize> {
    // We just care about local times, ignore offset from UTC.
    let parser = FixedOffset::west(0);
    candidates.iter()
        .enumerate()
        .filter_map(|(i, s)| {
            parser.datetime_from_str(s, DATE_FORMAT)
                .ok()
                .map(|t| (i, t))
        })
        .max_by_key(|&(i, t)| t)
        .map(|(i, t)| i)
}

/// Helper object for interacting with a single show.
pub struct Show {
    name: String,
    base_folder: PathBuf,
}

// impl Show {
//     /// Load a saved version of this show.
//     pub fn load<C: Console>(&self, spec: LoadSpec) -> Result<C, LibraryError> {
//         use LoadSpec::*;
//         match spec {
//             Latest =>
//         }
//     }
// }

/// Things that might go wrong during show saving and loading.
pub enum LibraryError {
    /// The named show doesn't exist.
    DoesNotExist(String),
    /// An error occurred during deserialization.
    LoadError(JsonError),
}

mod test {
    use super::*;
    
    #[test]
    fn test_index_of_latest_date() {
        let d0 = "2017-10-01_09:07:32.678945000";
        let d1 = "2017-10-01_09:09:32.456213000";
        let d2 = "2017-10-01_09:09:37.345679785";
        let baddate = "baddate";
        
        assert_eq!(index_of_latest_date(&[baddate]), None);
        assert_eq!(index_of_latest_date(&[baddate, baddate, baddate]), None);
        assert_eq!(index_of_latest_date(&[d0]), Some(0));
        assert_eq!(index_of_latest_date(&[d0, baddate]), Some(0));
        assert_eq!(index_of_latest_date(&[baddate, d0]), Some(1));
        assert_eq!(index_of_latest_date(&[d0, d1, baddate, d2]), Some(3));
        assert_eq!(index_of_latest_date(&[baddate, d2, d1, baddate, d0]), Some(1));
    }
}