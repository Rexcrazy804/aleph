use aleph::{
    manifest::{Manifest, OneOrMany},
    powershell::{
        installer::append_to_path,
        utilities::{download_url, get_home_directory},
    },
    zipper::unzip_alt,
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

    install_manifest(&manifest, &home_dir);

    let manifest =
        std::fs::read_to_string("./tests/sample_data/cowsay.json").expect("Failed to read file");
    let manifest: Manifest = serde_json::from_str(&manifest).expect("Failed to parse data");
    install_manifest(&manifest, &home_dir);
}

fn install_manifest(manifest: &Manifest, home_dir: &str) {
    let download_dir = home_dir.to_owned() + "/Downloads/";
    let extract_dir = home_dir.to_owned() + "/Documents/aleph/";

    let url = manifest.get_url();

    // TODO hash the downloads so that we can extract them
    // into <hash>-filename/ directories like nixos
    let file_path = match url {
        OneOrMany::One(url) => OneOrMany::One(download_url(&url, &download_dir)),
        OneOrMany::Many(urls) => OneOrMany::Many(
            urls.iter()
                .map(|url| download_url(url, &download_dir))
                .collect(),
        ),
    };

    println!("{file_path:?}");

    let extracted_dir = match file_path {
        OneOrMany::One(file_path) => {
            let mut manifest_extract_dir = None;
            if let Some(OneOrMany::One(dir)) = &manifest.extract_dir {
                manifest_extract_dir = Some(dir);
            };

            let Ok(file_path) = file_path else {
                panic!("FAILIURE");
            };

            let result = unzip_alt(&file_path, &extract_dir, manifest_extract_dir);
            OneOrMany::One(result)
        }

        OneOrMany::Many(file_paths) => {
            let result;
            if let Some(OneOrMany::Many(dirs)) = &manifest.extract_dir {
                result = file_paths
                    .iter()
                    .zip(dirs)
                    .map(|(file_path, m_extract_dir)| {
                        let Ok(file_path) = file_path else {
                            panic!("FAILIURE");
                        };

                        unzip_alt(file_path, &extract_dir, Some(m_extract_dir))
                    })
                    .collect();
            } else {
                result = file_paths
                    .iter()
                    .map(|file_path| {
                        let Ok(file_path) = file_path else {
                            panic!("FAILIURE");
                        };

                        unzip_alt(file_path, &extract_dir, None)
                    })
                    .collect()
            };
            OneOrMany::Many(result)
        }
    };

    // currently there is no real need to do this but once we start hashing our downloads we might
    // end up
    // WARN kinda hit a road bloack with shims, will leave it here for the time being
    // TODO do something about shims :')

    // NOTE for the time being we'll be using this
    let _ = match extracted_dir {
        OneOrMany::One(dir) => append_to_path(home_dir, &vec![dir]),
        OneOrMany::Many(dirs) => append_to_path(home_dir, &dirs),
    };
}
