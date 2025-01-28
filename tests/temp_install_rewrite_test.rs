#[test]
fn installer_runner() {
    use aleph::manifest::Manifest;
    use aleph::scoopd::manifest_install::manifest_installer;
    use std::fs::read_to_string;

    const COWSAY_MANIFEST: &'static str = "./tests/sample_data/cowsay.json";

    let manifest = read_to_string(COWSAY_MANIFEST).expect("Failed to read file");
    let manifest: Manifest = Manifest::parse(&manifest).expect("Failed to parse data");

    if let Err(error) = manifest_installer(&manifest) {
        eprintln!("{error}")
    }

    assert_eq!(Ok(()), manifest_installer(&manifest));
}
