use aleph::{scoopd::manifest_install::dependency_install, AlephConfig};

#[test]
fn simple_shortcut_test() {
    let config = AlephConfig::new();
    assert_eq!(Ok(()), dependency_install(&config, "autoit"));
}
