{

  description = "An untitled programming language project.";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, pre-commit-hooks, naersk }:
    let
      SYSTEMS = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
    in
    flake-utils.lib.eachSystem SYSTEMS (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };

        mkUplp = args:
          let
            defaultArgs = {
              src = ./.;

              cargoTestCommands = prev: prev ++ [
                ''cargo $cargo_options clippy -- -Dwarnings''
              ];

              override = prev: prev // { nativeBuildInputs = prev.nativeBuildInputs ++ [ pkgs.clippy ]; };
            };
          in
          naersk'.buildPackage (defaultArgs // args);

        preCommitHook = pre-commit-hooks.lib.${system}.run {
          src = self;
          hooks = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
        };
      in
      rec {
        packages = rec {
          uplp = mkUplp { };

          default = uplp;
        };

        apps.default = {
          type = "app";
          program = "${packages.uplp}/bin/uplp";
        };

        devShells.default = pkgs.mkShell {
          shellHook = ''
            ${preCommitHook.shellHook}
          '';

          nativeBuildInputs = with pkgs; [
            rustc
            rustfmt
            cargo
            clippy
            rust-analyzer
            nixpkgs-fmt
          ];
        };

        checks = {
          uplp = mkUplp { doCheck = true; };
          pre-commit = preCommitHook;
        };
      }
    );
}
