# Contributing

Countryfetch uses `cargo xtask` as a build script. It fetches data about every country as a JSON Array of Countries then generates Rust code in `gen_country` from that. This allows `countryfetch` to be used when there is no internet connection, as well as other perks (such as increased performance).

Run:

```sh
cargo xtask
```

## Cache

When `countryfetch` fetches data about a country, it is cached to your system's cache folder for 30 minutes. Specifically the only thing we store is the 2-digit ISO code of your country.

This has 2 advantages:

- Improved performance when running `countryfetch` for subsequent invocations
- Allows `countryfetch` to use the Cache as a fallback when there is no internet connection
