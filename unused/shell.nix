# Example flake for `nix shell`.
# See docs/cross_compilation.md for details.
(import <nixpkgs> {
  # crossSystem = "armv7a-unknown-linux-androideabi";
  # crossSystem = "armv7a-android-prebuilt";
  crossSystem = (import <nixpkgs/lib>).systems.examples.armv7a-android-prebuilt;
  overlays = [ (import ../..) ];
}).callPackage (
{ mkShell, stdenv, rust-bin, pkg-config, qemu, tinyalsa }:
mkShell {
  nativeBuildInputs = [
    rust-bin.stable.latest.minimal
    pkg-config
  ];

  depsBuildBuild = [ qemu ];

  buildInputs = [ tinyalsa ];

  CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER="${stdenv.cc.targetPrefix}cc";
  CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUNNER = "qemu-armv7a";
  # NIX_CFLAGS_COMPILE = "-Werror=implicit-function-declaration -ferror-limit=1000";
  # NIX_CFLAGS_COMPILE = "-W -Werror=implicit-function-declaration -ferror-limit=1000";
  # NIX_CFLAGS_COMPILE = "-W";
  # OPENSSL_DIR = "${openssl.dev}";
  # OPENSSL_LIB_DIR = "${openssl.out}/lib";
  # PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig";
}) {}
