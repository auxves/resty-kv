{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      flake = { };
      systems = [
        "aarch64-linux"
        "x86_64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem = { self', pkgs, ... }: {
        packages.default = self'.packages.resty-kv;

        packages.resty-kv =
          if pkgs.stdenv.isLinux
          then pkgs.pkgsStatic.callPackage ./pkg.nix { }
          else pkgs.callPackage ./pkg.nix { };

        devShells.default = with pkgs; mkShell {
          nativeBuildInputs = [ rustc cargo rust-analyzer clippy rustfmt ];
        };

        devShells.ci = with pkgs; mkShellNoCC {
          nativeBuildInputs = [ rustc cargo ];
        };
      };
    };
}
