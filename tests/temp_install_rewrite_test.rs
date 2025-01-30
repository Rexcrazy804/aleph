use aleph::AlephConfig;

#[test]
fn installer_runner() {
    use aleph::manifest::Manifest;
    use aleph::scoopd::manifest_install::manifest_installer;
    use std::fs::read_to_string;

    const MANIFEST: &str = "./tests/sample_data/less.json";

    let manifest = read_to_string(MANIFEST).expect("Failed to read file");
    let manifest: Manifest = Manifest::parse(&manifest).expect("Failed to parse data");
    let pname = MANIFEST
        .split('/')
        .last()
        .unwrap()
        .split_once('.')
        .unwrap()
        .0;
    dbg!(pname);
    let config = AlephConfig::new();

    assert_eq!(Ok(()), manifest_installer(&config, &manifest, pname));
}
