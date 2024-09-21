#[test]
fn required_attributes() {
    use aleph::manifest::Manifest;

    let data = std::fs::read_to_string("./tests/sample_data/required_attrs.json")
        .expect("Failed to retreive sample data");
    let data: Manifest = serde_json::from_str(&data).expect(".w.");

    assert_eq!("1.0", data.version);
    assert_eq!("Simple demo", data.description);
    assert_eq!("https://some_homepage.com", data.homepage);
    //assert_eq!("unlicensed", data.license);
}

#[test]
fn str_or_struct_attributes() {
    use aleph::manifest::Manifest;
    use aleph::manifest::OneOrMany as OM;

    let data = std::fs::read_to_string("./tests/sample_data/str_or_struct_attrs.json")
        .expect("Failed to retreive sample data");
    let data: Manifest = serde_json::from_str(&data).expect("Failed to parse data\n");

    assert_eq!(
        OM::One("https://github.com/lukesampson/cowsay-psh/archive/master.zip".to_owned()),
        data.url.unwrap()
    );
    assert_eq!("cowsay-psh-master", data.extract_dir.unwrap());
    assert_eq!(OM::One(vec!["cowsay.ps1".to_string()]), data.bin.unwrap());
}

#[test]
fn architecture_attribute() {
    use aleph::manifest::Manifest;
    use aleph::manifest::OneOrMany as OM;

    fn tests() -> Option<()> {
        let data = std::fs::read_to_string("./tests/sample_data/irfanview.json")
            .expect("Failed to retreive sample data");
        let data: Manifest = serde_json::from_str(&data).expect("Failed to parse data\n");

        assert_eq!(
            OM::Many(vec![
                String::from("https://www.irfanview.info/files/iview467_x64.zip"),
                String::from("https://www.irfanview.info/files/iview467_plugins_x64.zip")
            ]),
            data.architecture?.x86_65?.url?
        );

        Some(())
    }

    tests();
}
