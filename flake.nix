{
  description = "A flake for cross compiling and running rust windows target";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    git-hooks-nix = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    crane.url = "github:ipetkov/crane";
  };

  outputs = {flake-parts, ...} @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.git-hooks-nix.flakeModule
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
        config,
        ...
      }: let
        buildTarget = "x86_64-pc-windows-gnu";
        rustToolChain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src"];
          targets = [buildTarget];
        };

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolChain;

        depsBuildBuild = with pkgs; [
          pkgsCross.mingwW64.stdenv.cc
          # pkgsCross.mingwW64.windows.pthreads
        ];

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

        pre-commit = {
          check.enable = true;
          settings.hooks = let
            rust = pkgs.rust-bin.stable.latest;
          in {
            alejandra.enable = true;
            rustfmt = {
              enable = true;
              packageOverrides = {
                inherit (rust) rustfmt cargo;
              };
            };
            clippy = {
              enable = false;
              packageOverrides = {
                inherit (rust) cargo clippy;
              };
            };
          };
        };

        packages.default = aleph_wine;

        devShells.default = craneLib.devShell {
          shellHook = ''
            ${config.pre-commit.installationScript}
          '';
          inputsFrom = [aleph];

          nativeBuildInputs = [
            config.pre-commit.settings.enabledPackages
          ];

          inherit depsBuildBuild;

          CARGO_BUILD_TARGET = buildTarget;
          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER = "${pkgs.wineWowPackages.stable}/bin/wine";
          WINEDEBUG = "-all";

          # WHY DOES CRANE NOT ADD THIS IN THE DEV SHELL AAAAAAAAAAAAAA
          TARGET_CC = "${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}cc";
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";

          RUSTFLAGS = builtins.map (a: ''-L ${a}/lib'') [
            pkgs.pkgsCross.mingwW64.windows.pthreads
          ];
        };
      };
    };
}
