//! Saving and loading shows to and from disk.
//!
//! Shows are stored in a folder hierarchy.  The outermost folder has the same name as the show.
//! Individual saves will have a timestamp as their name, in this format:
//! yyyy_mm_dd_hh:mm:ss_nnnnnnnnn
//! where hh employs 24 time.  This will get interesting when we inevitably run this console on the
//! night in fall where DST ends and the system clock falls back an hour.
//!
//! Inside this folder is a series of save files in a marginally human-readable format, probably
//! a big pile of json.  They are named with the timestamp format above with a .wiggles extension.
//! Inside this folder is a folder named "autosave" which stores show snapshots in a more compact
//! but non-human-readable binary format, probably bincode.  These autosaves are saved with the same
//! filename as a regular save but with the extension .wiggles_autosave
//!
//! We also store a marker file named .clean_close in the show directory, which is written when
//! the console swaps shows or gracefully exists.  We check for the presence or absence of this
//! file when loading a new show.  If absent, we return this fact to the caller to enable future
//! UI action to determine whether to recover from an autosave.
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::marker::PhantomData;
use std::error::Error;
use std::io::Error as IoError;
use std::fmt;
use std::fs;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use chrono::prelude::*;
use serde_json;
use bincode;

/// 
const AUTOSAVE_DIR: &'static str = "autosave";

/// Format string used for parsing and writing save files.
/// 2017-06-17_15:44:32_345768000
/// Note the absence of period characters, so we can naively strip off the file extension when
/// parsing.
const DATE_FORMAT: &'static str = "%Y-%m-%d_%H:%M:%S_%f";

const AUTOSAVE_EXTENSION: &'static str = ".wiggles_autosave";
const SAVE_EXTENSION: &'static str = ".wiggles";
// If we cleanly close a show, we will create this file.
// Upon load, if it is missing, we will prompt to restore from autosave or use last regular save.
const CLEAN_CLOSE_FILE: &'static str = ".clean_close";

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
pub struct ShowLibrary {
    name: String,
    base_folder: PathBuf,
}

/// Clone this path and push a new element onto the end.
fn extend_path(lib: &Path, name: &str) -> PathBuf {
    let mut path = lib.to_path_buf();
    path.push(name);
    path
}

/// Delete every file in this directory with the specified extension(s) and the enclosing directory.
/// Does nothing if the directory contains subdirectories or any file with an extension beside
/// those provided.  Extensions should be provided with a leading period.
/// Returns nothing, but logs anything unexpected that happens.
fn remove_directory_and_files(path: &Path, extensions: &[&str]) {
    // First make sure that there are no subdirectories or unexpected file types.
    // To be extra paranoid, keep a hand-curated list of files to delete and only delete them.
    let mut files_to_remove = Vec::new();

    match fs::read_dir(&path) {
        Err(e) => error!("Could not read the directory '{:?}' because of an error: {}", path, e),
        Ok(items) => {

            for item in items.filter_map(Result::ok) {
                // If this is a directory (or if we can't determine if it is or not), do not proceed.
                if item.file_type().map(|f| f.is_dir()).unwrap_or(true) {
                    error!(
                        "Not removing directory '{:?}' as it contains a subdirectory '{:?}'.",
                        path,
                        item.file_name());
                    return
                }
                // The item is definitely a file, see if it ends with a valid extension.
                let file_name = item.file_name().into_string().unwrap_or("".to_string());
                let mut valid_extension = false;
                for ext in extensions {
                    if file_name.ends_with(ext) {
                        valid_extension = true;
                        files_to_remove.push(file_name.clone());
                        break;
                    }
                }
                // This file didn't have a valid extension abort removal.
                if ! valid_extension {
                    error!(
                        "Not removing directory '{:?}' as file '{}' has an unrecognized extension.",
                        path,
                        file_name);
                    return
                }
            }
            // We're good to go to delete this directory.
        }
    }
    for filename in files_to_remove {
        if let Err(e) = fs::remove_file(extend_path(path, &filename)) {
            error!("Error when removing file {}: {}", filename, e);
        }
    }
    // Try to delete the directory.
    if let Err(e) = fs::remove_dir(path) {
        error!("Error when removing directory {:?}: {}", path, e);
    }
}

