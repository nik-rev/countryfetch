use std::collections::HashMap;

use std::io::Read as _;
use std::sync::LazyLock;

static COUNTRIES_DATA_BYTES: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let countries = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/countries.rkyv.gz"));
    let mut decoder = flate2::read::GzDecoder::new(&countries[..]);
    let mut countries = Vec::new();
    decoder
        .read_to_end(&mut countries)
        .expect("failed to decompress `countries.rkyv.gz`, which stores data about all countries");

    countries
});

pub static COUNTRIES_DATA: LazyLock<Countries> = LazyLock::new(|| {
    let countries =
        rkyv::access::<ArchivedCountries, rkyv::rancor::Error>(&COUNTRIES_DATA_BYTES[..])
            .expect("failed to decode `countries.rkyv`, which stores data about all countries");
    rkyv::deserialize::<_, rkyv::rancor::Error>(countries)
        .expect("failed to deserialize `countries.rkyv`, which stores data about all countries")
});

#[subdef::subdef(
    derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        serde::Serialize,
        serde::Deserialize,
        Debug,
    ),
    serde(rename_all = "camelCase")
)]
pub struct Countries(pub Vec<Country>);

#[subdef::subdef(
    derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        serde::Serialize,
        serde::Deserialize,
        Debug,
    ),
    serde(rename_all = "camelCase")
)]
pub struct Country {
    pub name: [_; {
        pub struct CountryName {
            pub common: String,
            pub official: String,
            pub native_name: [Option<HashMap<String, _>>; {
                pub struct Translation {
                    pub official: String,
                    pub common: String,
                }
            }],
        }
    }],
    pub tld: Vec<String>,
    pub cca2: String,
    pub ccn3: String,
    pub cca3: String,
    pub cioc: Option<String>,
    pub fifa: Option<String>,
    pub independent: Option<bool>,
    pub status: String,
    pub un_member: bool,
    pub currencies: [Option<HashMap<String, _>>; {
        pub struct Currency {
            pub name: String,
            pub symbol: Option<String>,
        }
    }],
    pub idd: [Option<_>; {
        pub struct Idd {
            pub root: Option<String>,
            pub suffixes: Option<Vec<String>>,
        }
    }],
    pub capital: Option<Vec<String>>,
    pub capital_info: [Option<_>; {
        pub struct CapitalInfo {
            pub latlng: Option<Vec<f64>>,
        }
    }],
    pub alt_spellings: Vec<String>,
    pub region: String,
    pub subregion: Option<String>,
    pub continents: Vec<String>,
    pub languages: Option<HashMap<String, String>>,
    pub translations: HashMap<String, Translation>,
    pub latlng: Vec<f64>,
    pub landlocked: bool,
    pub borders: Vec<String>,
    pub area: f64,
    pub flag: Option<String>,
    pub demonyms: [Option<HashMap<String, _>>; {
        pub struct Demonym {
            pub f: String,
            pub m: String,
        }
    }],
    pub flags: [_; {
        pub struct FlagUrls {
            pub svg: String,
            pub png: String,
            pub alt: Option<String>,
        }
    }],
    pub coat_of_arms: [_; {
        pub struct CoatOfArms {
            pub svg: Option<String>,
            pub png: Option<String>,
        }
    }],
    pub population: u64,
    pub maps: [_; {
        pub struct Maps {
            pub google_maps: String,
            pub open_street_maps: String,
        }
    }],
    pub gini: Option<HashMap<String, f64>>,
    pub car: [_; {
        pub struct Car {
            pub signs: Vec<String>,
            pub side: String,
        }
    }],
    pub postal_code: [Option<PostalCode>; {
        pub struct PostalCode {
            pub format: Option<String>,
            pub regex: Option<String>,
        }
    }],
    pub start_of_week: String,
    pub timezones: Vec<String>,
    // These are NOT in the JSON API, they are added manually in the script
    pub flag_ascii_plain: String,
    pub flag_ascii_colored: String,
    pub flag_palette: Vec<(u8, u8, u8)>,
    // ID of this country, used to get its `CountryKind`
    //
    // Added by the script
    pub country_id: usize,
}
