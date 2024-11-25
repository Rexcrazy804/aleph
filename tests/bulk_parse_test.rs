use aleph::manifest::Manifest;
use std::fs;

#[test]
#[ignore] // we don't want this test to run by default [its expensive kinda .w.]
fn bulk_parse() {
    // hardcoding this to point to the main scoop bucket so that I can actually parse EVERYTHING
    let data_dir = fs::read_dir("Z:/home/rexies/temp/ScoopMainBucket/bucket/")
        .expect("Failed to read data directory");

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
