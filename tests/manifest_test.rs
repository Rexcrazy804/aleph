use std::fs;

// TODO: cover every possible attribute in as many different ways as possible

/* COVERED ATTRIBUTES
* version
* description
* homepage
* license
*   identifier
*   string
*
* bin
* [bin]
* [[bin]]
*
* url
* [url]
*
* [hash]
*
* Architecture:
*   64bit
*   32bit
*   TODO arm64
*/

#[test]
fn required_attributes() {
    use aleph::manifest::{License, Manifest};

    let data = fs::read_to_string("./tests/sample_data/required_attrs.json")
        .expect("Failed to retreive sample data");
    let data: Manifest = serde_json::from_str(&data).expect(".w.");

    assert_eq!("1.0", data.version);
    assert_eq!("Simple demo", data.description);
    assert_eq!("https://some_homepage.com", data.homepage);
    assert_eq!(License::License(String::from("unlicensed")), data.license);
}

#[test]
fn str_or_struct_attributes() {
    use aleph::manifest::Manifest;
    use aleph::manifest::OneOrMany as OM;

    let data = fs::read_to_string("./tests/sample_data/str_or_struct_attrs.json")
        .expect("Failed to retreive sample data");
    let data: Manifest = serde_json::from_str(&data).expect("Failed to parse data\n");

    assert_eq!(
        OM::One("https://github.com/lukesampson/cowsay-psh/archive/master.zip".to_owned()),
        data.url.unwrap()
    );
    assert_eq!("cowsay-psh-master", data.extract_dir.unwrap());
    assert_eq!(OM::Many(vec!["cowsay.ps1".to_string()]), data.bin.unwrap());
}

#[test]
fn architecture_attribute() {
    use aleph::manifest::{License, Manifest, CustomLicense};
    use aleph::manifest::OneOrMany as OM;

    let data = fs::read_to_string("./tests/sample_data/irfanview.json")
        .expect("Failed to retreive sample data");
    let data: Manifest = serde_json::from_str(&data).expect("Failed to parse data\n");

    assert_eq!(
        License::Custom(CustomLicense {
            identifier: String::from("Freeware"),
            url: String::from("https://www.irfanview.com/eula.htm"),
        }),
        data.license
    );

    let data = data.architecture.unwrap();
    assert_eq!(
        Some(OM::Many(vec![
            String::from("https://www.irfanview.info/files/iview467_x64.zip"),
            String::from("https://www.irfanview.info/files/iview467_plugins_x64.zip")
        ])),
        data.clone().x86_65.unwrap().url
    );
    assert_eq!(
        Some(OM::Many(vec![
            String::from("https://www.irfanview.info/files/iview467.zip"),
            String::from("https://www.irfanview.info/files/iview467_plugins.zip")
        ])),
        data.clone().x64.unwrap().url
    );
    assert_eq!(
        Some(OM::Many(vec![
            String::from("75aeec57c780ae7ad6e15f750e34f62abedb1569efce1bfc2d6023d4a045f5a3"),
            String::from("9d62c7b44c8d83c617758d90d373b3dd25dfa9af90a45a2c9629c4998b35d29a")
        ])),
        data.clone().x64.unwrap().hash
    );
    assert_eq!(
        Some(OM::TooMany(vec![vec![
            String::from("i_view32.exe"),
            String::from("irfanview")
        ]])),
        data.clone().x64.unwrap().bin
    );

    // TODO add some dummy data in the file for arm64 make test cases
    assert_eq!(None, data.clone().arm64);
}
