mod country;
mod country_impl;
mod flag;

pub use country::*;

pub enum CurrencyPosition {
    Left,
    Right,
}

pub fn currency_position(country: Country) -> CurrencyPosition {
    match country {
        Country::UnitedStates
        | Country::UnitedKingdom
        | Country::Canada
        | Country::Australia
        | Country::HongKong
        | Country::Singapore
        | Country::NewZealand
        | Country::Ireland
        | Country::Jamaica
        | Country::Bahamas
        | Country::Barbados
        | Country::Belize
        | Country::Bermuda
        | Country::BritishVirginIslands
        | Country::CaymanIslands
        | Country::Fiji
        | Country::Gibraltar
        | Country::Guernsey
        | Country::IsleOfMan
        | Country::Jersey
        | Country::Liberia
        | Country::Malawi
        | Country::Namibia
        | Country::Nigeria
        | Country::PuertoRico
        | Country::SouthAfrica
        | Country::TrinidadAndTobago
        | Country::Zimbabwe
        | Country::China
        | Country::Japan
        | Country::Taiwan
        | Country::SouthKorea
        | Country::Thailand
        | Country::Malaysia
        | Country::Philippines
        | Country::Mexico
        | Country::Argentina
        | Country::Chile
        | Country::Colombia
        | Country::Brazil
        | Country::Peru
        | Country::Uruguay
        | Country::Venezuela
        | Country::CostaRica
        | Country::DominicanRepublic
        | Country::ElSalvador
        | Country::Guatemala
        | Country::Honduras
        | Country::Nicaragua
        | Country::Panama
        | Country::Israel
        | Country::SaudiArabia
        | Country::UnitedArabEmirates
        | Country::Qatar
        | Country::Kuwait
        | Country::Bahrain
        | Country::Oman
        | Country::India
        | Country::Pakistan
        | Country::Bangladesh
        | Country::SriLanka
        | Country::Nepal
        | Country::Maldives => CurrencyPosition::Left,
        _ => CurrencyPosition::Right,
    }
}

