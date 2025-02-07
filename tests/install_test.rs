use aleph::{
    scoopd::{manifest_install::dependency_install, manifest_uninstall::manifest_uninstaller},
    AlephConfig,
};

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
    // ignoring this test cause it fucks something up in a way that ps can't download stuff
    // TODO find a similiar example [focus on extract_to / extract_dir attr]
    //assert_eq!(Ok(()), dependency_install(&config, "unxutils"));
}

#[test]
#[ignore]
fn general_uninstallation_test() {
    let config = AlephConfig::new();
    assert_eq!(Ok(()), manifest_uninstaller(&config, "cowsay"));
    assert_eq!(Ok(()), manifest_uninstaller(&config, "less"));
    assert_eq!(Ok(()), manifest_uninstaller(&config, "eza"));
    assert_eq!(Ok(()), manifest_uninstaller(&config, "git"));
    assert_eq!(Ok(()), manifest_uninstaller(&config, "bat"));
}
