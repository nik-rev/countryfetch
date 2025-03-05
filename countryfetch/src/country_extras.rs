//! Extra methods for `generated::Country` that are not from the API, but rather manually written.

use crate::generated;

#[derive(PartialEq, PartialOrd, Ord, Clone, Copy, Eq)]
pub enum CurrencyPosition {
    Left,
    Right,
}

pub fn currency_position(country: generated::Country) -> CurrencyPosition {
    match country {
        generated::Country::UnitedStates
        | generated::Country::UnitedKingdom
        | generated::Country::Canada
        | generated::Country::Australia
        | generated::Country::HongKong
        | generated::Country::Singapore
        | generated::Country::NewZealand
        | generated::Country::Ireland
        | generated::Country::Jamaica
        | generated::Country::Bahamas
        | generated::Country::Barbados
        | generated::Country::Belize
        | generated::Country::Bermuda
        | generated::Country::BritishVirginIslands
        | generated::Country::CaymanIslands
        | generated::Country::Fiji
        | generated::Country::Gibraltar
        | generated::Country::Guernsey
        | generated::Country::IsleOfMan
        | generated::Country::Jersey
        | generated::Country::Liberia
        | generated::Country::Malawi
        | generated::Country::Namibia
        | generated::Country::Nigeria
        | generated::Country::PuertoRico
        | generated::Country::SouthAfrica
        | generated::Country::TrinidadAndTobago
        | generated::Country::Zimbabwe
        | generated::Country::China
        | generated::Country::Japan
        | generated::Country::Taiwan
        | generated::Country::SouthKorea
        | generated::Country::Thailand
        | generated::Country::Malaysia
        | generated::Country::Philippines
        | generated::Country::Mexico
        | generated::Country::Argentina
        | generated::Country::Chile
        | generated::Country::Colombia
        | generated::Country::Brazil
        | generated::Country::Peru
        | generated::Country::Uruguay
        | generated::Country::Venezuela
        | generated::Country::CostaRica
        | generated::Country::DominicanRepublic
        | generated::Country::ElSalvador
        | generated::Country::Guatemala
        | generated::Country::Honduras
        | generated::Country::Nicaragua
        | generated::Country::Panama
        | generated::Country::Israel
        | generated::Country::SaudiArabia
        | generated::Country::UnitedArabEmirates
        | generated::Country::Qatar
        | generated::Country::Kuwait
        | generated::Country::Bahrain
        | generated::Country::Oman
        | generated::Country::India
        | generated::Country::Pakistan
        | generated::Country::Bangladesh
        | generated::Country::SriLanka
        | generated::Country::Nepal
        | generated::Country::Maldives => CurrencyPosition::Left,
        _ => CurrencyPosition::Right,
    }
}

