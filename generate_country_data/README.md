The script:

- Fetches data about all countries as JSON from <https://restcountries.com>
- Converts this data into a single Rust enum, writes to `countryfetch/src/generated_country_data.rs`
- Does some post-processing. Each country includes an image representing its flag. This flag is converted into colored ASCII and stored as a plain string.
