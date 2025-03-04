use aleph::{cli::subcommands, AlephConfig};

#[ignore]
#[test]
fn bucket_install() {
    let config = AlephConfig::new();
    std::fs::remove_dir_all(config.get_buckets_path()).expect("Failled to remove buckets path");

    config
        .re_initialize()
        .expect("Failed to initialize AlephPaths");
    assert_eq!(Ok(()), subcommands::fetch_repo(&config, None));
}