/// The date when each country was established
pub fn established_date(country: generated::Country) -> &'static str {
    match country {
        generated::Country::Afghanistan => "August 19, 1919",
        generated::Country::AlandIslands => "December 6, 1917",
        generated::Country::Albania => "November 28, 1912",
        generated::Country::Algeria => "July 5, 1962",
        generated::Country::AmericanSamoa => "April 17, 1900",
        generated::Country::Andorra => "September 8, 1278",
        generated::Country::Angola => "November 11, 1975",
        generated::Country::Anguilla => "December 19, 1980",
        generated::Country::Antarctica => "December 1, 1959",
        generated::Country::AntiguaAndBarbuda => "November 1, 1981",
        generated::Country::Argentina => "July 9, 1816",
        generated::Country::Armenia => "September 21, 1991",
        generated::Country::Aruba => "January 1, 1986",
        generated::Country::Australia => "January 1, 1901",
        generated::Country::Austria => "October 26, 1955",
        generated::Country::Azerbaijan => "August 30, 1991",
        generated::Country::Bahamas => "July 10, 1973",
        generated::Country::Bahrain => "August 15, 1971",
        generated::Country::Bangladesh => "March 26, 1971",
        generated::Country::Barbados => "November 30, 1966",
        generated::Country::Belarus => "July 27, 1990",
        generated::Country::Belgium => "October 4, 1830",
        generated::Country::Belize => "September 21, 1981",
        generated::Country::Benin => "August 1, 1960",
        generated::Country::Bermuda => "August 8, 1775",
        generated::Country::Bhutan => "December 17, 1907",
        generated::Country::Bolivia => "August 6, 1825",
        generated::Country::BosniaAndHerzegovina => "March 1, 1992",
        generated::Country::Botswana => "September 30, 1966",
        generated::Country::BouvetIsland => "December 1, 1927",
        generated::Country::Brazil => "September 7, 1822",
        generated::Country::BritishIndianOceanTerritory => "November 8, 1965",
        generated::Country::BritishVirginIslands => "June 13, 1956",
        generated::Country::Brunei => "January 1, 1984",
        generated::Country::Bulgaria => "March 3, 1878",
        generated::Country::BurkinaFaso => "August 5, 1960",
        generated::Country::Burundi => "July 1, 1962",
        generated::Country::Cambodia => "November 9, 1953",
        generated::Country::Cameroon => "January 1, 1960",
        generated::Country::Canada => "July 1, 1867",
        generated::Country::CapeVerde => "July 5, 1975",
        generated::Country::CaribbeanNetherlands => "October 10, 2010",
        generated::Country::CaymanIslands => "June 4, 1959",
        generated::Country::CentralAfricanRepublic => "August 13, 1960",
        generated::Country::Chad => "August 11, 1960",
        generated::Country::Chile => "February 12, 1818",
        generated::Country::China => "October 1, 1949",
        generated::Country::ChristmasIsland => "October 1, 1958",
        generated::Country::CocosKeelingIslands => "November 23, 1955",
        generated::Country::Colombia => "July 20, 1810",
        generated::Country::Comoros => "July 6, 1975",
        generated::Country::RepublicOfTheCongo => "August 15, 1960",
        generated::Country::DrCongo => "June 30, 1960",
        generated::Country::CookIslands => "August 4, 1965",
        generated::Country::CostaRica => "September 15, 1821",
        generated::Country::Croatia => "June 25, 1991",
        generated::Country::Cuba => "May 20, 1902",
        generated::Country::Curacao => "October 10, 2010",
        generated::Country::Cyprus => "August 16, 1960",
        generated::Country::Czechia => "January 1, 1993",
        generated::Country::Denmark => "June 5, 1849",
        generated::Country::Djibouti => "June 27, 1977",
        generated::Country::Dominica => "November 3, 1978",
        generated::Country::DominicanRepublic => "February 27, 1844",
        generated::Country::Ecuador => "May 24, 1822",
        generated::Country::Egypt => "February 28, 1922",
        generated::Country::ElSalvador => "September 15, 1821",
        generated::Country::EquatorialGuinea => "October 12, 1968",
        generated::Country::Eritrea => "May 24, 1993",
        generated::Country::Estonia => "February 24, 1918",
        generated::Country::Eswatini => "September 6, 1968",
        generated::Country::Ethiopia => "May 5, 1941",
        generated::Country::FalklandIslands => "January 3, 1833",
        generated::Country::FaroeIslands => "July 29, 1948",
        generated::Country::Fiji => "October 10, 1970",
        generated::Country::Finland => "December 6, 1917",
        generated::Country::France => "October 5, 1958",
        generated::Country::FrenchGuiana => "March 19, 1946",
        generated::Country::FrenchPolynesia => "September 28, 1958",
        generated::Country::FrenchSouthernAndAntarcticLands => "August 6, 1955",
        generated::Country::Gabon => "August 17, 1960",
        generated::Country::Gambia => "February 18, 1965",
        generated::Country::Georgia => "April 9, 1991",
        generated::Country::Germany => "October 3, 1990",
        generated::Country::Ghana => "March 6, 1957",
        generated::Country::Gibraltar => "August 4, 1704",
        generated::Country::Greece => "March 25, 1821",
        generated::Country::Greenland => "June 21, 2009",
        generated::Country::Grenada => "February 7, 1974",
        generated::Country::Guadeloupe => "March 19, 1946",
        generated::Country::Guam => "July 21, 1944",
        generated::Country::Guatemala => "September 15, 1821",
        generated::Country::Guernsey => "May 8, 1945",
        generated::Country::Guinea => "October 2, 1958",
        generated::Country::GuineaBissau => "September 24, 1973",
        generated::Country::Guyana => "May 26, 1966",
        generated::Country::Haiti => "January 1, 1804",
        generated::Country::HeardIslandAndMcDonaldIslands => "May 20, 1936",
        generated::Country::Honduras => "September 15, 1821",
        generated::Country::HongKong => "July 1, 1997",
        generated::Country::Hungary => "October 23, 1989",
        generated::Country::Iceland => "June 17, 1944",
        generated::Country::India => "August 15, 1947",
        generated::Country::Indonesia => "August 17, 1945",
        generated::Country::Iran => "April 1, 1979",
        generated::Country::Iraq => "October 3, 1932",
        generated::Country::Ireland => "December 6, 1922",
        generated::Country::IsleOfMan => "May 8, 1945",
        generated::Country::Israel => "May 14, 1948",
        generated::Country::Italy => "June 2, 1946",
        generated::Country::IvoryCoast => "August 7, 1960",
        generated::Country::Jamaica => "August 6, 1962",
        generated::Country::Japan => "February 11, 660 BC",
        generated::Country::Jersey => "May 8, 1945",
        generated::Country::Jordan => "May 25, 1946",
        generated::Country::Kazakhstan => "December 16, 1991",
        generated::Country::Kenya => "December 12, 1963",
        generated::Country::Kiribati => "July 12, 1979",
        generated::Country::Kosovo => "February 17, 2008",
        generated::Country::Kuwait => "June 19, 1961",
        generated::Country::Kyrgyzstan => "August 31, 1991",
        generated::Country::Laos => "July 19, 1949",
        generated::Country::Latvia => "November 18, 1918",
        generated::Country::Lebanon => "November 22, 1943",
        generated::Country::Lesotho => "October 4, 1966",
        generated::Country::Liberia => "July 26, 1847",
        generated::Country::Libya => "December 24, 1951",
        generated::Country::Liechtenstein => "July 23, 1806",
        generated::Country::Lithuania => "February 16, 1918",
        generated::Country::Luxembourg => "October 3, 1839",
        generated::Country::Macau => "December 20, 1999",
        generated::Country::NorthMacedonia => "September 8, 1991",
        generated::Country::Madagascar => "June 26, 1960",
        generated::Country::Malawi => "July 6, 1964",
        generated::Country::Malaysia => "August 31, 1957",
        generated::Country::Maldives => "July 26, 1965",
        generated::Country::Mali => "September 22, 1960",
        generated::Country::Malta => "September 21, 1964",
        generated::Country::MarshallIslands => "October 21, 1986",
        generated::Country::Martinique => "March 19, 1946",
        generated::Country::Mauritania => "November 28, 1960",
        generated::Country::Mauritius => "March 12, 1968",
        generated::Country::Mayotte => "March 31, 2011",
        generated::Country::Mexico => "September 16, 1810",
        generated::Country::Micronesia => "November 3, 1986",
        generated::Country::Moldova => "August 27, 1991",
        generated::Country::Monaco => "January 8, 1297",
        generated::Country::Mongolia => "December 29, 1911",
        generated::Country::Montenegro => "June 3, 2006",
        generated::Country::Montserrat => "November 1, 1983",
        generated::Country::Morocco => "March 2, 1956",
        generated::Country::Mozambique => "June 25, 1975",
        generated::Country::Myanmar => "January 4, 1948",
        generated::Country::Namibia => "March 21, 1990",
        generated::Country::Nauru => "January 31, 1968",
        generated::Country::Nepal => "December 21, 1923",
        generated::Country::Netherlands => "July 26, 1581",
        generated::Country::NewCaledonia => "January 1, 1853",
        generated::Country::NewZealand => "February 6, 1840",
        generated::Country::Nicaragua => "September 15, 1821",
        generated::Country::Niger => "August 3, 1960",
        generated::Country::Nigeria => "October 1, 1960",
        generated::Country::Niue => "October 19, 1974",
        generated::Country::NorfolkIsland => "October 8, 1856",
        generated::Country::NorthKorea => "September 9, 1948",
        generated::Country::NorthernMarianaIslands => "November 3, 1986",
        generated::Country::Norway => "May 17, 1814",
        generated::Country::Oman => "November 18, 1650",
        generated::Country::Pakistan => "August 14, 1947",
        generated::Country::Palau => "October 1, 1994",
        generated::Country::Palestine => "November 15, 1988",
        generated::Country::Panama => "November 3, 1903",
        generated::Country::PapuaNewGuinea => "September 16, 1975",
        generated::Country::Paraguay => "May 14, 1811",
        generated::Country::Peru => "July 28, 1821",
        generated::Country::Philippines => "June 12, 1898",
        generated::Country::PitcairnIslands => "January 15, 1790",
        generated::Country::Poland => "November 11, 1918",
        generated::Country::Portugal => "June 10, 1580",
        generated::Country::PuertoRico => "July 25, 1952",
        generated::Country::Qatar => "September 3, 1971",
        generated::Country::Reunion => "March 19, 1946",
        generated::Country::Romania => "May 9, 1877",
        generated::Country::Russia => "December 25, 1991",
        generated::Country::Rwanda => "July 1, 1962",
        generated::Country::SaintBarthelemy => "July 15, 2007",
        generated::Country::SaintHelenaAscensionAndTristanDaCunha => "January 12, 1659",
        generated::Country::SaintKittsAndNevis => "September 19, 1983",
        generated::Country::SaintLucia => "February 22, 1979",
        generated::Country::SaintMartin => "July 15, 2007",
        generated::Country::SaintPierreAndMiquelon => "April 27, 1816",
        generated::Country::SaintVincentAndTheGrenadines => "October 27, 1979",
        generated::Country::Samoa => "January 1, 1962",
        generated::Country::SanMarino => "September 3, 301",
        generated::Country::SaoTomeAndPrincipe => "July 12, 1975",
        generated::Country::SaudiArabia => "September 23, 1932",
        generated::Country::Senegal => "April 4, 1960",
        generated::Country::Serbia => "June 5, 2006",
        generated::Country::Seychelles => "June 29, 1976",
        generated::Country::SierraLeone => "April 27, 1961",
        generated::Country::Singapore => "August 9, 1965",
        generated::Country::SintMaarten => "October 10, 2010",
        generated::Country::Slovakia => "January 1, 1993",
        generated::Country::Slovenia => "June 25, 1991",
        generated::Country::SolomonIslands => "July 7, 1978",
        generated::Country::Somalia => "July 1, 1960",
        generated::Country::SouthAfrica => "May 31, 1910",
        generated::Country::SouthGeorgia => "May 3, 1985",
        generated::Country::SouthKorea => "August 15, 1948",
        generated::Country::SouthSudan => "July 9, 2011",
        generated::Country::Spain => "December 6, 1978",
        generated::Country::SriLanka => "February 4, 1948",
        generated::Country::Sudan => "January 1, 1956",
        generated::Country::Suriname => "November 25, 1975",
        generated::Country::SvalbardAndJanMayen => "February 9, 1920",
        generated::Country::Sweden => "June 6, 1523",
        generated::Country::Switzerland => "August 1, 1291",
        generated::Country::Syria => "April 17, 1946",
        generated::Country::Taiwan => "October 1, 1949",
        generated::Country::Tajikistan => "September 9, 1991",
        generated::Country::Tanzania => "December 9, 1961",
        generated::Country::Thailand => "December 5, 1932",
        generated::Country::TimorLeste => "May 20, 2002",
        generated::Country::Togo => "April 27, 1960",
        generated::Country::Tokelau => "October 29, 1948",
        generated::Country::Tonga => "June 4, 1970",
        generated::Country::TrinidadAndTobago => "August 31, 1962",
        generated::Country::Tunisia => "March 20, 1956",
        generated::Country::Turkey => "October 29, 1923",
        generated::Country::Turkmenistan => "October 27, 1991",
        generated::Country::TurksAndCaicosIslands => "August 30, 1976",
        generated::Country::Tuvalu => "October 1, 1978",
        generated::Country::Uganda => "October 9, 1962",
        generated::Country::Ukraine => "August 24, 1991",
        generated::Country::UnitedArabEmirates => "December 2, 1971",
        generated::Country::UnitedKingdom => "January 1, 1801",
        generated::Country::UnitedStates => "July 4, 1776",
        generated::Country::UnitedStatesMinorOutlyingIslands => "April 1, 1857",
        generated::Country::UnitedStatesVirginIslands => "March 31, 1917",
        generated::Country::Uruguay => "August 25, 1825",
        generated::Country::Uzbekistan => "September 1, 1991",
        generated::Country::Vanuatu => "July 30, 1980",
        generated::Country::VaticanCity => "February 11, 1929",
        generated::Country::Venezuela => "July 5, 1811",
        generated::Country::Vietnam => "September 2, 1945",
        generated::Country::WallisAndFutuna => "July 29, 1961",
        generated::Country::WesternSahara => "February 27, 1976",
        generated::Country::Yemen => "May 22, 1990",
        generated::Country::Zambia => "October 24, 1964",
        generated::Country::Zimbabwe => "April 18, 1980",
    }
}
