/*
* installer: Instructions for running a non-MSI installer.
    file: The installer executable file. For installer this defaults to the last URL downloaded.
    Must be specified for uninstaller.

    script: A one-line string, or array of strings, of commands to be executed as an
    installer/uninstaller instead of file.

    args: An array of arguments to pass to the installer. Optional.

    keep: "true" if the installer should be kept after running (for future uninstallation, as an
    example). If omitted or set to any other value, the installer will be deleted after running. See
    extras/oraclejdk for an example. This option will be ignored when used in an uninstaller directive.

    NOTE TODO: think of how to make these vars available .w. / how they can be used
    Variables available to script and args: $fname (the file last downloaded), $manifest (the
    deserialized manifest reference), $architecture (64bit or 32bit), $dir (install directory) Called
    during both scoop install and scoop update.
*/

use super::OneOrMany;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Installer {
    file: Option<String>,
    args: Option<OneOrMany<String>>,
    script: Option<Script>,
    keep: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Script(OneOrMany<String>);
