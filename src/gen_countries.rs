use crate::countries::COUNTRIES_DATA;
use crate::countries::Country;
#[rustfmt::skip]
#[derive(
    Eq,
    PartialEq,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Debug,
    clap::ValueEnum,
    strum::VariantArray
)]
#[clap(rename_all = "PascalCase")]
pub enum CountryKind {
    #[clap(alias = "AW")]
    Aruba,
    #[clap(alias = "AF")]
    Afghanistan,
    #[clap(alias = "AO")]
    Angola,
    #[clap(alias = "AI")]
    Anguilla,
    #[clap(alias = "AX")]
    AlandIslands,
    #[clap(alias = "AL")]
    Albania,
    #[clap(alias = "AD")]
    Andorra,
    #[clap(alias = "AE")]
    UnitedArabEmirates,
    #[clap(alias = "AR")]
    Argentina,
    #[clap(alias = "AM")]
    Armenia,
    #[clap(alias = "AS")]
    AmericanSamoa,
    #[clap(alias = "AQ")]
    Antarctica,
    #[clap(alias = "TF")]
    FrenchSouthernAndAntarcticLands,
    #[clap(alias = "AG")]
    AntiguaAndBarbuda,
    #[clap(alias = "AU")]
    Australia,
    #[clap(alias = "AT")]
    Austria,
    #[clap(alias = "AZ")]
    Azerbaijan,
    #[clap(alias = "BI")]
    Burundi,
    #[clap(alias = "BE")]
    Belgium,
    #[clap(alias = "BJ")]
    Benin,
    #[clap(alias = "BF")]
    BurkinaFaso,
    #[clap(alias = "BD")]
    Bangladesh,
    #[clap(alias = "BG")]
    Bulgaria,
    #[clap(alias = "BH")]
    Bahrain,
    #[clap(alias = "BS")]
    Bahamas,
    #[clap(alias = "BA")]
    BosniaAndHerzegovina,
    #[clap(alias = "BL")]
    SaintBarthelemy,
    #[clap(alias = "SH")]
    SaintHelenaAscensionAndTristanDaCunha,
    #[clap(alias = "BY")]
    Belarus,
    #[clap(alias = "BZ")]
    Belize,
    #[clap(alias = "BM")]
    Bermuda,
    #[clap(alias = "BO")]
    Bolivia,
    #[clap(alias = "BQ")]
    CaribbeanNetherlands,
    #[clap(alias = "BR")]
    Brazil,
    #[clap(alias = "BB")]
    Barbados,
    #[clap(alias = "BN")]
    Brunei,
    #[clap(alias = "BT")]
    Bhutan,
    #[clap(alias = "BV")]
    BouvetIsland,
    #[clap(alias = "BW")]
    Botswana,
    #[clap(alias = "CF")]
    CentralAfricanRepublic,
    #[clap(alias = "CA")]
    Canada,
    #[clap(alias = "CC")]
    CocosKeelingIslands,
    #[clap(alias = "CH")]
    Switzerland,
    #[clap(alias = "CL")]
    Chile,
    #[clap(alias = "CN")]
    China,
    #[clap(alias = "CI")]
    IvoryCoast,
    #[clap(alias = "CM")]
    Cameroon,
    #[clap(alias = "CD")]
    DrCongo,
    #[clap(alias = "CG")]
    RepublicOfTheCongo,
    #[clap(alias = "CK")]
    CookIslands,
    #[clap(alias = "CO")]
    Colombia,
    #[clap(alias = "KM")]
    Comoros,
    #[clap(alias = "CV")]
    CapeVerde,
    #[clap(alias = "CR")]
    CostaRica,
    #[clap(alias = "CU")]
    Cuba,
    #[clap(alias = "CW")]
    Curacao,
    #[clap(alias = "CX")]
    ChristmasIsland,
    #[clap(alias = "KY")]
    CaymanIslands,
    #[clap(alias = "CY")]
    Cyprus,
    #[clap(alias = "CZ")]
    Czechia,
    #[clap(alias = "DE")]
    Germany,
    #[clap(alias = "DJ")]
    Djibouti,
    #[clap(alias = "DM")]
    Dominica,
    #[clap(alias = "DK")]
    Denmark,
    #[clap(alias = "DO")]
    DominicanRepublic,
    #[clap(alias = "DZ")]
    Algeria,
    #[clap(alias = "EC")]
    Ecuador,
    #[clap(alias = "EG")]
    Egypt,
    #[clap(alias = "ER")]
    Eritrea,
    #[clap(alias = "EH")]
    WesternSahara,
    #[clap(alias = "ES")]
    Spain,
    #[clap(alias = "EE")]
    Estonia,
    #[clap(alias = "ET")]
    Ethiopia,
    #[clap(alias = "FI")]
    Finland,
    #[clap(alias = "FJ")]
    Fiji,
    #[clap(alias = "FK")]
    FalklandIslands,
    #[clap(alias = "FR")]
    France,
    #[clap(alias = "FO")]
    FaroeIslands,
    #[clap(alias = "FM")]
    Micronesia,
    #[clap(alias = "GA")]
    Gabon,
    #[clap(alias = "GB")]
    UnitedKingdom,
    #[clap(alias = "GE")]
    Georgia,
    #[clap(alias = "GG")]
    Guernsey,
    #[clap(alias = "GH")]
    Ghana,
    #[clap(alias = "GI")]
    Gibraltar,
    #[clap(alias = "GN")]
    Guinea,
    #[clap(alias = "GP")]
    Guadeloupe,
    #[clap(alias = "GM")]
    Gambia,
    #[clap(alias = "GW")]
    GuineaBissau,
    #[clap(alias = "GQ")]
    EquatorialGuinea,
    #[clap(alias = "GR")]
    Greece,
    #[clap(alias = "GD")]
    Grenada,
    #[clap(alias = "GL")]
    Greenland,
    #[clap(alias = "GT")]
    Guatemala,
    #[clap(alias = "GF")]
    FrenchGuiana,
    #[clap(alias = "GU")]
    Guam,
    #[clap(alias = "GY")]
    Guyana,
    #[clap(alias = "HK")]
    HongKong,
    #[clap(alias = "HM")]
    HeardIslandAndMcDonaldIslands,
    #[clap(alias = "HN")]
    Honduras,
    #[clap(alias = "HR")]
    Croatia,
    #[clap(alias = "HT")]
    Haiti,
    #[clap(alias = "HU")]
    Hungary,
    #[clap(alias = "ID")]
    Indonesia,
    #[clap(alias = "IM")]
    IsleOfMan,
    #[clap(alias = "IN")]
    India,
    #[clap(alias = "IO")]
    BritishIndianOceanTerritory,
    #[clap(alias = "IE")]
    Ireland,
    #[clap(alias = "IR")]
    Iran,
    #[clap(alias = "IQ")]
    Iraq,
    #[clap(alias = "IS")]
    Iceland,
    #[clap(alias = "IL")]
    Israel,
    #[clap(alias = "IT")]
    Italy,
    #[clap(alias = "JM")]
    Jamaica,
    #[clap(alias = "JE")]
    Jersey,
    #[clap(alias = "JO")]
    Jordan,
    #[clap(alias = "JP")]
    Japan,
    #[clap(alias = "KZ")]
    Kazakhstan,
    #[clap(alias = "KE")]
    Kenya,
    #[clap(alias = "KG")]
    Kyrgyzstan,
    #[clap(alias = "KH")]
    Cambodia,
    #[clap(alias = "KI")]
    Kiribati,
    #[clap(alias = "KN")]
    SaintKittsAndNevis,
    #[clap(alias = "KR")]
    SouthKorea,
    #[clap(alias = "XK")]
    Kosovo,
    #[clap(alias = "KW")]
    Kuwait,
    #[clap(alias = "LA")]
    Laos,
    #[clap(alias = "LB")]
    Lebanon,
    #[clap(alias = "LR")]
    Liberia,
    #[clap(alias = "LY")]
    Libya,
    #[clap(alias = "LC")]
    SaintLucia,
    #[clap(alias = "LI")]
    Liechtenstein,
    #[clap(alias = "LK")]
    SriLanka,
    #[clap(alias = "LS")]
    Lesotho,
    #[clap(alias = "LT")]
    Lithuania,
    #[clap(alias = "LU")]
    Luxembourg,
    #[clap(alias = "LV")]
    Latvia,
    #[clap(alias = "MO")]
    Macau,
    #[clap(alias = "MF")]
    SaintMartin,
    #[clap(alias = "MA")]
    Morocco,
    #[clap(alias = "MC")]
    Monaco,
    #[clap(alias = "MD")]
    Moldova,
    #[clap(alias = "MG")]
    Madagascar,
    #[clap(alias = "MV")]
    Maldives,
    #[clap(alias = "MX")]
    Mexico,
    #[clap(alias = "MH")]
    MarshallIslands,
    #[clap(alias = "MK")]
    NorthMacedonia,
    #[clap(alias = "ML")]
    Mali,
    #[clap(alias = "MT")]
    Malta,
    #[clap(alias = "MM")]
    Myanmar,
    #[clap(alias = "ME")]
    Montenegro,
    #[clap(alias = "MN")]
    Mongolia,
    #[clap(alias = "MP")]
    NorthernMarianaIslands,
    #[clap(alias = "MZ")]
    Mozambique,
    #[clap(alias = "MR")]
    Mauritania,
    #[clap(alias = "MS")]
    Montserrat,
    #[clap(alias = "MQ")]
    Martinique,
    #[clap(alias = "MU")]
    Mauritius,
    #[clap(alias = "MW")]
    Malawi,
    #[clap(alias = "MY")]
    Malaysia,
    #[clap(alias = "YT")]
    Mayotte,
    #[clap(alias = "NA")]
    Namibia,
    #[clap(alias = "NC")]
    NewCaledonia,
    #[clap(alias = "NE")]
    Niger,
    #[clap(alias = "NF")]
    NorfolkIsland,
    #[clap(alias = "NG")]
    Nigeria,
    #[clap(alias = "NI")]
    Nicaragua,
    #[clap(alias = "NU")]
    Niue,
    #[clap(alias = "NL")]
    Netherlands,
    #[clap(alias = "NO")]
    Norway,
    #[clap(alias = "NP")]
    Nepal,
    #[clap(alias = "NR")]
    Nauru,
    #[clap(alias = "NZ")]
    NewZealand,
    #[clap(alias = "OM")]
    Oman,
    #[clap(alias = "PK")]
    Pakistan,
    #[clap(alias = "PA")]
    Panama,
    #[clap(alias = "PN")]
    PitcairnIslands,
    #[clap(alias = "PE")]
    Peru,
    #[clap(alias = "PH")]
    Philippines,
    #[clap(alias = "PW")]
    Palau,
    #[clap(alias = "PG")]
    PapuaNewGuinea,
    #[clap(alias = "PL")]
    Poland,
    #[clap(alias = "PR")]
    PuertoRico,
    #[clap(alias = "KP")]
    NorthKorea,
    #[clap(alias = "PT")]
    Portugal,
    #[clap(alias = "PY")]
    Paraguay,
    #[clap(alias = "PS")]
    Palestine,
    #[clap(alias = "PF")]
    FrenchPolynesia,
    #[clap(alias = "QA")]
    Qatar,
    #[clap(alias = "RE")]
    Reunion,
    #[clap(alias = "RO")]
    Romania,
    #[clap(alias = "RU")]
    Russia,
    #[clap(alias = "RW")]
    Rwanda,
    #[clap(alias = "SA")]
    SaudiArabia,
    #[clap(alias = "SD")]
    Sudan,
    #[clap(alias = "SN")]
    Senegal,
    #[clap(alias = "SG")]
    Singapore,
    #[clap(alias = "GS")]
    SouthGeorgia,
    #[clap(alias = "SJ")]
    SvalbardAndJanMayen,
    #[clap(alias = "SB")]
    SolomonIslands,
    #[clap(alias = "SL")]
    SierraLeone,
    #[clap(alias = "SV")]
    ElSalvador,
    #[clap(alias = "SM")]
    SanMarino,
    #[clap(alias = "SO")]
    Somalia,
    #[clap(alias = "PM")]
    SaintPierreAndMiquelon,
    #[clap(alias = "RS")]
    Serbia,
    #[clap(alias = "SS")]
    SouthSudan,
    #[clap(alias = "ST")]
    SaoTomeAndPrincipe,
    #[clap(alias = "SR")]
    Suriname,
    #[clap(alias = "SK")]
    Slovakia,
    #[clap(alias = "SI")]
    Slovenia,
    #[clap(alias = "SE")]
    Sweden,
    #[clap(alias = "SZ")]
    Eswatini,
    #[clap(alias = "SX")]
    SintMaarten,
    #[clap(alias = "SC")]
    Seychelles,
    #[clap(alias = "SY")]
    Syria,
    #[clap(alias = "TC")]
    TurksAndCaicosIslands,
    #[clap(alias = "TD")]
    Chad,
    #[clap(alias = "TG")]
    Togo,
    #[clap(alias = "TH")]
    Thailand,
    #[clap(alias = "TJ")]
    Tajikistan,
    #[clap(alias = "TK")]
    Tokelau,
    #[clap(alias = "TM")]
    Turkmenistan,
    #[clap(alias = "TL")]
    TimorLeste,
    #[clap(alias = "TO")]
    Tonga,
    #[clap(alias = "TT")]
    TrinidadAndTobago,
    #[clap(alias = "TN")]
    Tunisia,
    #[clap(alias = "TR")]
    Turkey,
    #[clap(alias = "TV")]
    Tuvalu,
    #[clap(alias = "TW")]
    Taiwan,
    #[clap(alias = "TZ")]
    Tanzania,
    #[clap(alias = "UG")]
    Uganda,
    #[clap(alias = "UA")]
    Ukraine,
    #[clap(alias = "UM")]
    UnitedStatesMinorOutlyingIslands,
    #[clap(alias = "UY")]
    Uruguay,
    #[clap(alias = "US")]
    UnitedStates,
    #[clap(alias = "UZ")]
    Uzbekistan,
    #[clap(alias = "VA")]
    VaticanCity,
    #[clap(alias = "VC")]
    SaintVincentAndTheGrenadines,
    #[clap(alias = "VE")]
    Venezuela,
    #[clap(alias = "VG")]
    BritishVirginIslands,
    #[clap(alias = "VI")]
    UnitedStatesVirginIslands,
    #[clap(alias = "VN")]
    Vietnam,
    #[clap(alias = "VU")]
    Vanuatu,
    #[clap(alias = "WF")]
    WallisAndFutuna,
    #[clap(alias = "WS")]
    Samoa,
    #[clap(alias = "YE")]
    Yemen,
    #[clap(alias = "ZA")]
    SouthAfrica,
    #[clap(alias = "ZM")]
    Zambia,
    #[clap(alias = "ZW")]
    Zimbabwe,
}
/// Slice containing information about every country
#[rustfmt::skip]
pub fn all_countries() -> [&'static Country; 250usize] {
    [
        &COUNTRIES_DATA.0[0usize],
        &COUNTRIES_DATA.0[1usize],
        &COUNTRIES_DATA.0[2usize],
        &COUNTRIES_DATA.0[3usize],
        &COUNTRIES_DATA.0[4usize],
        &COUNTRIES_DATA.0[5usize],
        &COUNTRIES_DATA.0[6usize],
        &COUNTRIES_DATA.0[7usize],
        &COUNTRIES_DATA.0[8usize],
        &COUNTRIES_DATA.0[9usize],
        &COUNTRIES_DATA.0[10usize],
        &COUNTRIES_DATA.0[11usize],
        &COUNTRIES_DATA.0[12usize],
        &COUNTRIES_DATA.0[13usize],
        &COUNTRIES_DATA.0[14usize],
        &COUNTRIES_DATA.0[15usize],
        &COUNTRIES_DATA.0[16usize],
        &COUNTRIES_DATA.0[17usize],
        &COUNTRIES_DATA.0[18usize],
        &COUNTRIES_DATA.0[19usize],
        &COUNTRIES_DATA.0[20usize],
        &COUNTRIES_DATA.0[21usize],
        &COUNTRIES_DATA.0[22usize],
        &COUNTRIES_DATA.0[23usize],
        &COUNTRIES_DATA.0[24usize],
        &COUNTRIES_DATA.0[25usize],
        &COUNTRIES_DATA.0[26usize],
        &COUNTRIES_DATA.0[27usize],
        &COUNTRIES_DATA.0[28usize],
        &COUNTRIES_DATA.0[29usize],
        &COUNTRIES_DATA.0[30usize],
        &COUNTRIES_DATA.0[31usize],
        &COUNTRIES_DATA.0[32usize],
        &COUNTRIES_DATA.0[33usize],
        &COUNTRIES_DATA.0[34usize],
        &COUNTRIES_DATA.0[35usize],
        &COUNTRIES_DATA.0[36usize],
        &COUNTRIES_DATA.0[37usize],
        &COUNTRIES_DATA.0[38usize],
        &COUNTRIES_DATA.0[39usize],
        &COUNTRIES_DATA.0[40usize],
        &COUNTRIES_DATA.0[41usize],
        &COUNTRIES_DATA.0[42usize],
        &COUNTRIES_DATA.0[43usize],
        &COUNTRIES_DATA.0[44usize],
        &COUNTRIES_DATA.0[45usize],
        &COUNTRIES_DATA.0[46usize],
        &COUNTRIES_DATA.0[47usize],
        &COUNTRIES_DATA.0[48usize],
        &COUNTRIES_DATA.0[49usize],
        &COUNTRIES_DATA.0[50usize],
        &COUNTRIES_DATA.0[51usize],
        &COUNTRIES_DATA.0[52usize],
        &COUNTRIES_DATA.0[53usize],
        &COUNTRIES_DATA.0[54usize],
        &COUNTRIES_DATA.0[55usize],
        &COUNTRIES_DATA.0[56usize],
        &COUNTRIES_DATA.0[57usize],
        &COUNTRIES_DATA.0[58usize],
        &COUNTRIES_DATA.0[59usize],
        &COUNTRIES_DATA.0[60usize],
        &COUNTRIES_DATA.0[61usize],
        &COUNTRIES_DATA.0[62usize],
        &COUNTRIES_DATA.0[63usize],
        &COUNTRIES_DATA.0[64usize],
        &COUNTRIES_DATA.0[65usize],
        &COUNTRIES_DATA.0[66usize],
        &COUNTRIES_DATA.0[67usize],
        &COUNTRIES_DATA.0[68usize],
        &COUNTRIES_DATA.0[69usize],
        &COUNTRIES_DATA.0[70usize],
        &COUNTRIES_DATA.0[71usize],
        &COUNTRIES_DATA.0[72usize],
        &COUNTRIES_DATA.0[73usize],
        &COUNTRIES_DATA.0[74usize],
        &COUNTRIES_DATA.0[75usize],
        &COUNTRIES_DATA.0[76usize],
        &COUNTRIES_DATA.0[77usize],
        &COUNTRIES_DATA.0[78usize],
        &COUNTRIES_DATA.0[79usize],
        &COUNTRIES_DATA.0[80usize],
        &COUNTRIES_DATA.0[81usize],
        &COUNTRIES_DATA.0[82usize],
        &COUNTRIES_DATA.0[83usize],
        &COUNTRIES_DATA.0[84usize],
        &COUNTRIES_DATA.0[85usize],
        &COUNTRIES_DATA.0[86usize],
        &COUNTRIES_DATA.0[87usize],
        &COUNTRIES_DATA.0[88usize],
        &COUNTRIES_DATA.0[89usize],
        &COUNTRIES_DATA.0[90usize],
        &COUNTRIES_DATA.0[91usize],
        &COUNTRIES_DATA.0[92usize],
        &COUNTRIES_DATA.0[93usize],
        &COUNTRIES_DATA.0[94usize],
        &COUNTRIES_DATA.0[95usize],
        &COUNTRIES_DATA.0[96usize],
        &COUNTRIES_DATA.0[97usize],
        &COUNTRIES_DATA.0[98usize],
        &COUNTRIES_DATA.0[99usize],
        &COUNTRIES_DATA.0[100usize],
        &COUNTRIES_DATA.0[101usize],
        &COUNTRIES_DATA.0[102usize],
        &COUNTRIES_DATA.0[103usize],
        &COUNTRIES_DATA.0[104usize],
        &COUNTRIES_DATA.0[105usize],
        &COUNTRIES_DATA.0[106usize],
        &COUNTRIES_DATA.0[107usize],
        &COUNTRIES_DATA.0[108usize],
        &COUNTRIES_DATA.0[109usize],
        &COUNTRIES_DATA.0[110usize],
        &COUNTRIES_DATA.0[111usize],
        &COUNTRIES_DATA.0[112usize],
        &COUNTRIES_DATA.0[113usize],
        &COUNTRIES_DATA.0[114usize],
        &COUNTRIES_DATA.0[115usize],
        &COUNTRIES_DATA.0[116usize],
        &COUNTRIES_DATA.0[117usize],
        &COUNTRIES_DATA.0[118usize],
        &COUNTRIES_DATA.0[119usize],
        &COUNTRIES_DATA.0[120usize],
        &COUNTRIES_DATA.0[121usize],
        &COUNTRIES_DATA.0[122usize],
        &COUNTRIES_DATA.0[123usize],
        &COUNTRIES_DATA.0[124usize],
        &COUNTRIES_DATA.0[125usize],
        &COUNTRIES_DATA.0[126usize],
        &COUNTRIES_DATA.0[127usize],
        &COUNTRIES_DATA.0[128usize],
        &COUNTRIES_DATA.0[129usize],
        &COUNTRIES_DATA.0[130usize],
        &COUNTRIES_DATA.0[131usize],
        &COUNTRIES_DATA.0[132usize],
        &COUNTRIES_DATA.0[133usize],
        &COUNTRIES_DATA.0[134usize],
        &COUNTRIES_DATA.0[135usize],
        &COUNTRIES_DATA.0[136usize],
        &COUNTRIES_DATA.0[137usize],
        &COUNTRIES_DATA.0[138usize],
        &COUNTRIES_DATA.0[139usize],
        &COUNTRIES_DATA.0[140usize],
        &COUNTRIES_DATA.0[141usize],
        &COUNTRIES_DATA.0[142usize],
        &COUNTRIES_DATA.0[143usize],
        &COUNTRIES_DATA.0[144usize],
        &COUNTRIES_DATA.0[145usize],
        &COUNTRIES_DATA.0[146usize],
        &COUNTRIES_DATA.0[147usize],
        &COUNTRIES_DATA.0[148usize],
        &COUNTRIES_DATA.0[149usize],
        &COUNTRIES_DATA.0[150usize],
        &COUNTRIES_DATA.0[151usize],
        &COUNTRIES_DATA.0[152usize],
        &COUNTRIES_DATA.0[153usize],
        &COUNTRIES_DATA.0[154usize],
        &COUNTRIES_DATA.0[155usize],
        &COUNTRIES_DATA.0[156usize],
        &COUNTRIES_DATA.0[157usize],
        &COUNTRIES_DATA.0[158usize],
        &COUNTRIES_DATA.0[159usize],
        &COUNTRIES_DATA.0[160usize],
        &COUNTRIES_DATA.0[161usize],
        &COUNTRIES_DATA.0[162usize],
        &COUNTRIES_DATA.0[163usize],
        &COUNTRIES_DATA.0[164usize],
        &COUNTRIES_DATA.0[165usize],
        &COUNTRIES_DATA.0[166usize],
        &COUNTRIES_DATA.0[167usize],
        &COUNTRIES_DATA.0[168usize],
        &COUNTRIES_DATA.0[169usize],
        &COUNTRIES_DATA.0[170usize],
        &COUNTRIES_DATA.0[171usize],
        &COUNTRIES_DATA.0[172usize],
        &COUNTRIES_DATA.0[173usize],
        &COUNTRIES_DATA.0[174usize],
        &COUNTRIES_DATA.0[175usize],
        &COUNTRIES_DATA.0[176usize],
        &COUNTRIES_DATA.0[177usize],
        &COUNTRIES_DATA.0[178usize],
        &COUNTRIES_DATA.0[179usize],
        &COUNTRIES_DATA.0[180usize],
        &COUNTRIES_DATA.0[181usize],
        &COUNTRIES_DATA.0[182usize],
        &COUNTRIES_DATA.0[183usize],
        &COUNTRIES_DATA.0[184usize],
        &COUNTRIES_DATA.0[185usize],
        &COUNTRIES_DATA.0[186usize],
        &COUNTRIES_DATA.0[187usize],
        &COUNTRIES_DATA.0[188usize],
        &COUNTRIES_DATA.0[189usize],
        &COUNTRIES_DATA.0[190usize],
        &COUNTRIES_DATA.0[191usize],
        &COUNTRIES_DATA.0[192usize],
        &COUNTRIES_DATA.0[193usize],
        &COUNTRIES_DATA.0[194usize],
        &COUNTRIES_DATA.0[195usize],
        &COUNTRIES_DATA.0[196usize],
        &COUNTRIES_DATA.0[197usize],
        &COUNTRIES_DATA.0[198usize],
        &COUNTRIES_DATA.0[199usize],
        &COUNTRIES_DATA.0[200usize],
        &COUNTRIES_DATA.0[201usize],
        &COUNTRIES_DATA.0[202usize],
        &COUNTRIES_DATA.0[203usize],
        &COUNTRIES_DATA.0[204usize],
        &COUNTRIES_DATA.0[205usize],
        &COUNTRIES_DATA.0[206usize],
        &COUNTRIES_DATA.0[207usize],
        &COUNTRIES_DATA.0[208usize],
        &COUNTRIES_DATA.0[209usize],
        &COUNTRIES_DATA.0[210usize],
        &COUNTRIES_DATA.0[211usize],
        &COUNTRIES_DATA.0[212usize],
        &COUNTRIES_DATA.0[213usize],
        &COUNTRIES_DATA.0[214usize],
        &COUNTRIES_DATA.0[215usize],
        &COUNTRIES_DATA.0[216usize],
        &COUNTRIES_DATA.0[217usize],
        &COUNTRIES_DATA.0[218usize],
        &COUNTRIES_DATA.0[219usize],
        &COUNTRIES_DATA.0[220usize],
        &COUNTRIES_DATA.0[221usize],
        &COUNTRIES_DATA.0[222usize],
        &COUNTRIES_DATA.0[223usize],
        &COUNTRIES_DATA.0[224usize],
        &COUNTRIES_DATA.0[225usize],
        &COUNTRIES_DATA.0[226usize],
        &COUNTRIES_DATA.0[227usize],
        &COUNTRIES_DATA.0[228usize],
        &COUNTRIES_DATA.0[229usize],
        &COUNTRIES_DATA.0[230usize],
        &COUNTRIES_DATA.0[231usize],
        &COUNTRIES_DATA.0[232usize],
        &COUNTRIES_DATA.0[233usize],
        &COUNTRIES_DATA.0[234usize],
        &COUNTRIES_DATA.0[235usize],
        &COUNTRIES_DATA.0[236usize],
        &COUNTRIES_DATA.0[237usize],
        &COUNTRIES_DATA.0[238usize],
        &COUNTRIES_DATA.0[239usize],
        &COUNTRIES_DATA.0[240usize],
        &COUNTRIES_DATA.0[241usize],
        &COUNTRIES_DATA.0[242usize],
        &COUNTRIES_DATA.0[243usize],
        &COUNTRIES_DATA.0[244usize],
        &COUNTRIES_DATA.0[245usize],
        &COUNTRIES_DATA.0[246usize],
        &COUNTRIES_DATA.0[247usize],
        &COUNTRIES_DATA.0[248usize],
        &COUNTRIES_DATA.0[249usize],
    ]
}
#[rustfmt::skip]
impl CountryKind {
    /// Data about this specific country
    pub fn data(self) -> &'static Country {
        match self {
            Self::Aruba => &COUNTRIES_DATA.0[0usize],
            Self::Afghanistan => &COUNTRIES_DATA.0[1usize],
            Self::Angola => &COUNTRIES_DATA.0[2usize],
            Self::Anguilla => &COUNTRIES_DATA.0[3usize],
            Self::AlandIslands => &COUNTRIES_DATA.0[4usize],
            Self::Albania => &COUNTRIES_DATA.0[5usize],
            Self::Andorra => &COUNTRIES_DATA.0[6usize],
            Self::UnitedArabEmirates => &COUNTRIES_DATA.0[7usize],
            Self::Argentina => &COUNTRIES_DATA.0[8usize],
            Self::Armenia => &COUNTRIES_DATA.0[9usize],
            Self::AmericanSamoa => &COUNTRIES_DATA.0[10usize],
            Self::Antarctica => &COUNTRIES_DATA.0[11usize],
            Self::FrenchSouthernAndAntarcticLands => &COUNTRIES_DATA.0[12usize],
            Self::AntiguaAndBarbuda => &COUNTRIES_DATA.0[13usize],
            Self::Australia => &COUNTRIES_DATA.0[14usize],
            Self::Austria => &COUNTRIES_DATA.0[15usize],
            Self::Azerbaijan => &COUNTRIES_DATA.0[16usize],
            Self::Burundi => &COUNTRIES_DATA.0[17usize],
            Self::Belgium => &COUNTRIES_DATA.0[18usize],
            Self::Benin => &COUNTRIES_DATA.0[19usize],
            Self::BurkinaFaso => &COUNTRIES_DATA.0[20usize],
            Self::Bangladesh => &COUNTRIES_DATA.0[21usize],
            Self::Bulgaria => &COUNTRIES_DATA.0[22usize],
            Self::Bahrain => &COUNTRIES_DATA.0[23usize],
            Self::Bahamas => &COUNTRIES_DATA.0[24usize],
            Self::BosniaAndHerzegovina => &COUNTRIES_DATA.0[25usize],
            Self::SaintBarthelemy => &COUNTRIES_DATA.0[26usize],
            Self::SaintHelenaAscensionAndTristanDaCunha => &COUNTRIES_DATA.0[27usize],
            Self::Belarus => &COUNTRIES_DATA.0[28usize],
            Self::Belize => &COUNTRIES_DATA.0[29usize],
            Self::Bermuda => &COUNTRIES_DATA.0[30usize],
            Self::Bolivia => &COUNTRIES_DATA.0[31usize],
            Self::CaribbeanNetherlands => &COUNTRIES_DATA.0[32usize],
            Self::Brazil => &COUNTRIES_DATA.0[33usize],
            Self::Barbados => &COUNTRIES_DATA.0[34usize],
            Self::Brunei => &COUNTRIES_DATA.0[35usize],
            Self::Bhutan => &COUNTRIES_DATA.0[36usize],
            Self::BouvetIsland => &COUNTRIES_DATA.0[37usize],
            Self::Botswana => &COUNTRIES_DATA.0[38usize],
            Self::CentralAfricanRepublic => &COUNTRIES_DATA.0[39usize],
            Self::Canada => &COUNTRIES_DATA.0[40usize],
            Self::CocosKeelingIslands => &COUNTRIES_DATA.0[41usize],
            Self::Switzerland => &COUNTRIES_DATA.0[42usize],
            Self::Chile => &COUNTRIES_DATA.0[43usize],
            Self::China => &COUNTRIES_DATA.0[44usize],
            Self::IvoryCoast => &COUNTRIES_DATA.0[45usize],
            Self::Cameroon => &COUNTRIES_DATA.0[46usize],
            Self::DrCongo => &COUNTRIES_DATA.0[47usize],
            Self::RepublicOfTheCongo => &COUNTRIES_DATA.0[48usize],
            Self::CookIslands => &COUNTRIES_DATA.0[49usize],
            Self::Colombia => &COUNTRIES_DATA.0[50usize],
            Self::Comoros => &COUNTRIES_DATA.0[51usize],
            Self::CapeVerde => &COUNTRIES_DATA.0[52usize],
            Self::CostaRica => &COUNTRIES_DATA.0[53usize],
            Self::Cuba => &COUNTRIES_DATA.0[54usize],
            Self::Curacao => &COUNTRIES_DATA.0[55usize],
            Self::ChristmasIsland => &COUNTRIES_DATA.0[56usize],
            Self::CaymanIslands => &COUNTRIES_DATA.0[57usize],
            Self::Cyprus => &COUNTRIES_DATA.0[58usize],
            Self::Czechia => &COUNTRIES_DATA.0[59usize],
            Self::Germany => &COUNTRIES_DATA.0[60usize],
            Self::Djibouti => &COUNTRIES_DATA.0[61usize],
            Self::Dominica => &COUNTRIES_DATA.0[62usize],
            Self::Denmark => &COUNTRIES_DATA.0[63usize],
            Self::DominicanRepublic => &COUNTRIES_DATA.0[64usize],
            Self::Algeria => &COUNTRIES_DATA.0[65usize],
            Self::Ecuador => &COUNTRIES_DATA.0[66usize],
            Self::Egypt => &COUNTRIES_DATA.0[67usize],
            Self::Eritrea => &COUNTRIES_DATA.0[68usize],
            Self::WesternSahara => &COUNTRIES_DATA.0[69usize],
            Self::Spain => &COUNTRIES_DATA.0[70usize],
            Self::Estonia => &COUNTRIES_DATA.0[71usize],
            Self::Ethiopia => &COUNTRIES_DATA.0[72usize],
            Self::Finland => &COUNTRIES_DATA.0[73usize],
            Self::Fiji => &COUNTRIES_DATA.0[74usize],
            Self::FalklandIslands => &COUNTRIES_DATA.0[75usize],
            Self::France => &COUNTRIES_DATA.0[76usize],
            Self::FaroeIslands => &COUNTRIES_DATA.0[77usize],
            Self::Micronesia => &COUNTRIES_DATA.0[78usize],
            Self::Gabon => &COUNTRIES_DATA.0[79usize],
            Self::UnitedKingdom => &COUNTRIES_DATA.0[80usize],
            Self::Georgia => &COUNTRIES_DATA.0[81usize],
            Self::Guernsey => &COUNTRIES_DATA.0[82usize],
            Self::Ghana => &COUNTRIES_DATA.0[83usize],
            Self::Gibraltar => &COUNTRIES_DATA.0[84usize],
            Self::Guinea => &COUNTRIES_DATA.0[85usize],
            Self::Guadeloupe => &COUNTRIES_DATA.0[86usize],
            Self::Gambia => &COUNTRIES_DATA.0[87usize],
            Self::GuineaBissau => &COUNTRIES_DATA.0[88usize],
            Self::EquatorialGuinea => &COUNTRIES_DATA.0[89usize],
            Self::Greece => &COUNTRIES_DATA.0[90usize],
            Self::Grenada => &COUNTRIES_DATA.0[91usize],
            Self::Greenland => &COUNTRIES_DATA.0[92usize],
            Self::Guatemala => &COUNTRIES_DATA.0[93usize],
            Self::FrenchGuiana => &COUNTRIES_DATA.0[94usize],
            Self::Guam => &COUNTRIES_DATA.0[95usize],
            Self::Guyana => &COUNTRIES_DATA.0[96usize],
            Self::HongKong => &COUNTRIES_DATA.0[97usize],
            Self::HeardIslandAndMcDonaldIslands => &COUNTRIES_DATA.0[98usize],
            Self::Honduras => &COUNTRIES_DATA.0[99usize],
            Self::Croatia => &COUNTRIES_DATA.0[100usize],
            Self::Haiti => &COUNTRIES_DATA.0[101usize],
            Self::Hungary => &COUNTRIES_DATA.0[102usize],
            Self::Indonesia => &COUNTRIES_DATA.0[103usize],
            Self::IsleOfMan => &COUNTRIES_DATA.0[104usize],
            Self::India => &COUNTRIES_DATA.0[105usize],
            Self::BritishIndianOceanTerritory => &COUNTRIES_DATA.0[106usize],
            Self::Ireland => &COUNTRIES_DATA.0[107usize],
            Self::Iran => &COUNTRIES_DATA.0[108usize],
            Self::Iraq => &COUNTRIES_DATA.0[109usize],
            Self::Iceland => &COUNTRIES_DATA.0[110usize],
            Self::Israel => &COUNTRIES_DATA.0[111usize],
            Self::Italy => &COUNTRIES_DATA.0[112usize],
            Self::Jamaica => &COUNTRIES_DATA.0[113usize],
            Self::Jersey => &COUNTRIES_DATA.0[114usize],
            Self::Jordan => &COUNTRIES_DATA.0[115usize],
            Self::Japan => &COUNTRIES_DATA.0[116usize],
            Self::Kazakhstan => &COUNTRIES_DATA.0[117usize],
            Self::Kenya => &COUNTRIES_DATA.0[118usize],
            Self::Kyrgyzstan => &COUNTRIES_DATA.0[119usize],
            Self::Cambodia => &COUNTRIES_DATA.0[120usize],
            Self::Kiribati => &COUNTRIES_DATA.0[121usize],
            Self::SaintKittsAndNevis => &COUNTRIES_DATA.0[122usize],
            Self::SouthKorea => &COUNTRIES_DATA.0[123usize],
            Self::Kosovo => &COUNTRIES_DATA.0[124usize],
            Self::Kuwait => &COUNTRIES_DATA.0[125usize],
            Self::Laos => &COUNTRIES_DATA.0[126usize],
            Self::Lebanon => &COUNTRIES_DATA.0[127usize],
            Self::Liberia => &COUNTRIES_DATA.0[128usize],
            Self::Libya => &COUNTRIES_DATA.0[129usize],
            Self::SaintLucia => &COUNTRIES_DATA.0[130usize],
            Self::Liechtenstein => &COUNTRIES_DATA.0[131usize],
            Self::SriLanka => &COUNTRIES_DATA.0[132usize],
            Self::Lesotho => &COUNTRIES_DATA.0[133usize],
            Self::Lithuania => &COUNTRIES_DATA.0[134usize],
            Self::Luxembourg => &COUNTRIES_DATA.0[135usize],
            Self::Latvia => &COUNTRIES_DATA.0[136usize],
            Self::Macau => &COUNTRIES_DATA.0[137usize],
            Self::SaintMartin => &COUNTRIES_DATA.0[138usize],
            Self::Morocco => &COUNTRIES_DATA.0[139usize],
            Self::Monaco => &COUNTRIES_DATA.0[140usize],
            Self::Moldova => &COUNTRIES_DATA.0[141usize],
            Self::Madagascar => &COUNTRIES_DATA.0[142usize],
            Self::Maldives => &COUNTRIES_DATA.0[143usize],
            Self::Mexico => &COUNTRIES_DATA.0[144usize],
            Self::MarshallIslands => &COUNTRIES_DATA.0[145usize],
            Self::NorthMacedonia => &COUNTRIES_DATA.0[146usize],
            Self::Mali => &COUNTRIES_DATA.0[147usize],
            Self::Malta => &COUNTRIES_DATA.0[148usize],
            Self::Myanmar => &COUNTRIES_DATA.0[149usize],
            Self::Montenegro => &COUNTRIES_DATA.0[150usize],
            Self::Mongolia => &COUNTRIES_DATA.0[151usize],
            Self::NorthernMarianaIslands => &COUNTRIES_DATA.0[152usize],
            Self::Mozambique => &COUNTRIES_DATA.0[153usize],
            Self::Mauritania => &COUNTRIES_DATA.0[154usize],
            Self::Montserrat => &COUNTRIES_DATA.0[155usize],
            Self::Martinique => &COUNTRIES_DATA.0[156usize],
            Self::Mauritius => &COUNTRIES_DATA.0[157usize],
            Self::Malawi => &COUNTRIES_DATA.0[158usize],
            Self::Malaysia => &COUNTRIES_DATA.0[159usize],
            Self::Mayotte => &COUNTRIES_DATA.0[160usize],
            Self::Namibia => &COUNTRIES_DATA.0[161usize],
            Self::NewCaledonia => &COUNTRIES_DATA.0[162usize],
            Self::Niger => &COUNTRIES_DATA.0[163usize],
            Self::NorfolkIsland => &COUNTRIES_DATA.0[164usize],
            Self::Nigeria => &COUNTRIES_DATA.0[165usize],
            Self::Nicaragua => &COUNTRIES_DATA.0[166usize],
            Self::Niue => &COUNTRIES_DATA.0[167usize],
            Self::Netherlands => &COUNTRIES_DATA.0[168usize],
            Self::Norway => &COUNTRIES_DATA.0[169usize],
            Self::Nepal => &COUNTRIES_DATA.0[170usize],
            Self::Nauru => &COUNTRIES_DATA.0[171usize],
            Self::NewZealand => &COUNTRIES_DATA.0[172usize],
            Self::Oman => &COUNTRIES_DATA.0[173usize],
            Self::Pakistan => &COUNTRIES_DATA.0[174usize],
            Self::Panama => &COUNTRIES_DATA.0[175usize],
            Self::PitcairnIslands => &COUNTRIES_DATA.0[176usize],
            Self::Peru => &COUNTRIES_DATA.0[177usize],
            Self::Philippines => &COUNTRIES_DATA.0[178usize],
            Self::Palau => &COUNTRIES_DATA.0[179usize],
            Self::PapuaNewGuinea => &COUNTRIES_DATA.0[180usize],
            Self::Poland => &COUNTRIES_DATA.0[181usize],
            Self::PuertoRico => &COUNTRIES_DATA.0[182usize],
            Self::NorthKorea => &COUNTRIES_DATA.0[183usize],
            Self::Portugal => &COUNTRIES_DATA.0[184usize],
            Self::Paraguay => &COUNTRIES_DATA.0[185usize],
            Self::Palestine => &COUNTRIES_DATA.0[186usize],
            Self::FrenchPolynesia => &COUNTRIES_DATA.0[187usize],
            Self::Qatar => &COUNTRIES_DATA.0[188usize],
            Self::Reunion => &COUNTRIES_DATA.0[189usize],
            Self::Romania => &COUNTRIES_DATA.0[190usize],
            Self::Russia => &COUNTRIES_DATA.0[191usize],
            Self::Rwanda => &COUNTRIES_DATA.0[192usize],
            Self::SaudiArabia => &COUNTRIES_DATA.0[193usize],
            Self::Sudan => &COUNTRIES_DATA.0[194usize],
            Self::Senegal => &COUNTRIES_DATA.0[195usize],
            Self::Singapore => &COUNTRIES_DATA.0[196usize],
            Self::SouthGeorgia => &COUNTRIES_DATA.0[197usize],
            Self::SvalbardAndJanMayen => &COUNTRIES_DATA.0[198usize],
            Self::SolomonIslands => &COUNTRIES_DATA.0[199usize],
            Self::SierraLeone => &COUNTRIES_DATA.0[200usize],
            Self::ElSalvador => &COUNTRIES_DATA.0[201usize],
            Self::SanMarino => &COUNTRIES_DATA.0[202usize],
            Self::Somalia => &COUNTRIES_DATA.0[203usize],
            Self::SaintPierreAndMiquelon => &COUNTRIES_DATA.0[204usize],
            Self::Serbia => &COUNTRIES_DATA.0[205usize],
            Self::SouthSudan => &COUNTRIES_DATA.0[206usize],
            Self::SaoTomeAndPrincipe => &COUNTRIES_DATA.0[207usize],
            Self::Suriname => &COUNTRIES_DATA.0[208usize],
            Self::Slovakia => &COUNTRIES_DATA.0[209usize],
            Self::Slovenia => &COUNTRIES_DATA.0[210usize],
            Self::Sweden => &COUNTRIES_DATA.0[211usize],
            Self::Eswatini => &COUNTRIES_DATA.0[212usize],
            Self::SintMaarten => &COUNTRIES_DATA.0[213usize],
            Self::Seychelles => &COUNTRIES_DATA.0[214usize],
            Self::Syria => &COUNTRIES_DATA.0[215usize],
            Self::TurksAndCaicosIslands => &COUNTRIES_DATA.0[216usize],
            Self::Chad => &COUNTRIES_DATA.0[217usize],
            Self::Togo => &COUNTRIES_DATA.0[218usize],
            Self::Thailand => &COUNTRIES_DATA.0[219usize],
            Self::Tajikistan => &COUNTRIES_DATA.0[220usize],
            Self::Tokelau => &COUNTRIES_DATA.0[221usize],
            Self::Turkmenistan => &COUNTRIES_DATA.0[222usize],
            Self::TimorLeste => &COUNTRIES_DATA.0[223usize],
            Self::Tonga => &COUNTRIES_DATA.0[224usize],
            Self::TrinidadAndTobago => &COUNTRIES_DATA.0[225usize],
            Self::Tunisia => &COUNTRIES_DATA.0[226usize],
            Self::Turkey => &COUNTRIES_DATA.0[227usize],
            Self::Tuvalu => &COUNTRIES_DATA.0[228usize],
            Self::Taiwan => &COUNTRIES_DATA.0[229usize],
            Self::Tanzania => &COUNTRIES_DATA.0[230usize],
            Self::Uganda => &COUNTRIES_DATA.0[231usize],
            Self::Ukraine => &COUNTRIES_DATA.0[232usize],
            Self::UnitedStatesMinorOutlyingIslands => &COUNTRIES_DATA.0[233usize],
            Self::Uruguay => &COUNTRIES_DATA.0[234usize],
            Self::UnitedStates => &COUNTRIES_DATA.0[235usize],
            Self::Uzbekistan => &COUNTRIES_DATA.0[236usize],
            Self::VaticanCity => &COUNTRIES_DATA.0[237usize],
            Self::SaintVincentAndTheGrenadines => &COUNTRIES_DATA.0[238usize],
            Self::Venezuela => &COUNTRIES_DATA.0[239usize],
            Self::BritishVirginIslands => &COUNTRIES_DATA.0[240usize],
            Self::UnitedStatesVirginIslands => &COUNTRIES_DATA.0[241usize],
            Self::Vietnam => &COUNTRIES_DATA.0[242usize],
            Self::Vanuatu => &COUNTRIES_DATA.0[243usize],
            Self::WallisAndFutuna => &COUNTRIES_DATA.0[244usize],
            Self::Samoa => &COUNTRIES_DATA.0[245usize],
            Self::Yemen => &COUNTRIES_DATA.0[246usize],
            Self::SouthAfrica => &COUNTRIES_DATA.0[247usize],
            Self::Zambia => &COUNTRIES_DATA.0[248usize],
            Self::Zimbabwe => &COUNTRIES_DATA.0[249usize],
        }
    }
}
#[rustfmt::skip]
impl Country {
    pub fn kind(&self) -> CountryKind {
        match self.country_id {
            0usize => CountryKind::Aruba,
            1usize => CountryKind::Afghanistan,
            2usize => CountryKind::Angola,
            3usize => CountryKind::Anguilla,
            4usize => CountryKind::AlandIslands,
            5usize => CountryKind::Albania,
            6usize => CountryKind::Andorra,
            7usize => CountryKind::UnitedArabEmirates,
            8usize => CountryKind::Argentina,
            9usize => CountryKind::Armenia,
            10usize => CountryKind::AmericanSamoa,
            11usize => CountryKind::Antarctica,
            12usize => CountryKind::FrenchSouthernAndAntarcticLands,
            13usize => CountryKind::AntiguaAndBarbuda,
            14usize => CountryKind::Australia,
            15usize => CountryKind::Austria,
            16usize => CountryKind::Azerbaijan,
            17usize => CountryKind::Burundi,
            18usize => CountryKind::Belgium,
            19usize => CountryKind::Benin,
            20usize => CountryKind::BurkinaFaso,
            21usize => CountryKind::Bangladesh,
            22usize => CountryKind::Bulgaria,
            23usize => CountryKind::Bahrain,
            24usize => CountryKind::Bahamas,
            25usize => CountryKind::BosniaAndHerzegovina,
            26usize => CountryKind::SaintBarthelemy,
            27usize => CountryKind::SaintHelenaAscensionAndTristanDaCunha,
            28usize => CountryKind::Belarus,
            29usize => CountryKind::Belize,
            30usize => CountryKind::Bermuda,
            31usize => CountryKind::Bolivia,
            32usize => CountryKind::CaribbeanNetherlands,
            33usize => CountryKind::Brazil,
            34usize => CountryKind::Barbados,
            35usize => CountryKind::Brunei,
            36usize => CountryKind::Bhutan,
            37usize => CountryKind::BouvetIsland,
            38usize => CountryKind::Botswana,
            39usize => CountryKind::CentralAfricanRepublic,
            40usize => CountryKind::Canada,
            41usize => CountryKind::CocosKeelingIslands,
            42usize => CountryKind::Switzerland,
            43usize => CountryKind::Chile,
            44usize => CountryKind::China,
            45usize => CountryKind::IvoryCoast,
            46usize => CountryKind::Cameroon,
            47usize => CountryKind::DrCongo,
            48usize => CountryKind::RepublicOfTheCongo,
            49usize => CountryKind::CookIslands,
            50usize => CountryKind::Colombia,
            51usize => CountryKind::Comoros,
            52usize => CountryKind::CapeVerde,
            53usize => CountryKind::CostaRica,
            54usize => CountryKind::Cuba,
            55usize => CountryKind::Curacao,
            56usize => CountryKind::ChristmasIsland,
            57usize => CountryKind::CaymanIslands,
            58usize => CountryKind::Cyprus,
            59usize => CountryKind::Czechia,
            60usize => CountryKind::Germany,
            61usize => CountryKind::Djibouti,
            62usize => CountryKind::Dominica,
            63usize => CountryKind::Denmark,
            64usize => CountryKind::DominicanRepublic,
            65usize => CountryKind::Algeria,
            66usize => CountryKind::Ecuador,
            67usize => CountryKind::Egypt,
            68usize => CountryKind::Eritrea,
            69usize => CountryKind::WesternSahara,
            70usize => CountryKind::Spain,
            71usize => CountryKind::Estonia,
            72usize => CountryKind::Ethiopia,
            73usize => CountryKind::Finland,
            74usize => CountryKind::Fiji,
            75usize => CountryKind::FalklandIslands,
            76usize => CountryKind::France,
            77usize => CountryKind::FaroeIslands,
            78usize => CountryKind::Micronesia,
            79usize => CountryKind::Gabon,
            80usize => CountryKind::UnitedKingdom,
            81usize => CountryKind::Georgia,
            82usize => CountryKind::Guernsey,
            83usize => CountryKind::Ghana,
            84usize => CountryKind::Gibraltar,
            85usize => CountryKind::Guinea,
            86usize => CountryKind::Guadeloupe,
            87usize => CountryKind::Gambia,
            88usize => CountryKind::GuineaBissau,
            89usize => CountryKind::EquatorialGuinea,
            90usize => CountryKind::Greece,
            91usize => CountryKind::Grenada,
            92usize => CountryKind::Greenland,
            93usize => CountryKind::Guatemala,
            94usize => CountryKind::FrenchGuiana,
            95usize => CountryKind::Guam,
            96usize => CountryKind::Guyana,
            97usize => CountryKind::HongKong,
            98usize => CountryKind::HeardIslandAndMcDonaldIslands,
            99usize => CountryKind::Honduras,
            100usize => CountryKind::Croatia,
            101usize => CountryKind::Haiti,
            102usize => CountryKind::Hungary,
            103usize => CountryKind::Indonesia,
            104usize => CountryKind::IsleOfMan,
            105usize => CountryKind::India,
            106usize => CountryKind::BritishIndianOceanTerritory,
            107usize => CountryKind::Ireland,
            108usize => CountryKind::Iran,
            109usize => CountryKind::Iraq,
            110usize => CountryKind::Iceland,
            111usize => CountryKind::Israel,
            112usize => CountryKind::Italy,
            113usize => CountryKind::Jamaica,
            114usize => CountryKind::Jersey,
            115usize => CountryKind::Jordan,
            116usize => CountryKind::Japan,
            117usize => CountryKind::Kazakhstan,
            118usize => CountryKind::Kenya,
            119usize => CountryKind::Kyrgyzstan,
            120usize => CountryKind::Cambodia,
            121usize => CountryKind::Kiribati,
            122usize => CountryKind::SaintKittsAndNevis,
            123usize => CountryKind::SouthKorea,
            124usize => CountryKind::Kosovo,
            125usize => CountryKind::Kuwait,
            126usize => CountryKind::Laos,
            127usize => CountryKind::Lebanon,
            128usize => CountryKind::Liberia,
            129usize => CountryKind::Libya,
            130usize => CountryKind::SaintLucia,
            131usize => CountryKind::Liechtenstein,
            132usize => CountryKind::SriLanka,
            133usize => CountryKind::Lesotho,
            134usize => CountryKind::Lithuania,
            135usize => CountryKind::Luxembourg,
            136usize => CountryKind::Latvia,
            137usize => CountryKind::Macau,
            138usize => CountryKind::SaintMartin,
            139usize => CountryKind::Morocco,
            140usize => CountryKind::Monaco,
            141usize => CountryKind::Moldova,
            142usize => CountryKind::Madagascar,
            143usize => CountryKind::Maldives,
            144usize => CountryKind::Mexico,
            145usize => CountryKind::MarshallIslands,
            146usize => CountryKind::NorthMacedonia,
            147usize => CountryKind::Mali,
            148usize => CountryKind::Malta,
            149usize => CountryKind::Myanmar,
            150usize => CountryKind::Montenegro,
            151usize => CountryKind::Mongolia,
            152usize => CountryKind::NorthernMarianaIslands,
            153usize => CountryKind::Mozambique,
            154usize => CountryKind::Mauritania,
            155usize => CountryKind::Montserrat,
            156usize => CountryKind::Martinique,
            157usize => CountryKind::Mauritius,
            158usize => CountryKind::Malawi,
            159usize => CountryKind::Malaysia,
            160usize => CountryKind::Mayotte,
            161usize => CountryKind::Namibia,
            162usize => CountryKind::NewCaledonia,
            163usize => CountryKind::Niger,
            164usize => CountryKind::NorfolkIsland,
            165usize => CountryKind::Nigeria,
            166usize => CountryKind::Nicaragua,
            167usize => CountryKind::Niue,
            168usize => CountryKind::Netherlands,
            169usize => CountryKind::Norway,
            170usize => CountryKind::Nepal,
            171usize => CountryKind::Nauru,
            172usize => CountryKind::NewZealand,
            173usize => CountryKind::Oman,
            174usize => CountryKind::Pakistan,
            175usize => CountryKind::Panama,
            176usize => CountryKind::PitcairnIslands,
            177usize => CountryKind::Peru,
            178usize => CountryKind::Philippines,
            179usize => CountryKind::Palau,
            180usize => CountryKind::PapuaNewGuinea,
            181usize => CountryKind::Poland,
            182usize => CountryKind::PuertoRico,
            183usize => CountryKind::NorthKorea,
            184usize => CountryKind::Portugal,
            185usize => CountryKind::Paraguay,
            186usize => CountryKind::Palestine,
            187usize => CountryKind::FrenchPolynesia,
            188usize => CountryKind::Qatar,
            189usize => CountryKind::Reunion,
            190usize => CountryKind::Romania,
            191usize => CountryKind::Russia,
            192usize => CountryKind::Rwanda,
            193usize => CountryKind::SaudiArabia,
            194usize => CountryKind::Sudan,
            195usize => CountryKind::Senegal,
            196usize => CountryKind::Singapore,
            197usize => CountryKind::SouthGeorgia,
            198usize => CountryKind::SvalbardAndJanMayen,
            199usize => CountryKind::SolomonIslands,
            200usize => CountryKind::SierraLeone,
            201usize => CountryKind::ElSalvador,
            202usize => CountryKind::SanMarino,
            203usize => CountryKind::Somalia,
            204usize => CountryKind::SaintPierreAndMiquelon,
            205usize => CountryKind::Serbia,
            206usize => CountryKind::SouthSudan,
            207usize => CountryKind::SaoTomeAndPrincipe,
            208usize => CountryKind::Suriname,
            209usize => CountryKind::Slovakia,
            210usize => CountryKind::Slovenia,
            211usize => CountryKind::Sweden,
            212usize => CountryKind::Eswatini,
            213usize => CountryKind::SintMaarten,
            214usize => CountryKind::Seychelles,
            215usize => CountryKind::Syria,
            216usize => CountryKind::TurksAndCaicosIslands,
            217usize => CountryKind::Chad,
            218usize => CountryKind::Togo,
            219usize => CountryKind::Thailand,
            220usize => CountryKind::Tajikistan,
            221usize => CountryKind::Tokelau,
            222usize => CountryKind::Turkmenistan,
            223usize => CountryKind::TimorLeste,
            224usize => CountryKind::Tonga,
            225usize => CountryKind::TrinidadAndTobago,
            226usize => CountryKind::Tunisia,
            227usize => CountryKind::Turkey,
            228usize => CountryKind::Tuvalu,
            229usize => CountryKind::Taiwan,
            230usize => CountryKind::Tanzania,
            231usize => CountryKind::Uganda,
            232usize => CountryKind::Ukraine,
            233usize => CountryKind::UnitedStatesMinorOutlyingIslands,
            234usize => CountryKind::Uruguay,
            235usize => CountryKind::UnitedStates,
            236usize => CountryKind::Uzbekistan,
            237usize => CountryKind::VaticanCity,
            238usize => CountryKind::SaintVincentAndTheGrenadines,
            239usize => CountryKind::Venezuela,
            240usize => CountryKind::BritishVirginIslands,
            241usize => CountryKind::UnitedStatesVirginIslands,
            242usize => CountryKind::Vietnam,
            243usize => CountryKind::Vanuatu,
            244usize => CountryKind::WallisAndFutuna,
            245usize => CountryKind::Samoa,
            246usize => CountryKind::Yemen,
            247usize => CountryKind::SouthAfrica,
            248usize => CountryKind::Zambia,
            249usize => CountryKind::Zimbabwe,
            _ => panic!("unknown `country_id`"),
        }
    }
}
