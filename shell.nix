with import <nixos> { };
stdenv.mkDerivation rec {
  name = "rs";
  buildInputs = [
    (rust-bin.nightly.latest.rust.override {
      extensions = [ "rust-src" ];
    })
    (rust-bin.stable.latest.rust.override {
      extensions = [ "rust-src" ];
    })
    cargo-edit
    cargo-audit
    cargo-license
  ];
  RUST_BACKTRACE = 1;
  src = null;
}
