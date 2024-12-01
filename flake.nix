{
  description = "Nathan's advent of code 2024";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    utils,
    rust-overlay,
    crane,
    ...
  }:
    utils.lib.eachSystem [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ]
    (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      common = import ./nix/common.nix {
        inherit pkgs;
        craneLib = crane.mkLib pkgs;
      };
    in {
      packages = {
        default = common.package;
        coverage = common.coverage;
      };

      devShells.default = import ./nix/shell.nix {
        inherit pkgs;
        inherit (common) toolchain;
      };

      checks = common.checks;
      formatter = pkgs.alejandra;
    });
}
