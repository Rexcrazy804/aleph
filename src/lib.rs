pub mod cli;
pub mod manifest;
pub mod powershell;
pub mod scoopd;
pub mod zipper;

use std::path::PathBuf;

// maybe branch these out into CONFIG/
pub struct AlephConfig {
    pub paths: AlephPaths,
    // more to come later?
}

impl AlephConfig {
    pub fn new() -> Self {
        let paths = AlephPaths::new();
        paths.initialize_root_dir();
        Self { paths }
    }
}

struct AlephPaths {
    home: PathBuf,
    root: PathBuf,
    buckets: PathBuf,
    packages: PathBuf,
    download: PathBuf,
}

impl AlephPaths {
    fn new() -> Self {
        let home = crate::powershell::utilities::get_home_directory();
        let root = home.join("Aleph");
        let download = root.join("Downloads");
        let packages = root.join("Packages");
        let buckets = root.join("Buckets");

        Self {
            home,
            root,
            buckets,
            packages,
            download,
        }
    }

    /// this function creates the aleph root directory and popluates it with the required directory
    /// skips directory creation if it exists
    // unsure whether this function should have a return type
    // since if anything fails here the programs stops execution
    // so if this function executes successfully it can be assumed
    fn initialize_root_dir(&self) {
        use std::fs::create_dir;

        if let Ok(false) = &self.root.try_exists() {
            println!("Aleph root not found");
            create_dir(&self.root).expect("Failed to create Aleph Root directory");
            println!("Created aleph root at {:?}", &self.root);
        }

        if let Ok(false) = self.buckets.try_exists() {
            create_dir(&self.buckets).expect("Failed to create Aleph/Buckets");
        }
        if let Ok(false) = self.download.try_exists() {
            create_dir(&self.download).expect("Failed to create Aleph/Downloads");
        }
        if let Ok(false) = self.packages.try_exists() {
            create_dir(&self.packages).expect("Failed to create Aleph/Packages");
        }
    }
}
