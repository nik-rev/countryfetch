//! Cache

use std::fs;
use std::fs::File;
use std::io::Read as _;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use serde::Deserialize;
use serde::Serialize;

/// Cache allows us to make less network request at the cost of inacurracy if
/// the user moves to another country within the `Cache::REFRESH_AFTER_SEC`
/// period
#[derive(Serialize, Deserialize)]
pub struct Cache {
    /// When was the last time we modified the cache
    modified_time: u64,
    /// Country code stored
    pub country_code: String,
}

impl Cache {
    /// If the cache has not been written to for this amount of seconds,
    /// it will make another networkr request to the country API to get the
    /// user's current country
    const REFRESH_AFTER_SEC: u64 = 30 * 60;
    /// The file that we use for cache
    const CACHE_FILE: &str = "countryfetch.json";

    /// If it is outdated
    fn is_outdated(&self) -> bool {
        (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .saturating_sub(self.modified_time))
            < Self::REFRESH_AFTER_SEC
    }

    /// Returns the file of the cache
    fn cache_file() -> Option<PathBuf> {
        directories::BaseDirs::new().map(|b| b.cache_dir().join(Self::CACHE_FILE))
    }

    /// Read the cache file if we can find it
    pub fn read() -> Option<Self> {
        let mut cache = Vec::new();
        File::open(Self::cache_file()?)
            .ok()?
            .read_to_end(&mut cache)
            .ok()?;
        serde_json::de::from_slice::<Self>(&cache)
            .ok()
            .filter(Self::is_outdated)
    }

    /// Read the cache file if we can find it
    pub fn write(country_code: String) -> Result<(), Box<dyn core::error::Error>> {
        let serialized = serde_json::ser::to_vec(&Self {
            modified_time: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            country_code,
        })?;

        fs::write(Self::cache_file().ok_or("No home directory")?, serialized)?;

        Ok(())
    }
}
