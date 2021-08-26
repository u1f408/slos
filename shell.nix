let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { 
    overlays = [
      (import sources.nixpkgs-mozilla)
      (import ./nix/rust-overlay.nix)
    ];
  };

  inherit (pkgs) lib;

in pkgs.mkShell {
  buildInputs = with pkgs; [
    rust-nightly-bin
    cargo-watch
  ];
}
