use aleph::{scoopd::manifest_install::dependency_install, AlephConfig};

#[test]
#[ignore]
fn general_installation_test() {
    let config = AlephConfig::new();

    // in case we don't have the main bucket
    assert_eq!(Ok(()), aleph::cli::subcommands::fetch_repo(&config, None));
    assert_eq!(Ok(()), dependency_install(&config, "cowsay"));
    assert_eq!(Ok(()), dependency_install(&config, "less"));
    assert_eq!(Ok(()), dependency_install(&config, "eza"));
    assert_eq!(Ok(()), dependency_install(&config, "git"));
    assert_eq!(Ok(()), dependency_install(&config, "bat"));
    assert_eq!(Ok(()), dependency_install(&config, "unxutils"));
}
