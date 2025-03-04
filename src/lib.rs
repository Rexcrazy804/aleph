pub mod cli;
pub mod errors;
pub mod manifest;
pub mod powershell;
pub mod scoopd;
pub mod zipper;

use std::{io, path::PathBuf};

// maybe branch these out into CONFIG/
pub struct AlephConfig {
    paths: AlephPaths,
}

impl AlephConfig {
    #[must_use]
    pub fn new() -> Self {
        let paths = AlephPaths::new();
        if let Err(e) = paths.initialize_root_dir() {
            eprintln!("WARN: Aleph Directory initialization failure: {e}");
        };
        Self { paths }
    }

    // required for tests
    #[must_use]
    pub fn get_buckets_path(&self) -> &PathBuf {
        &self.paths.buckets
    }
    #[must_use]
    pub fn get_root_path(&self) -> &PathBuf {
        &self.paths.root
    }

    /// # Errors
    /// failure to create ``AlephPath`` Directories
    pub fn re_initialize(&self) -> io::Result<()> {
        self.paths.initialize_root_dir()
    }
}

impl Default for AlephConfig {
    fn default() -> Self {
        Self::new()
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
    /// # Errors
    /// failure to create ``AlephPath`` Directories
    fn initialize_root_dir(&self) -> io::Result<()> {
        // unsure whether this function should have a return type
        // since if anything fails here the programs stops execution
        // so if this function executes successfully it can be assumed
        use std::fs::create_dir;

        if let Ok(false) = &self.root.try_exists() {
            println!("Aleph root not found");
            create_dir(&self.root)?;
            println!("Created aleph root at {:?}", &self.root);
        }

        if let Ok(false) = self.buckets.try_exists() {
            create_dir(&self.buckets)?;
        }
        if let Ok(false) = self.download.try_exists() {
            create_dir(&self.download)?;
        }
        if let Ok(false) = self.packages.try_exists() {
            create_dir(&self.packages)?;
        }

        Ok(())
    }
}
