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
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::marker::PhantomData;
use std::error::Error;
use std::io::Error as IoError;
use std::fmt;
use std::fs;
use chrono::prelude::{TimeZone, FixedOffset};
use serde_json;
use bincode;

const AUTOSAVE_DIR: &'static Path = Path::new("autosave");

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
            Err(LibraryError::ShowDoesNotExist(name))
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

/// Format string used for parsing and writing save files.
/// 2017-06-17_15:44:32_345768000
/// Note the absence of period characters, so we can naively strip off the file extension when
/// parsing.
const DATE_FORMAT: &'static str = "%Y-%m-%d_%H:%M:%S_%f";

/// Return a vector of the file names in this directory.
fn filenames(dir: &Path) -> Result<Vec<String>, IoError> {
    let entry_iter = fs::read_dir(dir)?;
    // get just the entries that are files
    let filenames = 
        entry_iter
        .filter_map(|r| r.ok())
        // filter out just files
        .filter(|f| f.file_type().map(|t| t.is_file()).unwrap_or(false))
        .map(|f| f.file_name())
        // Convert from OsString to valid utf-8.
        .filter_map(|filename| filename.into_string().ok())
        .collect();
    Ok(filenames)
}

/// Parse a slice of strings as dates and return the index of the latest one, or None if none were
/// valid dates.  It is assumed these strings represent filenames, so their extension will be
/// stripped before date parsing.
fn index_of_latest_date(candidates: &[String]) -> Option<usize> {
    // We just care about local times, ignore offset from UTC.
    let parser = FixedOffset::west(0);
    candidates.iter()
        .enumerate()
        .filter_map(|(i, filename)| {
            let filename_no_ext = filename.split(".").next().unwrap_or(filename);
            parser.datetime_from_str(filename_no_ext, DATE_FORMAT)
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

impl Show {
    /// Load a saved version of this show.
    pub fn load<C: Console>(&self, spec: LoadSpec) -> Result<C, LibraryError> {
        use LoadSpec::*;
        match spec {
            Latest => self.load_latest(),
            Exact(name) => self.load_from_save_file(&name),
        }
    }

    /// Return the path to the directory in which this show stores autosaves.
    fn autosave_dir(&self) -> PathBuf {
        let base = self.base_folder.clone();
        base.push(AUTOSAVE_DIR);
        base
    }

    /// Return the most recent save file name in this directory.
    fn latest_filename(&self, base_folder: &Path) -> Result<String, LibraryError> {
        use LibraryError::*;
        let file_names = 
            filenames(base_folder)
            .map_err(|e| {
                error!("Folder '{}' could not be opened.", base_folder.to_str().unwrap_or(""));
                ShowDoesNotExist(self.name.clone())
            })?;
        let latest_index =
            index_of_latest_date(file_names.as_slice())
            .ok_or(SaveNotFound{name: self.name.clone(), save_name: "any".to_string()})?;
        Ok(file_names[latest_index])
    }

    /// Load the latest save file we have for this show.
    fn load_latest<C: Console>(&self) -> Result<C, LibraryError> {
        let filename = self.latest_filename(&self.base_folder)?;
        self.load_from_save_file(&filename)
    }
    
    /// Load the lastest autosave file we have for this show.
    fn load_latest_autosave<C: Console>(&self) -> Result<C, LibraryError> {
        let filename = self.latest_filename(&self.autosave_dir())?;
        self.load_from_autosave_file(&filename)
    }

    /// Open a named file in base_dir.
    fn open_file(&self, filename: &str, base_dir: &Path) -> Result<fs::File, LibraryError> {
        let filepath = PathBuf::from(base_dir);
        filepath.set_file_name(filename);
        fs::File::open(filepath)
        .map_err(|e| {
            error!("Could not open file '{}' due to an error:\n{}", filename, e);
            LibraryError::SaveNotFound {
                name: self.name.clone(),
                save_name: filename.to_string(),
            }
        })
    }

    /// Try to load console state from this file name.
    fn load_from_save_file<C: Console>(&self, filename: &str) -> Result<C, LibraryError> {
        let file = self.open_file(filename, &self.base_folder)?;
        serde_json::from_reader(file).map_err(Into::into)
    }

    /// Try to load console state from this autosave file name.
    fn load_from_autosave_file<C: Console>(&self, filename: &str) -> Result<C, LibraryError> {
        let file = self.open_file(filename, &self.autosave_dir())?;
        bincode::deserialize_from(&mut file, bincode::Infinite).map_err(Into::into)
    }
}

/// Things that might go wrong during show saving and loading.
#[derive(Debug)]
pub enum LibraryError {
    /// The named show doesn't exist.
    ShowDoesNotExist(String),
    /// A saved state for this show could not be found.
    SaveNotFound{name: String, save_name: String},
    /// An error occurred during deserialization.
    LoadError(serde_json::Error),
    /// An error occurred during autosave deserialization.
    AutosaveLoadError(bincode::Error),
}

impl From<serde_json::Error> for LibraryError {
    fn from(e: serde_json::Error) -> Self {
        LibraryError::LoadError(e)
    }
}

impl From<bincode::Error> for LibraryError {
    fn from(e: bincode::Error) -> Self {
        LibraryError::AutosaveLoadError(e)
    }
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LibraryError::*;
        match *self {
            ShowDoesNotExist(ref name) => write!(f, "The show '{}' does not exist.", name),
            SaveNotFound{name: ref name, save_name: ref save_name} =>
                write!(f, "Could not load the save file '{}' for show '{}'.", save_name, name),
            LoadError(ref e) => write!(f, "Show load error: {}", e),
            AutosaveLoadError(ref e) => write!(f, "Autosave load error: {}", e),
        }
    }
}

impl Error for LibraryError {
    fn description(&self) -> &str {
        use LibraryError::*;
        match *self {
            ShowDoesNotExist(_) => "Show does not exist.",
            SaveNotFound{..} => "Save file not found.",
            LoadError(_) => "Show could not be loaded.",
            AutosaveLoadError(_) => "Autosave could not be loaded.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        use LibraryError::*;
        match *self {
            ShowDoesNotExist(_) => None,
            SaveNotFound{..} => None,
            LoadError(ref e) => Some(e),
            AutosaveLoadError(ref e) => Some(e),
        }
    }
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