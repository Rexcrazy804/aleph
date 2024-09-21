#[test]
fn required_attributes() {
    use aleph::manifest::Manifest;

    let data = std::fs::read_to_string("./tests/sample_data/required_attrs.json")
        .expect("Failed to retreive sample data");
    let data: Manifest = serde_json::from_str(&data).expect(".w.");

    assert_eq!("1.0", data.version);
    assert_eq!("Simple demo", data.description);
    assert_eq!("https://some_homepage.com", data.homepage);
    assert_eq!("unlicensed", data.license);
}
