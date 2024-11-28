use aleph::{
    manifest::Manifest, powershell::utilities::get_home_directory,
    scoopd::manifest_install::manifest_installer,
};

#[test]
#[ignore]
fn installation() {
    //println!("{manifest:?}");
    let home_dir = dbg!(get_home_directory()
        .expect("Failed to get homeDIR")
        .trim()
        .to_owned());

    let manifest =
        std::fs::read_to_string("./tests/sample_data/less.json").expect("Failed to read file");
    let manifest: Manifest = serde_json::from_str(&manifest).expect("Failed to parse data");

    manifest_installer(&manifest, &home_dir);

    let manifest =
        std::fs::read_to_string("./tests/sample_data/cowsay.json").expect("Failed to read file");
    let manifest: Manifest = serde_json::from_str(&manifest).expect("Failed to parse data");
    manifest_installer(&manifest, &home_dir);
}
