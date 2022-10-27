{
  description = "An untitled programming language project.";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

      in rec {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            rust-analyzer
          ];
        };
      }
    );
}
