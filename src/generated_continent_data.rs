// @generated
#![allow(warnings)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::needless_arbitrary_self_type)]

#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug, clap::ValueEnum)]
// #[clap(rename_all = "PascalCase")]
pub enum Continent {
    #[clap(alias = "NA", name = "North America")]
    NorthAmerica,
    #[clap(alias = "SA", name = "South America")]
    SouthAmerica,
    #[clap(alias = "EU", name = "Europe")]
    Europe,
    #[clap(alias = "AS", name = "Asia")]
    Asia,
    #[clap(alias = "AF", name = "Africa")]
    Africa,
    #[clap(alias = "OC", name = "Oceania")]
    Oceania,
    #[clap(alias = "AN", name = "Antartica")]
    Antartica,
}

impl Continent {
    pub const ALL_CONTINENTS: &[Self] = &[
        Continent::NorthAmerica,
        Continent::SouthAmerica,
        Continent::Europe,
        Continent::Asia,
        Continent::Africa,
        Continent::Oceania,
        Continent::Antartica
    ];

    pub fn emoji(self: &Self) -> &'static str {
        match self {
            Self::NorthAmerica => r###"🗽"###,
            Self::SouthAmerica => r###"🌵"###,
            Self::Europe => r###"🏰"###,
            Self::Asia => r###"🐼"###,
            Self::Africa => r###"🦒"###,
            Self::Oceania => r###"🗿"###,
            Self::Antartica => r###"🧊"###,
        }
    }
}


