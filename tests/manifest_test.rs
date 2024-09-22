use aleph::manifest::license::{CustomLicense, License};
use aleph::manifest::{Manifest, OneOrMany as OM, WayTooMany};
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
fn str_or_struct_attributes() {
    let data = fs::read_to_string("./tests/sample_data/str_or_struct_attrs.json")
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
    assert_eq!(
        WayTooMany::Many(vec![String::from("cowsay.ps1")]),
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
        Some(OM::Many(vec![
            String::from("https://www.irfanview.info/files/iview467_x64.zip"),
            String::from("https://www.irfanview.info/files/iview467_plugins_x64.zip")
        ])),
        data.clone().x86_64.unwrap().url
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
        Some(WayTooMany::TooMany(vec![WayTooMany::Many(vec![
            String::from("i_view32.exe"),
            String::from("irfanview")
        ])])),
        data.clone().x64.unwrap().bin
    );

    // TODO add some dummy data in the file for arm64 make test cases
    assert_eq!(None, data.clone().arm64);
}

#[test]
#[ignore] // we don't want this test to run by default [its expensive kinda .w.]
fn bulk_parse() {
    // hardcoding this to point to the main scoop bucket so that I can actually parse EVERYTHING
    let data_dir =
        fs::read_dir("Z:/home/rexies/temp/Main/bucket/").expect("Failed to read data directory");

    let mut file_count = 0;
    for data in data_dir {
        let Ok(data) = data else {
            continue;
        };

        file_count += 1;
        println!("{:?}", data.file_name());

        let data = fs::read_to_string(data.path()).expect("Failed to read file");
        let data: Manifest = serde_json::from_str(&data).unwrap();
        println!("desc: {}", data.description);

        // TEST: Shortcuts
        // TODO: this works, but I guess it may be better to write a seperate test
        //if let Some(shortcuts) = data.shortcuts {
        //    use aleph::manifest::shortcuts::Shortcuts;
        //    for shortcut in shortcuts {
        //        if let Shortcuts::Standard([path, label]) = shortcut {
        //            println!("Path: {path}\nLabel: {label}");
        //        }
        //    }
        //}
    }

    println!("parsed {file_count} files")
}
