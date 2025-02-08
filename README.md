# Aleph - a (WIP) declarative package manager for windows (that works on linux under wine)

## Highlights
- declarative package manager
- works on linux for all your wine prefixes
- compatible with [Scoop](https://github.com/ScoopInstaller/Scoop/)
[Manifest](https://github.com/ScoopInstaller/Scoop/wiki/App-Manifests)

## A brief introduction
This is an attempt at mimicking the ability of the [nix](https://nixos.org/)
package manager for linux.

So why the works under linux part? Well, this whole idea started because I
wanted to install scoop in wine for a reason that I no longer have any
recollection of and was frustrated enough by it to try and re-implement scoop
on rust WHILE keeping linux support in mind (I mean the whole thing has been
developed bottom up on linux) and I am yet to perform sufficient testing on
windows.

## Requirements
A windows pc or a wine prefix (LINUX) with powershell installed

## Scoop Manifest Compatibility
#### Mandatory
- [x] version
- [x] description
- [x] homepage
- [x] license

#### currently supported
- [x] ## (comments)
- [x] url
- [x] depends
- [x] env_add_path
- [x] env_set
- [x] extract_to
- [x] notes
- [x] suggest

#### partial support
- [x] bin
- [x] architecture

#### Pending implementation
- [ ] shortcuts
- [ ] installer
- [ ] post_install
- [ ] pre_install
- [ ] pre_uninstall
- [ ] post_uninstall
- [ ] uninstaller

#### Low Priority
- [ ] innosetup
- [ ] hash
- [ ] psmodule

#### IGNORED
- [ ] autoupdate
- [ ] checkver
