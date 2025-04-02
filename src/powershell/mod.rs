pub mod profile_util;
pub mod utilities;

use std::sync::LazyLock;

static PWSH_EXE: LazyLock<String> = LazyLock::new(|| -> String {
    match std::env::var("ALEPH_LINUX") {
        Ok(_) => String::from("pwsh"),
        Err(_) => String::from("powershell"),
    }
});
