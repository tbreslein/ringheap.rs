{
  description = "heap with an underlying ringbuffer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ self, ... }:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
      perSystem = { system, pkgs, ... }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ (import inputs.rust-overlay) ];
        };
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            openssl
            pkg-config
            # (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" "rustfmt" ];
            })

            # dev tools
            bacon
            cargo-nextest
            cargo-criterion
          ];
        };
      };
    };
}

