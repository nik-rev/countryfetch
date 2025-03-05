use serde::Deserialize;
use serde::Serialize;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

/// Cache allows us to make less network request at the cost of inacurracy if the
/// user moves to another country within the [`Cache::REFRESH_AFTER_SEC`] period
#[derive(Serialize, Deserialize)]
pub struct Cache {
    modified_time: u64,
    pub country_code: String,
}

impl Cache {
    /// If the cache has not been written to for this amount of seconds,
    /// it will make another networkr request to the country API to get the user's current country
    const REFRESH_AFTER_SEC: u64 = 18 * 60;
    const CACHE_FILE: &str = "countryfetch.json";

    fn is_outdated(&self) -> bool {
        (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .saturating_sub(self.modified_time))
            < Self::REFRESH_AFTER_SEC
    }

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
        serde_json::de::from_slice::<Cache>(&cache)
            .ok()
            .filter(|cache| cache.is_outdated())
    }

    /// Read the cache file if we can find it
    pub fn write(country_code: String) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_json::ser::to_vec(&Self {
            modified_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            country_code,
        })?;

        fs::write(Self::cache_file().ok_or("No home directory")?, serialized)?;

        Ok(())
    }
}
