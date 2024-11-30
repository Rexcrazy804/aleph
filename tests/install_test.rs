use aleph::{manifest::Manifest, scoopd::manifest_install::manifest_installer};

#[test]
#[ignore]
fn installation() {
    use std::fs::read_to_string;

    const DEBUG_SKIP_COWSAY: bool = true;
    const DEBUG_SKIP_LESS: bool = true;
    const DEBUG_SKIP_FFMPEG: bool = false;

    const COWSAY_MANIFEST: &'static str = "./tests/sample_data/cowsay.json";
    const LESS_MANIFEST: &'static str = "./tests/sample_data/less.json";
    const FFMPEG_MANIFEST: &'static str = "./tests/sample_data/ffmpeg.json";

    if !DEBUG_SKIP_COWSAY {
        let manifest = read_to_string(COWSAY_MANIFEST).expect("Failed to read file");
        let manifest: Manifest = serde_json::from_str(&manifest).expect("Failed to parse data");
        if let Err(error) = manifest_installer(&manifest) {
            eprintln!("{error}")
        }
    }

    if !DEBUG_SKIP_LESS {
        let manifest = read_to_string(LESS_MANIFEST).expect("Failed to read file");
        let manifest: Manifest = serde_json::from_str(&manifest).expect("Failed to parse data");
        if let Err(error) = manifest_installer(&manifest) {
            eprintln!("{error}")
        }
    }

    if !DEBUG_SKIP_FFMPEG {
        let manifest = read_to_string(FFMPEG_MANIFEST).expect("Failed to read file");
        let manifest: Manifest = serde_json::from_str(&manifest).expect("Failed to parse data");
        if let Err(error) = manifest_installer(&manifest) {
            eprintln!("{error}")
        }
    }
}
