# Example flake for `nix develop`.
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, rust-overlay, nixpkgs }: {
    devShells.x86_64-linux.default = let
      pkgs = nixpkgs.legacyPackages.x86_64-linux.pkgsCross.armv7a-android-prebuilt;
      rust-bin = rust-overlay.lib.mkRustBin { } pkgs.buildPackages;
    in
      pkgs.mkShell {
        nativeBuildInputs = [
          rust-bin.stable.latest.minimal
          pkgs.buildPackages.pkg-config
        ];

        depsBuildBuild = [ pkgs.pkgsBuildBuild.qemu ];
        buildInputs = [ pkgs.tinyalsa ];

        env = {

          CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER="${pkgs.stdenv.cc.targetPrefix}cc";
          CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUNNER = "qemu-armv7a";
          # OPENSSL_DIR = "${pkgs.openssl.dev}";
          # OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      };
  };
}
