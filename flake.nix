{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    naersk,
  } @ inputs: let
    system = "x86_64-linux";
    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs {inherit system overlays;};

    rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
      toolchain.default.override {
        extensions = ["rust-src"];
      });

    naersk-lib = pkgs.callPackage naersk {
      cargo = rustToolchain;
      rustc = rustToolchain;
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [rustToolchain alejandra nasm gdb];

      RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/src";
    };

    packages.${system}.qrrust = let
      shrinkRustlibHook =
        pkgs.makeSetupHook {
          name = "shrink-rustlib-hook.sh";
          propagatedBuildInputs = with pkgs; [nasm];
          substitutions = {
            linkScript = ./build/script.ld;
            headerAsm = ./build/header.s;
            unzipScript = ./build/unzip-template.sh;
          };
        }
        ./build/shrink-rustlib-hook.sh;
    in
      naersk-lib.buildPackage {
        pname = "qrrust";
        src = ./.;

        overrideMain = finalAttrs: previousAttrs: {
          nativeBuildInputs = previousAttrs.nativeBuildInputs ++ [shrinkRustlibHook];
        };

        copyTarget = true;
        compressTarget = false;
        copyBins = false;

        cargoBuildOptions = default: default ++ ["--lib"];
        additionalCargoLock = "${rustToolchain.availableComponents.rust-src}/lib/rustlib/src/rust/library/Cargo.lock";
      };

    /*
      rustPlatform.buildRustPackage rec {
      pname = "qrrust";
      version = "0.1.0";

      src = ./.;
      cargoLock = {
        lockFile = ./Cargo.lock;
      };

      additionalCargoLock = pkgs.lib.traceVal "${rustToolchain.availableComponents.rust-src}/lib/rustlib/src/rust/library/Cargo.lock";

      # Disable nix patches and things, they only add bloat without any linked libraries
      phases = ["unpackPhase" "buildPhase" "checkPhase" "installPhase"];
      nativeBuildInputs = [shrinkRustlibHook];

      cargoBuildFlags = "--lib";
      doCheck = false;
    };
    */

    defaultPackage.${system} = self.packages.${system}.qrrust;

    overlays.default = final: prev: {
      qrrust = self.packages.${system}.qrrust;
    };
  };
}
