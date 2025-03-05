# Aleph 
a **Work in Progress** declarative package manager for win(e)dows strongly based on
[Scoop](https://github.com/ScoopInstaller/Scoop/) and inspired by
[nix](https://nixos.org/)

## PREFACE
Currently there is still A LOOOOT of work to be done but I felt its about time
I make this public, I don't really expect it to garner much interest since it
was just a random project idea I've wanted to work on out of spite and eventually
I've decided to work on it as my college project. Everything up to version 0.0.50 are
worked on by me and sanoy07 and that will be the only part that will be included and
presented as part of our project.

I've tested a lot of packages that (unfortunately) wasn't logged earlier so the current
`testedpkgs.csv` is hardly reflective of the number of packages that do work I highly encourage
populating that list with your own testing.

As a rule of thumb anything that does not require the unsupported attributes will work
and then again *some* packages will still install and run even when some of its attributes
do not work.

Also my Rust Code is pretty lacking so feel welcome to critique (also almost
non existent error handling it is my next priority)

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
A windows pc or a wine prefix (LINUX) with **powershell** installed

> NOTE you may need to install powershell nonetheless on windows if you run into
powershell not found error (I should fix it soon)

> NOTE I have worked on this thing entirely on nix so I strongly recommend using
the nix devShell when working on this project (also its really fucked up to set
this cross compiling shinanigan up otherwise)

## Configuration spec (HEAVILY SUBJECT TO CHANGE)
The configuration is passed to aleph using rebuild subcommand followed by the path
to the configuration
```lua
-- UNSTABLE SPECIFICATION FOR ALEPH Configuration
-- works without the return but my lsp will complain
return {
  description = "Minimal Config";

  buckets = {
    main = "https://github.com/ScoopInstaller/Main/archive/68898739d017dfed8fcf7c096c48fe5d829c7bbd.zip",
    extras = "https://github.com/ScoopInstaller/Extras/archive/8c3c91d622775f8bafc9ee6051c7ccc016fe7ec7.zip",
  };

  -- list of package names,
  -- (unimplemented) can specify the bucket by using
  -- "<bucketName>.packageName"
  packages = {
    "cowsay",
    "less",
    "eza",
    "notepadplusplus",
    "git",
  };
}
```

## Scoop Manifest Compatibility
#### No implementation required
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
- [x] shortcuts

#### partial support
- [x] bin
- [x] architecture

#### Pending implementation
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
