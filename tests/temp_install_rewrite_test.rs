#[test]
fn installer_runner() {
    use aleph::manifest::Manifest;
    use aleph::scoopd::manifest_install::manifest_installer;
    use std::fs::read_to_string;

    const MANIFEST: &'static str = "./tests/sample_data/cowsay.json";

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
    assert_eq!(Ok(()), manifest_installer(&manifest, pname));
}