impl ShowLibrary {
    /// Create a new show with the given name.
    /// The expected folder hierarchy will be created, and an initial saved state will be recorded
    /// as well as an autosave.
    pub fn create_new<C, N>(
            library_path: &Path,
            name: N,
            console: &C)
            -> Result<Self, LibraryError>
        where C: Serialize, N: Into<String>
    {
        let name = name.into();
        let path = extend_path(library_path, &name);
        if let Err(e) = fs::create_dir(&path) {
            // Exists already or some weird error.
            // For UI purposes we'll just run with duplicate show but log the full error so we can
            // debug weirdness later.
            error!("Could not create a new show due to an error: {}", e);
            return Err(LibraryError::DuplicateName(name));
        }
        let show = ShowLibrary {
            name: name,
            base_folder: path,
        };
        // Try to create the autosave directory.  If this fails, we'll clean up the original to make
        // sure we don't have rogue stuff in the library folder.
        if let Err(e) = fs::create_dir(show.autosave_dir()) {
            error!("Could not create autosave dir due to an error: {}", e);
            show.delete();
            return Err(e.into());
        }
        // Make initial save and autosave of this show.
        if let Err(e) = show.autosave(console) {
            show.delete();
            return Err(e);
        }
        if let Err(e) = show.save(console) {
            show.delete();
            return Err(e);
        }
        Ok(show)
    }

    /// Open the library for an existing show.
    /// Nothing is checked about the show folder at this point except that it exists.
    pub fn open_existing<C, N>(library_path: &Path, name: N) -> Result<Self, LibraryError>
        where N: Into<String>
    {
        let name = name.into();
        let path = extend_path(library_path, &name);
        if ! path.is_dir() {
            Err(LibraryError::ShowDoesNotExist(name))
        }
        else {
            Ok(ShowLibrary {
                name: name,
                base_folder: path,
            })
        }
    }

    /// Save a snapshot of the current state of this show, probably as the result of someone
    /// deciding to hit a save button somewhere.
    pub fn save<C: Serialize>(&self, console: &C) -> Result<(), LibraryError> {
        let now = Local::now();
        let filename = format!("{}{}", now.format(DATE_FORMAT), SAVE_EXTENSION);
        let path = extend_path(&self.base_folder, &filename);
        debug!("Saving show '{}' to {:?}.", self.name, path);
        let file = fs::File::create(path)?;
        serde_json::to_writer_pretty(file, &console).map_err(Into::into)
    }

    /// Autosave a snapshot of the current state of this show.
    pub fn autosave<C: Serialize>(&self, console: &C) -> Result<(), LibraryError> {
        let now = Local::now();
        let filename = format!("{}{}", now.format(DATE_FORMAT), AUTOSAVE_EXTENSION);
        let path = extend_path(&self.autosave_dir(), &filename);
        debug!("Autosaving show '{}' to {:?}.", self.name, path);
        let mut file = fs::File::create(path)?;
        bincode::serialize_into(&mut file, console, bincode::Infinite).map_err(Into::into)
    }

    /// Load a saved version of this show.
    pub fn load<C: DeserializeOwned>(&self, spec: LoadSpec) -> Result<C, LibraryError> {
        use LoadSpec::*;
        debug!("Loading state for show '{}' using spec {:?}.", self.name, spec);        
        match spec {
            Latest => self.load_latest(),
            Exact(name) => self.load_from_save_file(&name),
            LatestAutosave => self.load_latest_autosave(),
            ExactAutosave(name) => self.load_from_autosave_file(&name),
        }
    }

    /// Delete the show directory and all of its contents.
    /// Errors here are logged but this function returns unconditionally.
    fn delete(self) {
        debug!("Deleting show '{}'.", self.name);
        // Delete all autosave files.
        remove_directory_and_files(&self.autosave_dir(), &[AUTOSAVE_EXTENSION]);
        // Delete the show directory.
        remove_directory_and_files(&self.base_folder, &[SAVE_EXTENSION, CLEAN_CLOSE_FILE]);
    }

