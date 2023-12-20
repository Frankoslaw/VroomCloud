{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        overlays = [
          (self: super: {
              nodePackages.pnpm = super.nodePackages.pnpm.override {
                  nodejs = pkgs.nodejs-21_x;
              };
          })
        ];
        libPath = with pkgs; lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            pre-commit
            tokei
            pkg-config
            nodejs
            nodejs.pnpm
          ];
          LD_LIBRARY_PATH = libPath;
        };
      }
    );
}
