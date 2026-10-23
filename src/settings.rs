//! A small JSON file in the app's data folder (the scope, the app's settings), read leniently and written atomically.
//!
//! A file that is missing or unreadable reads as the default: a damaged settings file never stops the app from opening.
//! A write goes to a temporary file in the same folder, then a rename, so a crash mid-write never leaves half a file.

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::Path;

/// The value stored in `file`, or the default when the file is missing or is not that value.
pub fn load<T: DeserializeOwned + Default>(file: &Path) -> T {
    fs::read_to_string(file)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

/// Writes `value` as pretty JSON, atomically (temporary file, `fsync`, rename), creating the folder if needed.
pub fn save<T: Serialize>(file: &Path, value: &T) -> Result<(), String> {
    let dir = file
        .parent()
        .filter(|d| !d.as_os_str().is_empty())
        .ok_or_else(|| format!("no folder for {}", file.display()))?;
    fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    let name = file
        .file_name()
        .ok_or_else(|| format!("not a file name: {}", file.display()))?;
    let temporary = dir.join(format!(".{}.tmp", name.to_string_lossy()));
    {
        let mut out = fs::File::create(&temporary).map_err(|e| e.to_string())?;
        out.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
        out.sync_all().map_err(|e| e.to_string())?;
    }
    fs::rename(&temporary, file).map_err(|e| {
        let _ = fs::remove_file(&temporary);
        e.to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use tempfile::tempdir;

    #[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
    struct Prefs {
        channel: String,
        count: u32,
    }

    #[test]
    fn a_saved_value_reads_back_and_leaves_no_temporary_file() {
        let d = tempdir().unwrap();
        let file = d.path().join("nested").join("settings.json");
        let prefs = Prefs {
            channel: "beta".into(),
            count: 3,
        };
        save(&file, &prefs).unwrap();
        assert_eq!(load::<Prefs>(&file), prefs);
        let names: Vec<_> = fs::read_dir(file.parent().unwrap())
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        assert_eq!(names, vec![std::ffi::OsString::from("settings.json")]);
    }

    #[test]
    fn a_missing_or_damaged_file_reads_as_the_default() {
        let d = tempdir().unwrap();
        assert_eq!(
            load::<Prefs>(&d.path().join("missing.json")),
            Prefs::default()
        );
        let damaged = d.path().join("damaged.json");
        fs::write(&damaged, "{ not json").unwrap();
        assert_eq!(load::<Prefs>(&damaged), Prefs::default());
    }

    #[test]
    fn a_path_without_a_folder_is_refused() {
        assert!(save(Path::new("settings.json"), &Prefs::default()).is_err());
    }
}