    /// Return the path to the directory in which this show stores autosaves.
    fn autosave_dir(&self) -> PathBuf {
        let mut base = self.base_folder.clone();
        base.push(AUTOSAVE_DIR);
        base
    }

    /// Return the most recent save file name in this directory.
    fn latest_filename(&self, base_folder: &Path) -> Result<String, LibraryError> {
        use LibraryError::*;
        let mut file_names = 
            filenames(base_folder)
            .map_err(|e| {
                error!("Folder '{}' could not be opened.", base_folder.to_str().unwrap_or(""));
                ShowDoesNotExist(self.name.clone())
            })?;
        let latest_index =
            index_of_latest_date(file_names.as_slice())
            .ok_or(SaveNotFound{name: self.name.clone(), save_name: "any".to_string()})?;
        Ok(file_names.swap_remove(latest_index))
    }

    /// Load the latest save file we have for this show.
    fn load_latest<C: DeserializeOwned>(&self) -> Result<C, LibraryError> {
        let filename = self.latest_filename(&self.base_folder)?;
        self.load_from_save_file(&filename)
    }
    
    /// Load the lastest autosave file we have for this show.
    fn load_latest_autosave<C: DeserializeOwned>(&self) -> Result<C, LibraryError> {
        let filename = self.latest_filename(&self.autosave_dir())?;
        self.load_from_autosave_file(&filename)
    }

    /// Open a named file in base_dir.
    fn open_file(&self, filename: &str, base_dir: &Path) -> Result<fs::File, LibraryError> {
        let mut filepath = PathBuf::from(base_dir);
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
    fn load_from_save_file<C: DeserializeOwned>(&self, filename: &str) -> Result<C, LibraryError> {
        let file = self.open_file(filename, &self.base_folder)?;
        serde_json::from_reader(file).map_err(Into::into)
    }

    /// Try to load console state from this autosave file name.
    fn load_from_autosave_file<C: DeserializeOwned>(&self, filename: &str) -> Result<C, LibraryError> {
        let mut file = self.open_file(filename, &self.autosave_dir())?;
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
    JsonError(serde_json::Error),
    /// An error occurred during autosave deserialization.
    Bincode(bincode::Error),
    /// A show of this name already exists.
    DuplicateName(String),
    /// A save or load operation failed due to a file system error.
    Io(IoError),
}

impl From<serde_json::Error> for LibraryError {
    fn from(e: serde_json::Error) -> Self {
        LibraryError::JsonError(e)
    }
}

impl From<bincode::Error> for LibraryError {
    fn from(e: bincode::Error) -> Self {
        LibraryError::Bincode(e)
    }
}

impl From<IoError> for LibraryError {
    fn from(e: IoError) -> Self {
        LibraryError::Io(e)
    }
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LibraryError::*;
        match *self {
            DuplicateName(ref name) => write!(f, "Duplicate show name '{}'.", name),
            ShowDoesNotExist(ref name) => write!(f, "The show '{}' does not exist.", name),
            SaveNotFound{ref name, ref save_name} =>
                write!(f, "Could not load the save file '{}' for show '{}'.", save_name, name),
            JsonError(ref e) => write!(f, "Show load error: {}", e),
            Bincode(ref e) => write!(f, "Autosave load error: {}", e),
            Io(ref e) => write!(f, "An IO error occurred: {}", e),
        }
    }
}

impl Error for LibraryError {
    fn description(&self) -> &str {
        use LibraryError::*;
        match *self {
            DuplicateName(_) => "Duplicate show name.",
            ShowDoesNotExist(_) => "Show does not exist.",
            SaveNotFound{..} => "Save file not found.",
            JsonError(_) => "Show could not be loaded.",
            Bincode(_) => "Autosave could not be loaded.",
            Io(_) => "IO error occurred.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        use LibraryError::*;
        match *self {
            DuplicateName(_) => None,
            ShowDoesNotExist(_) => None,
            SaveNotFound{..} => None,
            JsonError(ref e) => Some(e),
            Bincode(ref e) => Some(e),
            Io(ref e) => Some(e),
        }
    }
}

#[cfg(test)]
mod test {
    
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