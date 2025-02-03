use aleph::manifest::license::{CustomLicense, License};
use aleph::manifest::{bin::Binary, Manifest, OneOrMany as OM};
use std::collections::VecDeque;
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
    let data = fs::read_to_string("./tests/sample_data/required_attrs.json")
        .expect("Failed to retreive sample data");
    let data: Manifest = serde_json::from_str(&data).expect(".w.");

    assert_eq!("1.0", data.version);
    assert_eq!("Simple demo", data.description);
    assert_eq!("https://some_homepage.com", data.homepage);
    assert_eq!(License::License(String::from("unlicensed")), data.license);
}

#[test]
fn cowsay_bin() {
    let data = fs::read_to_string("./tests/sample_data/cowsay.json")
        .expect("Failed to retreive sample data");
    let data: Manifest = serde_json::from_str(&data).expect("Failed to parse data\n");

    assert_eq!(
        OM::One("https://github.com/lukesampson/cowsay-psh/archive/master.zip".to_owned()),
        data.url.unwrap()
    );
    assert_eq!(
        OM::One(String::from("cowsay-psh-master")),
        data.extract_dir.unwrap()
    );
    println!("{:?}", data.bin);
    assert_eq!(
        Binary::Executables(vec![
            String::from("cowsay.ps1"),
            String::from("cowthink.ps1"),
        ]),
        data.bin.unwrap()
    );
}

#[test]
fn architecture_attribute() {
    let data = fs::read_to_string("./tests/sample_data/irfanview.json")
        .expect("Failed to retreive sample data");
    let data: Manifest = serde_json::from_str(&data).expect("Failed to parse data\n");

    assert_eq!(
        License::Custom(CustomLicense {
            identifier: Some(String::from("Freeware")),
            url: Some(String::from("https://www.irfanview.com/eula.htm")),
        }),
        data.license
    );

    let data = data.architecture.unwrap();
    assert_eq!(
        Some(OM::Many(VecDeque::from([
            String::from("https://www.irfanview.info/files/iview467_x64.zip"),
            String::from("https://www.irfanview.info/files/iview467_plugins_x64.zip")
        ]))),
        data.clone().x86_64.unwrap().url
    );
    assert_eq!(
        Some(OM::Many(VecDeque::from([
            String::from("https://www.irfanview.info/files/iview467.zip"),
            String::from("https://www.irfanview.info/files/iview467_plugins.zip")
        ]))),
        data.clone().x86.unwrap().url
    );
    assert_eq!(
        Some(OM::Many(VecDeque::from([
            String::from("75aeec57c780ae7ad6e15f750e34f62abedb1569efce1bfc2d6023d4a045f5a3"),
            String::from("9d62c7b44c8d83c617758d90d373b3dd25dfa9af90a45a2c9629c4998b35d29a")
        ]))),
        data.clone().x86.unwrap().hash
    );

    //"bin": [["i_view32.exe", "irfanview"]],
    assert_eq!(
        Binary::AliasedExecutables(vec![Binary::Executables(vec![
            String::from("i_view32.exe"),
            String::from("irfanview"),
        ])]),
        data.clone().x86.unwrap().bin.unwrap()
    );

    // TODO add some dummy data in the file for arm64 make test cases
    assert_eq!(None, data.clone().arm64);
}