/// The date when each country was established
pub fn established_date(country: Country) -> &'static str {
    match country {
        Country::Afghanistan => "August 19, 1919",
        Country::AlandIslands => "December 6, 1917",
        Country::Albania => "November 28, 1912",
        Country::Algeria => "July 5, 1962",
        Country::AmericanSamoa => "April 17, 1900",
        Country::Andorra => "September 8, 1278",
        Country::Angola => "November 11, 1975",
        Country::Anguilla => "December 19, 1980",
        Country::Antarctica => "December 1, 1959",
        Country::AntiguaAndBarbuda => "November 1, 1981",
        Country::Argentina => "July 9, 1816",
        Country::Armenia => "September 21, 1991",
        Country::Aruba => "January 1, 1986",
        Country::Australia => "January 1, 1901",
        Country::Austria => "October 26, 1955",
        Country::Azerbaijan => "August 30, 1991",
        Country::Bahamas => "July 10, 1973",
        Country::Bahrain => "August 15, 1971",
        Country::Bangladesh => "March 26, 1971",
        Country::Barbados => "November 30, 1966",
        Country::Belarus => "July 27, 1990",
        Country::Belgium => "October 4, 1830",
        Country::Belize => "September 21, 1981",
        Country::Benin => "August 1, 1960",
        Country::Bermuda => "August 8, 1775",
        Country::Bhutan => "December 17, 1907",
        Country::Bolivia => "August 6, 1825",
        Country::BosniaAndHerzegovina => "March 1, 1992",
        Country::Botswana => "September 30, 1966",
        Country::BouvetIsland => "December 1, 1927",
        Country::Brazil => "September 7, 1822",
        Country::BritishIndianOceanTerritory => "November 8, 1965",
        Country::BritishVirginIslands => "June 13, 1956",
        Country::Brunei => "January 1, 1984",
        Country::Bulgaria => "March 3, 1878",
        Country::BurkinaFaso => "August 5, 1960",
        Country::Burundi => "July 1, 1962",
        Country::Cambodia => "November 9, 1953",
        Country::Cameroon => "January 1, 1960",
        Country::Canada => "July 1, 1867",
        Country::CapeVerde => "July 5, 1975",
        Country::CaribbeanNetherlands => "October 10, 2010",
        Country::CaymanIslands => "June 4, 1959",
        Country::CentralAfricanRepublic => "August 13, 1960",
        Country::Chad => "August 11, 1960",
        Country::Chile => "February 12, 1818",
        Country::China => "October 1, 1949",
        Country::ChristmasIsland => "October 1, 1958",
        Country::CocosKeelingIslands => "November 23, 1955",
        Country::Colombia => "July 20, 1810",
        Country::Comoros => "July 6, 1975",
        Country::RepublicOfTheCongo => "August 15, 1960",
        Country::DrCongo => "June 30, 1960",
        Country::CookIslands => "August 4, 1965",
        Country::CostaRica => "September 15, 1821",
        Country::Croatia => "June 25, 1991",
        Country::Cuba => "May 20, 1902",
        Country::Curacao => "October 10, 2010",
        Country::Cyprus => "August 16, 1960",
        Country::Czechia => "January 1, 1993",
        Country::Denmark => "June 5, 1849",
        Country::Djibouti => "June 27, 1977",
        Country::Dominica => "November 3, 1978",
        Country::DominicanRepublic => "February 27, 1844",
        Country::Ecuador => "May 24, 1822",
        Country::Egypt => "February 28, 1922",
        Country::ElSalvador => "September 15, 1821",
        Country::EquatorialGuinea => "October 12, 1968",
        Country::Eritrea => "May 24, 1993",
        Country::Estonia => "February 24, 1918",
        Country::Eswatini => "September 6, 1968",
        Country::Ethiopia => "May 5, 1941",
        Country::FalklandIslands => "January 3, 1833",
        Country::FaroeIslands => "July 29, 1948",
        Country::Fiji => "October 10, 1970",
        Country::Finland => "December 6, 1917",
        Country::France => "October 5, 1958",
        Country::FrenchGuiana => "March 19, 1946",
        Country::FrenchPolynesia => "September 28, 1958",
        Country::FrenchSouthernAndAntarcticLands => "August 6, 1955",
        Country::Gabon => "August 17, 1960",
        Country::Gambia => "February 18, 1965",
        Country::Georgia => "April 9, 1991",
        Country::Germany => "October 3, 1990",
        Country::Ghana => "March 6, 1957",
        Country::Gibraltar => "August 4, 1704",
        Country::Greece => "March 25, 1821",
        Country::Greenland => "June 21, 2009",
        Country::Grenada => "February 7, 1974",
        Country::Guadeloupe => "March 19, 1946",
        Country::Guam => "July 21, 1944",
        Country::Guatemala => "September 15, 1821",
        Country::Guernsey => "May 8, 1945",
        Country::Guinea => "October 2, 1958",
        Country::GuineaBissau => "September 24, 1973",
        Country::Guyana => "May 26, 1966",
        Country::Haiti => "January 1, 1804",
        Country::HeardIslandAndMcDonaldIslands => "May 20, 1936",
        Country::Honduras => "September 15, 1821",
        Country::HongKong => "July 1, 1997",
        Country::Hungary => "October 23, 1989",
        Country::Iceland => "June 17, 1944",
        Country::India => "August 15, 1947",
        Country::Indonesia => "August 17, 1945",
        Country::Iran => "April 1, 1979",
        Country::Iraq => "October 3, 1932",
        Country::Ireland => "December 6, 1922",
        Country::IsleOfMan => "May 8, 1945",
        Country::Israel => "May 14, 1948",
        Country::Italy => "June 2, 1946",
        Country::IvoryCoast => "August 7, 1960",
        Country::Jamaica => "August 6, 1962",
        Country::Japan => "February 11, 660 BC",
        Country::Jersey => "May 8, 1945",
        Country::Jordan => "May 25, 1946",
        Country::Kazakhstan => "December 16, 1991",
        Country::Kenya => "December 12, 1963",
        Country::Kiribati => "July 12, 1979",
        Country::Kosovo => "February 17, 2008",
        Country::Kuwait => "June 19, 1961",
        Country::Kyrgyzstan => "August 31, 1991",
        Country::Laos => "July 19, 1949",
        Country::Latvia => "November 18, 1918",
        Country::Lebanon => "November 22, 1943",
        Country::Lesotho => "October 4, 1966",
        Country::Liberia => "July 26, 1847",
        Country::Libya => "December 24, 1951",
        Country::Liechtenstein => "July 23, 1806",
        Country::Lithuania => "February 16, 1918",
        Country::Luxembourg => "October 3, 1839",
        Country::Macau => "December 20, 1999",
        Country::NorthMacedonia => "September 8, 1991",
        Country::Madagascar => "June 26, 1960",
        Country::Malawi => "July 6, 1964",
        Country::Malaysia => "August 31, 1957",
        Country::Maldives => "July 26, 1965",
        Country::Mali => "September 22, 1960",
        Country::Malta => "September 21, 1964",
        Country::MarshallIslands => "October 21, 1986",
        Country::Martinique => "March 19, 1946",
        Country::Mauritania => "November 28, 1960",
        Country::Mauritius => "March 12, 1968",
        Country::Mayotte => "March 31, 2011",
        Country::Mexico => "September 16, 1810",
        Country::Micronesia => "November 3, 1986",
        Country::Moldova => "August 27, 1991",
        Country::Monaco => "January 8, 1297",
        Country::Mongolia => "December 29, 1911",
        Country::Montenegro => "June 3, 2006",
        Country::Montserrat => "November 1, 1983",
        Country::Morocco => "March 2, 1956",
        Country::Mozambique => "June 25, 1975",
        Country::Myanmar => "January 4, 1948",
        Country::Namibia => "March 21, 1990",
        Country::Nauru => "January 31, 1968",
        Country::Nepal => "December 21, 1923",
        Country::Netherlands => "July 26, 1581",
        Country::NewCaledonia => "January 1, 1853",
        Country::NewZealand => "February 6, 1840",
        Country::Nicaragua => "September 15, 1821",
        Country::Niger => "August 3, 1960",
        Country::Nigeria => "October 1, 1960",
        Country::Niue => "October 19, 1974",
        Country::NorfolkIsland => "October 8, 1856",
        Country::NorthKorea => "September 9, 1948",
        Country::NorthernMarianaIslands => "November 3, 1986",
        Country::Norway => "May 17, 1814",
        Country::Oman => "November 18, 1650",
        Country::Pakistan => "August 14, 1947",
        Country::Palau => "October 1, 1994",
        Country::Palestine => "November 15, 1988",
        Country::Panama => "November 3, 1903",
        Country::PapuaNewGuinea => "September 16, 1975",
        Country::Paraguay => "May 14, 1811",
        Country::Peru => "July 28, 1821",
        Country::Philippines => "June 12, 1898",
        Country::PitcairnIslands => "January 15, 1790",
        Country::Poland => "November 11, 1918",
        Country::Portugal => "June 10, 1580",
        Country::PuertoRico => "July 25, 1952",
        Country::Qatar => "September 3, 1971",
        Country::Reunion => "March 19, 1946",
        Country::Romania => "May 9, 1877",
        Country::Russia => "December 25, 1991",
        Country::Rwanda => "July 1, 1962",
        Country::SaintBarthelemy => "July 15, 2007",
        Country::SaintHelenaAscensionAndTristanDaCunha => "January 12, 1659",
        Country::SaintKittsAndNevis => "September 19, 1983",
        Country::SaintLucia => "February 22, 1979",
        Country::SaintMartin => "July 15, 2007",
        Country::SaintPierreAndMiquelon => "April 27, 1816",
        Country::SaintVincentAndTheGrenadines => "October 27, 1979",
        Country::Samoa => "January 1, 1962",
        Country::SanMarino => "September 3, 301",
        Country::SaoTomeAndPrincipe => "July 12, 1975",
        Country::SaudiArabia => "September 23, 1932",
        Country::Senegal => "April 4, 1960",
        Country::Serbia => "June 5, 2006",
        Country::Seychelles => "June 29, 1976",
        Country::SierraLeone => "April 27, 1961",
        Country::Singapore => "August 9, 1965",
        Country::SintMaarten => "October 10, 2010",
        Country::Slovakia => "January 1, 1993",
        Country::Slovenia => "June 25, 1991",
        Country::SolomonIslands => "July 7, 1978",
        Country::Somalia => "July 1, 1960",
        Country::SouthAfrica => "May 31, 1910",
        Country::SouthGeorgia => "May 3, 1985",
        Country::SouthKorea => "August 15, 1948",
        Country::SouthSudan => "July 9, 2011",
        Country::Spain => "December 6, 1978",
        Country::SriLanka => "February 4, 1948",
        Country::Sudan => "January 1, 1956",
        Country::Suriname => "November 25, 1975",
        Country::SvalbardAndJanMayen => "February 9, 1920",
        Country::Sweden => "June 6, 1523",
        Country::Switzerland => "August 1, 1291",
        Country::Syria => "April 17, 1946",
        Country::Taiwan => "October 1, 1949",
        Country::Tajikistan => "September 9, 1991",
        Country::Tanzania => "December 9, 1961",
        Country::Thailand => "December 5, 1932",
        Country::TimorLeste => "May 20, 2002",
        Country::Togo => "April 27, 1960",
        Country::Tokelau => "October 29, 1948",
        Country::Tonga => "June 4, 1970",
        Country::TrinidadAndTobago => "August 31, 1962",
        Country::Tunisia => "March 20, 1956",
        Country::Turkey => "October 29, 1923",
        Country::Turkmenistan => "October 27, 1991",
        Country::TurksAndCaicosIslands => "August 30, 1976",
        Country::Tuvalu => "October 1, 1978",
        Country::Uganda => "October 9, 1962",
        Country::Ukraine => "August 24, 1991",
        Country::UnitedArabEmirates => "December 2, 1971",
        Country::UnitedKingdom => "January 1, 1801",
        Country::UnitedStates => "July 4, 1776",
        Country::UnitedStatesMinorOutlyingIslands => "April 1, 1857",
        Country::UnitedStatesVirginIslands => "March 31, 1917",
        Country::Uruguay => "August 25, 1825",
        Country::Uzbekistan => "September 1, 1991",
        Country::Vanuatu => "July 30, 1980",
        Country::VaticanCity => "February 11, 1929",
        Country::Venezuela => "July 5, 1811",
        Country::Vietnam => "September 2, 1945",
        Country::WallisAndFutuna => "July 29, 1961",
        Country::WesternSahara => "February 27, 1976",
        Country::Yemen => "May 22, 1990",
        Country::Zambia => "October 24, 1964",
        Country::Zimbabwe => "April 18, 1980",
    }
}
