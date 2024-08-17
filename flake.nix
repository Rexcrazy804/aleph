{
  description = "A flake for cross compiling and running rust windows target";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {flake-parts, ...} @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.treefmt.flakeModule
      ];

      flake = {
        # original stuff? idk what this does just yet
      };

      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem = {
        pkgs,
        system,
        ...
      }: let
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src"];
          targets = [buildTarget];
        };

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

        depsBuildBuild = with pkgs; [
          pkgsCross.mingwW64.stdenv.cc
          # pkgsCross.mingwW64.windows.pthreads
        ];

        buildTarget = "x86_64-pc-windows-gnu";

        aleph = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
          doCheck = false;

          inherit depsBuildBuild;
          CARGO_BUILD_TARGET = buildTarget;

          # fixes issues related to libring
          TARGET_CC = "${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}cc";

          #fixes issues related to openssl
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";
        };

        aleph_wine = pkgs.writeScriptBin "run" ''
          #!${pkgs.stdenv.shell}
          ${pkgs.wineWowPackages.stable}/bin/wine ${aleph}/bin/aleph.exe
        '';
      in {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [inputs.rust-overlay.overlays.default];
        };

        treefmt = {
          projectRootFile = "./.git/config";
          programs = {
            alejandra.enable = true;
            rustfmt = {
              enable = true;
              package = pkgs.rust-bin.nightly.latest.rustfmt;
            };
          };
        };

        packages.default = aleph_wine;

        devShells.default = craneLib.devShell {
          inputsFrom = [aleph];

          inherit depsBuildBuild;

          CARGO_BUILD_TARGET = buildTarget;
          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER = "${pkgs.wineWowPackages.stable}/bin/wine";
          WINEDEBUG = "-all";

          # WHY DOES CRANE NOT ADD THIS IN THE DEV SHELL AAAAAAAAAAAAAA
          TARGET_CC = "${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}cc";
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";

          RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
            pkgs.pkgsCross.mingwW64.windows.pthreads
          ]);
        };
      };
    };
}
