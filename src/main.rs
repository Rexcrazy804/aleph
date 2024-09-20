mod manifest;
mod powershell;
mod zipper;

use powershell::{installer, utilities};

fn main() {
    println!("Running on {}", std::env::consts::OS);

    let home_dir = utilities::get_home_directory()
        .expect("Failed to get home directory, perhaps powershell is not installed?")
        // we can't do anything if powershell itself is not present. maybe if I can get something
        // like reqwuest to work correctly maybe then we'll have a shot?
        .trim()
        .to_owned();

    let download_dir = home_dir.clone() + "/Downloads/";
    let extract_dir = home_dir.clone() + "/Documents/aleph/";

    let mut filepath = String::new();
    match utilities::download_url(
        // I guess the next step is to get this link from the scoop manifest
        "https://github.com/jftuga/less-Windows/releases/download/less-v661/less-x64.zip",
        &download_dir,
    ) {
        Ok(name) => filepath = name,
        Err(error) => println!("Download Failed with: {error}"),
    };

    let unziped_dir = zipper::unzip(&filepath, &extract_dir).expect("Failed to extract");

    installer::append_to_path(&home_dir, &vec![extract_dir + &unziped_dir + "/"])
        .unwrap_or_else(|e| panic!("{e}"));
}
