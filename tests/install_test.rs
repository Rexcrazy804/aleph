use aleph::{manifest::Manifest, scoopd::manifest_install::manifest_installer, AlephConfig};

#[test]
#[ignore]
fn general_installation_test() {
    use std::fs::read_to_string;

    const DEBUG_SKIP_COWSAY: bool = true;
    const DEBUG_SKIP_LESS: bool = true;
    const DEBUG_SKIP_FFMPEG: bool = false;

    const COWSAY_MANIFEST: &str = "./tests/sample_data/cowsay.json";
    const LESS_MANIFEST: &str = "./tests/sample_data/less.json";
    const FFMPEG_MANIFEST: &str = "./tests/sample_data/ffmpeg.json";

    let config = AlephConfig::new();
    // TODO Make a function for this repeated bs
    if !DEBUG_SKIP_COWSAY {
        let manifest = read_to_string(COWSAY_MANIFEST).expect("Failed to read file");
        let manifest: Manifest = Manifest::parse(&manifest).expect("Failed to parse data");
        if let Err(error) = manifest_installer(&config, &manifest, "cowsay") {
            eprintln!("{error}");
        }
    }

    if !DEBUG_SKIP_LESS {
        let manifest = read_to_string(LESS_MANIFEST).expect("Failed to read file");
        let manifest = Manifest::parse(&manifest).expect("Failed to parse data");
        if let Err(error) = manifest_installer(&config, &manifest, "less") {
            eprintln!("{error}");
        }
    }

    if !DEBUG_SKIP_FFMPEG {
        let manifest = read_to_string(FFMPEG_MANIFEST).expect("Failed to read file");
        let manifest = Manifest::parse(&manifest).expect("Failed to parse data");
        if let Err(error) = manifest_installer(&config, &manifest, "ffmpeg") {
            eprintln!("{error}");
        }
    }
}

#[test]
#[ignore]
fn msi_installation_test() {
    use std::fs::read_to_string;

    const DEBUG_SKIP_FIO: bool = false;
    const FIO_MANIFEST: &str = "./tests/sample_data/fio.json";

    let config = AlephConfig::new();

    if !DEBUG_SKIP_FIO {
        let manifest = read_to_string(FIO_MANIFEST).expect("Failed to read file");
        let manifest = Manifest::parse(&manifest).expect("Failed to parse data");
        if let Err(error) = manifest_installer(&config, &manifest, "fio") {
            eprintln!("{error}");
        }
    }
}
