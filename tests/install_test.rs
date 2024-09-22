use aleph::{
    manifest::{Manifest, OneOrMany},
    powershell::utilities::{download_url, get_home_directory},
    zipper::unzip,
};

#[test]
#[ignore]
fn installation() {
    let manifest =
        std::fs::read_to_string("./tests/sample_data/cowsay.json").expect("Failed to read file");
    let manifest: Manifest = serde_json::from_str(&manifest).expect("Failed to parse data");

    //println!("{manifest:?}");
    let home_dir = dbg!(get_home_directory()
        .expect("Failed to get homeDIR")
        .trim()
        .to_owned());

    install_manifest(&manifest, &home_dir);
}

fn install_manifest(manifest: &Manifest, home_dir: &str) {
    let download_dir = home_dir.to_owned() + "/Downloads/";
    let extract_dir = home_dir.to_owned() + "/Documents/aleph/";

    let Some(url) = &manifest.url else {
        panic!("I think I should change the manifest to ensure that URL isn't optional")
    };

    let file_path = match url {
        OneOrMany::One(url) => OneOrMany::One(download_url(url, &download_dir)),
        OneOrMany::Many(urls) => OneOrMany::Many(
            urls.iter()
                .map(|url| download_url(url, &download_dir))
                .collect(),
        ),
    };

    println!("{file_path:?}");

    let extraction_output = match file_path {
        OneOrMany::One(file_path) => {
            let Some(OneOrMany::One(manifest_extract_dir)) = &manifest.extract_dir else {
                panic!("looks like you have multiple extract directories for a single url")
            };

            let Ok(file_path) = file_path else {
                panic!("FAILIURE");
            };

            // TODO: FIX THIS THIS
            //unzip(&file_path, &extract_dir, &manifest_extract_dir)
            OneOrMany::One(unzip(&file_path, &extract_dir))
        }

        OneOrMany::Many(file_paths) => {
            let Some(OneOrMany::Many(manifest_extract_dirs)) = &manifest.extract_dir else {
                panic!("looks like you have a single extract directory for multiple url")
                // NOTE: this might be recoverable, we might just have a make a
                // manifest_extract_dirs vector with x number of the same string where
                // x is the number of file_paths we've received from the urls
            };

            OneOrMany::Many(
                file_paths
                    .iter()
                    .zip(manifest_extract_dirs)
                    .map(|(file_path, m_extract_dir)| {
                        let Ok(file_path) = file_path else {
                            panic!("FAILIURE");
                        };

                        // TODO: FIX THIS
                        //unzip(&file_path.unwrap(), &extract_dir, &manifest_extract_dir)
                        unzip(file_path, &extract_dir)
                    })
                    .collect(),
            )
        }
    };
}
