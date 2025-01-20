{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  } @ inputs: let
    system = "x86_64-linux";
    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs {inherit system overlays;};

    rustToolchain = pkgs.pkgsBuildHost.rust-bin.nightly.latest.default.override {
      extensions = ["rust-src"];
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [rustToolchain alejandra upx];

      RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/src";
      RUSTFLAGS = "-Zlocation-detail=none -C link-arg=-nostartfiles";
    };
  };
}
