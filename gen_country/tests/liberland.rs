use gen_country::Country;

#[test]
fn resolves_liberland_from_project_codes() {
    assert_eq!(Country::from_country_code("LIB"), Some(Country::Liberland));
    assert_eq!(Country::country_code3_from_country_code2("LL"), Some("LIB"));
}

#[test]
fn exposes_seeded_liberland_metadata() {
    assert_eq!(Country::Liberland.country_name(), "Liberland");
    assert_eq!(Country::Liberland.country_code2(), "LL");
    assert_eq!(Country::Liberland.country_code3(), "LIB");
    assert_eq!(Country::Liberland.population(), 63);
    assert_eq!(Country::Liberland.area_km(), 7.0);
    assert_eq!(
        Country::Liberland.currencies(),
        &[("LLD", "Liberland dollar", "LLD")]
    );
    assert_eq!(
        gen_country::status_note(Country::Liberland),
        Some("Self-proclaimed micronation on disputed territory between Croatia and Serbia"),
    );
}

#[test]
fn liberland_flag_matches_repo_generation_shape() {
    let flag_plain = Country::Liberland.flag_no_color();
    let flag_colored = Country::Liberland.flag();

    assert!(flag_plain.is_ascii());
    assert!(flag_colored.is_ascii());
    assert_eq!(flag_plain.lines().count(), 17);
    assert!(flag_plain.lines().all(|line| line.len() == 40));
    assert_eq!(Country::Liberland.palette().len(), 9);
}
